use bollard::{
    container::{
        Config, CreateContainerOptions, InspectContainerOptions, ListContainersOptions,
        LogsOptions, RemoveContainerOptions, StartContainerOptions,
    },
    errors::Error,
    exec::{CreateExecOptions, StartExecResults},
    image::{CreateImageOptions, ListImagesOptions},
    secret::{ContainerStateStatusEnum, ContainerSummary, PortBinding},
    Docker,
};
use futures::{Stream, StreamExt};
use std::{collections::HashMap, pin::Pin};
use tauri::Emitter;

use crate::{app_handle, gpt_sovits_model_dir, user_files_dir};

const YT_DLP_IMAGE: &str = "jauderho/yt-dlp:latest";
const AENEAS_IMAGE: &str = "shreshthtuli/aeneas:latest";
const FFMPEG_IMAGE: &str = "ruobinqaq/ffmpeg:latest";
// const GPT_SOVITS_IMAGE: &str = "breakstring/gpt-sovits:latest";
const GPT_SOVITS_IMAGE: &str = "ruobinqaq/gpt_sovits:latest";

/// 创建并启动一个 yt-dlp 容器，执行指定的命令
pub async fn create_and_run_yt_dlp_container(cmd: Vec<&str>) -> Result<String, String> {
    let docker = get_docker().await?;

    if !is_image_exists(YT_DLP_IMAGE).await? {
        pull_image(YT_DLP_IMAGE, "gpt_sovits_api_log").await?;
    }

    let mount_path = user_files_dir().to_string_lossy().to_string() + ":/workspace";

    let config = Config::<&str> {
        image: Some(YT_DLP_IMAGE),
        cmd: Some(cmd),
        env: Some(vec![
            "HTTP_PROXY=http://host.docker.internal:7890",
            "HTTPS_PROXY=http://host.docker.internal:7890",
        ]),
        host_config: Some(bollard::service::HostConfig {
            binds: Some(vec![mount_path]),
            auto_remove: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };

    let options = None::<CreateContainerOptions<&str>>;
    let container_id = docker
        .create_container(options, config)
        .await
        .map_err(|e| e.to_string())?
        .id; // 获取容器 ID

    docker
        .start_container(&container_id, None::<StartContainerOptions<&str>>)
        .await
        .map_err(|e| e.to_string())?;

    let mut log_stream = docker.logs(
        &container_id,
        Some(LogsOptions::<String> {
            stdout: true,
            stderr: true,
            follow: true,
            ..Default::default()
        }),
    );

    let mut collected_logs = String::new();
    while let Some(log_result) = log_stream.next().await {
        match log_result {
            Ok(log_output) => {
                collected_logs.push_str(&format!("{}", log_output));
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
    Ok(collected_logs)
}

/// 创建并启动一个 aeneas 容器，执行指定的命令
pub async fn create_and_run_aeneas_container(
    audio_path: String,
    text_path: String,
    output_path: String,
) -> Result<(), String> {
    let docker = get_docker().await?;

    if !is_image_exists(AENEAS_IMAGE).await? {
        pull_image(AENEAS_IMAGE, "aeneas_log").await?;
    }

    if let Ok(_) = get_container_by_name("aeneas").await {
        remove_container("aeneas").await?;
    }

    let mount_path = user_files_dir().to_string_lossy().to_string() + ":/workspace";

    let cmd = format!(
        "cd ~/Aeneas/aeneas && python -m aeneas.tools.execute_task {} {} 'task_language=zho|os_task_file_format=srt|is_text_type=plain' {}",
        audio_path,
        text_path,
        output_path
    );

    let config = Config::<&str> {
        image: Some(AENEAS_IMAGE),
        cmd: Some(vec!["/bin/bash", "-c", cmd.as_str()]),
        env: Some(vec!["PYTHONIOENCODING=UTF-8"]),
        host_config: Some(bollard::service::HostConfig {
            binds: Some(vec![mount_path]),
            ..Default::default()
        }),
        ..Default::default()
    };

    let options = Some(CreateContainerOptions::<&str> {
        name: "aeneas",
        platform: None,
    });

    docker
        .create_container(options, config)
        .await
        .map_err(|e| e.to_string())?;
    docker
        .start_container("aeneas", None::<StartContainerOptions<&str>>)
        .await
        .map_err(|e| e.to_string())?;

    emit_container_logs("aeneas", "gpt_sovits_api_log").await?;
    check_container_logs_for_string("aeneas", "Created file").await?;

    Ok(())
}

/// 创建并启动一个 ffmpeg 容器，执行指定的命令
pub async fn create_and_run_ffmpeg_container(cmd: Vec<&str>) -> Result<(), String> {
    let docker = get_docker().await?;

    if !is_image_exists(FFMPEG_IMAGE).await? {
        pull_image(FFMPEG_IMAGE, "gpt_sovits_api_log").await?;
    }

    if let Ok(_) = get_container_by_name("ffmpeg").await {
        remove_container("ffmpeg").await?;
    }

    let options = Some(CreateContainerOptions::<&str> {
        name: "ffmpeg",
        platform: None,
    });

    let mount_path = user_files_dir().to_string_lossy().to_string() + ":/workspace";

    let config = Config::<&str> {
        image: Some(FFMPEG_IMAGE),
        cmd: Some(cmd),
        host_config: Some(bollard::service::HostConfig {
            binds: Some(vec![mount_path]),
            ..Default::default()
        }),
        ..Default::default()
    };

    docker
        .create_container(options, config)
        .await
        .map_err(|e| e.to_string())?;

    docker
        .start_container("ffmpeg", None::<StartContainerOptions<&str>>)
        .await
        .map_err(|e| e.to_string())?;

    emit_container_logs("ffmpeg", "gpt_sovits_api_log").await?;

    Ok(())
}

// 开启gpt-sovits
pub async fn start_gpt_sovits_container() -> Result<(), String> {
    let docker = get_docker().await?;

    match docker
        .start_container("gpt-sovits", None::<StartContainerOptions<&str>>)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => match e {
            Error::DockerResponseServerError { status_code, .. } => {
                if status_code == 404 {
                    create_gpt_sovits().await?;
                    Box::pin(start_gpt_sovits_container()).await
                } else {
                    Err(e.to_string())
                }
            }
            _ => Err(e.to_string()),
        },
    }
}

// 开启gpt-sovits API服务
pub async fn start_gpt_sovits_api() -> Result<(), String> {
    if is_container_running("gpt-sovits").await? {
        return Ok(());
    } else {
        start_gpt_sovits_container().await?;
        let cmd = vec![
            "python",
            "api_v2.py",
            "-a",
            "0.0.0.0",
            "-p",
            "9880",
            "-c",
            "GPT_SoVITS/configs/tts_infer.yaml",
        ];

        let mut stream = run_command_in_container("gpt-sovits", cmd).await?;

        while let Some(result) = stream.next().await {
            match result {
                Ok(output) => {
                    if output.contains("Press CTRL+C to quit") {
                        return Ok(());
                    }
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
    }
    Ok(())
}

// 创建gpt-sovits容器
pub async fn create_gpt_sovits() -> Result<(), String> {
    let docker = get_docker().await?;

    if !is_image_exists(GPT_SOVITS_IMAGE).await? {
        pull_image(GPT_SOVITS_IMAGE, "gpt_sovits_api_log").await?;
    }

    let options = Some(CreateContainerOptions::<&str> {
        name: "gpt-sovits",
        platform: None,
    });

    let mut port_bindings: HashMap<String, Option<Vec<PortBinding>>> = HashMap::new();
    port_bindings.insert(
        "9880/tcp".to_string(),
        Some(vec![PortBinding {
            host_ip: Some("0.0.0.0".to_string()),
            host_port: Some("9880".to_string()),
        }]),
    );
    port_bindings.insert(
        "9871/tcp".to_string(),
        Some(vec![PortBinding {
            host_ip: Some("0.0.0.0".to_string()),
            host_port: Some("9871".to_string()),
        }]),
    );
    port_bindings.insert(
        "9872/tcp".to_string(),
        Some(vec![PortBinding {
            host_ip: Some("0.0.0.0".to_string()),
            host_port: Some("9872".to_string()),
        }]),
    );
    port_bindings.insert(
        "9873/tcp".to_string(),
        Some(vec![PortBinding {
            host_ip: Some("0.0.0.0".to_string()),
            host_port: Some("9873".to_string()),
        }]),
    );
    port_bindings.insert(
        "9874/tcp".to_string(),
        Some(vec![PortBinding {
            host_ip: Some("0.0.0.0".to_string()),
            host_port: Some("9874".to_string()),
        }]),
    );

    let mount_path =
        gpt_sovits_model_dir().to_string_lossy().to_string() + ":/workspace/gpt_sovits_model";

    let config = Config::<&str> {
        image: Some(GPT_SOVITS_IMAGE),
        host_config: Some(bollard::service::HostConfig {
            port_bindings: Some(port_bindings),
            binds: Some(vec![mount_path]),
            shm_size: Some(16 * 1024 * 1024 * 1024), // 16G
            device_requests: Some(vec![bollard::models::DeviceRequest {
                driver: Some("nvidia".to_string()),
                count: Some(-1),
                capabilities: Some(vec![vec!["gpu".to_string()]]),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    };
    docker
        .create_container(options, config)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// 运行命令在容器中
async fn run_command_in_container(
    container_name: &str,
    cmd: Vec<&str>,
) -> Result<Pin<Box<dyn Stream<Item = Result<String, Error>> + Send>>, String> {
    let docker = get_docker().await?;

    let exec_instance = docker
        .create_exec(
            container_name,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(cmd),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    let start_result = docker
        .start_exec(&exec_instance.id, None)
        .await
        .map_err(|e| e.to_string())?;

    match start_result {
        StartExecResults::Attached { output, .. } => {
            let stream = output.map(|log_output| match log_output {
                Ok(log_output) => Ok(format!("{}", log_output)),
                Err(e) => Err(e),
            });
            Ok(stream.boxed())
        }
        StartExecResults::Detached => Err("命令已执行无返回值".to_string()),
    }
}

// 判断容器是否运行中
pub async fn is_container_running(container_name: &str) -> Result<bool, String> {
    let container = match get_container_by_name(container_name).await {
        Ok(container) => container,
        Err(_) => return Ok(false),
    };

    let docker = get_docker().await?;

    let inspect_info = docker
        .inspect_container(
            &container.id.as_ref().unwrap(),
            None::<InspectContainerOptions>,
        )
        .await
        .map_err(|e| e.to_string())?;

    let state = inspect_info.state;

    match state {
        Some(state) if state.status == Some(ContainerStateStatusEnum::RUNNING) => Ok(true),
        _ => Ok(false),
    }
}

// 判断镜像是否存在
pub async fn is_image_exists(image_name: &str) -> Result<bool, String> {
    let docker = get_docker().await?;

    let mut filters = HashMap::new();
    filters.insert("reference".to_string(), vec![image_name.to_string()]);

    let options = ListImagesOptions::<String> {
        all: true,
        filters,
        digests: false,
    };

    let images = docker
        .list_images(Some(options))
        .await
        .map_err(|e| e.to_string())?;

    if !images.is_empty() {
        return Ok(true);
    }

    Ok(false)
}

// 拉取镜像
pub async fn pull_image(image_name: &str, emit_name: &str) -> Result<(), String> {
    let docker = get_docker().await?;

    let create_options = CreateImageOptions {
        from_image: image_name,
        ..Default::default()
    };

    let mut stream = docker.create_image(Some(create_options), None, None);

    while let Some(result) = stream.next().await {
        match result {
            Ok(info) => {
                if let Some(progress_detail) = info.progress_detail {
                    let progress = format!(
                        "{}/{}",
                        progress_detail.current.unwrap_or(-1),
                        progress_detail.total.unwrap_or(-1)
                    );
                    println!("{}", progress);
                    app_handle().emit(emit_name, &progress).unwrap();
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    Ok(())
}

// 获取docker
pub async fn get_docker() -> Result<Docker, String> {
    match Docker::connect_with_socket_defaults() {
        Ok(docker) => match docker.ping().await {
            Ok(_) => Ok(docker),
            Err(e) => Err(format!("ping docker失败: {}", e.to_string())),
        },
        Err(e) => Err(format!("链接docker失败: {}", e.to_string())),
    }
}

// 根据名称获取容器
#[allow(dead_code)]
pub async fn get_container_by_name(container_name: &str) -> Result<ContainerSummary, String> {
    let docker = get_docker().await?;

    let containers = docker
        .list_containers(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .map_err(|e| e.to_string())?;

    for container in containers {
        if let Some(names) = &container.names {
            if names
                .iter()
                .any(|name| name == format!("/{container_name}").as_str())
            {
                return Ok(container);
            }
        }
    }

    Err(format!("没有找到容器'{}'", container_name))
}

/// 删除指定名称的容器
pub async fn remove_container(container_name: &str) -> Result<(), String> {
    let docker = get_docker().await?;

    docker
        .remove_container(
            container_name,
            Some(RemoveContainerOptions {
                v: true,     // 删除容器时也删除其挂载的卷
                force: true, // 强制删除，即使容器正在运行
                link: false, // 默认不删除链接
            }),
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 监听指定名称的容器的日志并发送到前端
pub async fn emit_container_logs(container_name: &str, emit_name: &str) -> Result<(), String> {
    let docker = get_docker().await?;

    let logs_options = LogsOptions::<String> {
        stdout: true,
        stderr: true,
        tail: "all".to_string(),
        follow: true,
        ..Default::default()
    };

    let mut log_stream = docker.logs(container_name, Some(logs_options));

    while let Some(log_output) = log_stream.next().await {
        match log_output {
            Ok(log_output) => {
                app_handle()
                    .emit(emit_name, format!("{}", log_output))
                    .unwrap();
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    Ok(())
}

/// 检查指定名称的容器的日志中是否包含指定字符串
pub async fn check_container_logs_for_string(
    container_name: &str,
    search_string: &str,
) -> Result<(), String> {
    let docker = get_docker().await?;
    let mut logs = docker.logs(
        container_name,
        Some(LogsOptions::<String> {
            stdout: true,
            stderr: true,
            follow: true,
            ..Default::default()
        }),
    );

    let mut collected_logs = String::new();
    while let Some(log) = logs.next().await {
        match log {
            Ok(log) => {
                let log_message = format!("{}", log);
                collected_logs.push_str(&log_message);
                if log_message.contains(search_string) {
                    return Ok(());
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    Err(format!(
        "容器'{}'的日志中没有输出'{}'以下是日志：{}",
        container_name, search_string, collected_logs
    ))
}

/// 获取指定容器的完整日志
pub async fn get_container_logs(container_name: &str) -> Result<String, String> {
    let docker = get_docker().await?;

    let logs_options = LogsOptions::<String> {
        stdout: true,
        stderr: true,
        tail: "all".to_string(),
        follow: false,
        ..Default::default()
    };

    let mut log_stream = docker.logs(container_name, Some(logs_options));

    let mut collected_logs = String::new();

    while let Some(log_output) = log_stream.next().await {
        match log_output {
            Ok(log_output) => {
                collected_logs.push_str(&format!("{}", log_output));
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    Ok(collected_logs)
}
