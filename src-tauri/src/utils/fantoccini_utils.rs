use super::default_utils::{ensure_path_exists, is_process_running};
use crate::user_files_dir;
use fantoccini::key::Key;
use fantoccini::{Client, ClientBuilder, Locator};
use serde_json::{json, Map};
use std::os::windows::process::CommandExt;
use std::process::Command;

pub async fn get_client() -> Result<Client, String> {
    check_webdriver().await?;
    kill_edge_processes()?;
    let mut caps = Map::new();
    caps.insert(
        "ms:edgeOptions".to_string(),
        json!( {
            "args": [
                "--user-data-dir=C:/Users/17732/AppData/Local/Microsoft/Edge/User Data"
            ]
        }),
    );

    let c = ClientBuilder::native()
        .capabilities(caps)
        .connect("http://localhost:9516")
        .await
        .map_err(|e| e.to_string())?;
    Ok(c)
}

pub async fn check_webdriver() -> Result<(), String> {
    if is_process_running("msedgedriver.exe") {
        return Ok(());
    }

    let driver_dir_path = user_files_dir().join("webdriver");
    ensure_path_exists(&driver_dir_path).unwrap();
    let driver_path = driver_dir_path.join("msedgedriver.exe");
    if !driver_path.exists() {
        let zip_path = driver_dir_path.join("edgedriver_win64.zip");
        let url = "https://msedgedriver.azureedge.net/130.0.2849.52/edgedriver_win64.zip";
        let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
        let content = response.bytes().await.map_err(|e| e.to_string())?;
        std::fs::write(&zip_path, &content).map_err(|e| e.to_string())?;
        let output = Command::new("tar")
            .args(&[
                "-xvf",
                zip_path.to_str().unwrap(),
                "-C",
                driver_dir_path.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| format!("解压失败: {}", e))?;
        if !output.status.success() {
            return Err(format!(
                "解压失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    const CREATE_NEW_CONSOLE: u32 = 0x00000010;
    Command::new(driver_path)
        .arg("--port=9516")
        .creation_flags(CREATE_NEW_CONSOLE)
        .spawn()
        .map_err(|e| format!("启动msedgedriver失败: {}", e))?;
    Ok(())
}

pub fn kill_edge_processes() -> Result<(), String> {
    if is_process_running("msedge.exe") {
        let output = Command::new("taskkill")
            .args(&["/F", "/IM", "msedge.exe"])
            .output()
            .map_err(|e| format!("执行taskkill失败: {}", e))?;
        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "结束进程edge失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    } else {
        Ok(())
    }
}

pub async fn upload_bilibili(path: &str, tags: &Vec<String>, name: &str) -> Result<(), String> {
    let c = get_client().await?;

    c.goto("https://member.bilibili.com/platform/upload/video/frame")
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css(".upload-btn"))
        .await
        .map_err(|e| e.to_string())?;

    let input_element = c
        .wait()
        .for_element(Locator::Css("input[type='file']"))
        .await
        .map_err(|e| e.to_string())?;

    c.execute(
        "document.querySelector('input[type=\"file\"]').style.display = 'block';",
        vec![],
    )
    .await
    .map_err(|e| e.to_string())?;

    input_element
        .send_keys(path)
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css(".img"))
        .await
        .map_err(|e| e.to_string())?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css(".cropper-view-box img"))
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css(".bcc-button.bcc-button--primary.large"))
        .await
        .map_err(|e| e.to_string())?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    let name_input = c
        .wait()
        .for_element(Locator::Css(".input-val"))
        .await
        .map_err(|e| e.to_string())?;
    name_input.clear().await.map_err(|e| e.to_string())?;
    tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;
    name_input
        .send_keys(name)
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css(".select-controller"))
        .await
        .map_err(|e| e.to_string())?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    let elements = c
        .find_all(Locator::Css(".f-item-content"))
        .await
        .map_err(|e| e.to_string())?;

    for element in elements {
        let text = element.text().await.map_err(|e| e.to_string())?;
        if text.contains("国创") {
            element.click().await.map_err(|e| e.to_string())?;
            break;
        }
    }

    let elements = c
        .find_all(Locator::Css(".item-main"))
        .await
        .map_err(|e| e.to_string())?;

    for element in elements {
        let text = element.text().await.map_err(|e| e.to_string())?;
        if text.contains("国产原创相关") {
            element.click().await.map_err(|e| e.to_string())?;
            break;
        }
    }

    let tag_input = c
        .wait()
        .for_element(Locator::Css(".tag-input-wrp .input-val")) // 直接使用组合选择器
        .await
        .map_err(|e| e.to_string())?;

    for _ in 0..3 {
        tag_input
            .send_keys(&format!("{}", Key::Backspace))
            .await
            .map_err(|e| e.to_string())?;
    }

    for tag in tags {
        tag_input
            .send_keys(&format!("{}{}", tag, Key::Enter))
            .await
            .map_err(|e| e.to_string())?;
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;
    }

    c.wait()
        .forever()
        .for_element(Locator::Css("non-existent-element"))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn upload_douyin(path: &str, tags: &Vec<String>, name: &str) -> Result<(), String> {
    let c = get_client().await?;

    c.goto("https://creator.douyin.com/creator-micro/content/upload")
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css("input[type='file']"))
        .await
        .map_err(|e| e.to_string())?
        .send_keys(path)
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css(".semi-input"))
        .await
        .map_err(|e| e.to_string())?
        .send_keys(name)
        .await
        .map_err(|e| e.to_string())?;

    let tag_input = c
        .wait()
        .for_element(Locator::Css(".zone-container"))
        .await
        .map_err(|e| e.to_string())?;

    for tag in tags {
        tag_input
            .send_keys(&format!("#{}{}", tag, Key::Enter))
            .await
            .map_err(|e| e.to_string())?;
    }

    c.wait()
        .for_element(Locator::XPath("//div[contains(text(), '选择封面')]"))
        .await
        .map_err(|e| e.to_string())?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::XPath(
            "//span[@class='semi-button-content' and text()='完成']",
        ))
        .await
        .map_err(|e| e.to_string())?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .forever()
        .for_element(Locator::Css("non-existent-element"))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn upload_ks(path: &str, tags: &Vec<String>, name: &str) -> Result<(), String> {
    let c = get_client().await?;

    c.goto("https://cp.kuaishou.com/article/publish/video")
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css("input[type='file']"))
        .await
        .map_err(|e| e.to_string())?
        .send_keys(path)
        .await
        .map_err(|e| e.to_string())?;

    let tag_input = c
        .wait()
        .for_element(Locator::Id("work-description-edit"))
        .await
        .map_err(|e| e.to_string())?;

    tag_input.send_keys(name).await.map_err(|e| e.to_string())?;

    for tag in tags {
        tag_input
            .send_keys(&format!("#{}{}", tag, Key::Enter))
            .await
            .map_err(|e| e.to_string())?;
    }

    c.wait()
        .forever()
        .for_element(Locator::Css("non-existent-element"))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn upload_wx(path: &str, tags: &Vec<String>, name: &str) -> Result<(), String> {
    let c = get_client().await?;

    c.goto("https://channels.weixin.qq.com/platform/post/create")
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css("input[type='file']"))
        .await
        .map_err(|e| e.to_string())?
        .send_keys(path)
        .await
        .map_err(|e| e.to_string())?;

    let tag_input = c
        .wait()
        .for_element(Locator::Css(".input-editor"))
        .await
        .map_err(|e| e.to_string())?;

    tag_input.send_keys(name).await.map_err(|e| e.to_string())?;

    for tag in tags {
        tag_input
            .send_keys(&format!("#{}", tag))
            .await
            .map_err(|e| e.to_string())?;
    }

    let formatted_name = if name.chars().count() < 5 {
        format!("《{}》", name)
    } else {
        name.to_string()
    };
    c.wait()
        .for_element(Locator::Css(".short-title-wrap .weui-desktop-form__input"))
        .await
        .map_err(|e| e.to_string())?
        .send_keys(&formatted_name)
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .forever()
        .for_element(Locator::Css("non-existent-element"))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn upload_bd(path: &str, tags: &Vec<String>, name: &str) -> Result<(), String> {
    let c = get_client().await?;

    c.goto("https://baijiahao.baidu.com/builder/rc/edit?type=videoV2")
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css("input[type='file']"))
        .await
        .map_err(|e| e.to_string())?
        .send_keys(path)
        .await
        .map_err(|e| e.to_string())?;

    let name_input = c
        .wait()
        .for_element(Locator::Css(".left-area-content-box .cheetah-input"))
        .await
        .map_err(|e| e.to_string())?;

    for _ in 0..15 {
        name_input
            .send_keys(&format!("{}", Key::Backspace))
            .await
            .map_err(|e| e.to_string())?;
    }

    let formatted_name = if name.chars().count() < 9 {
        format!("【《{}》】", name)
    } else {
        name.to_string()
    };

    name_input
        .send_keys(&formatted_name)
        .await
        .map_err(|e| e.to_string())?;

    let tag_input = c
        .wait()
        .for_element(Locator::Css(".edit-video-topic-input"))
        .await
        .map_err(|e| e.to_string())?;

    tag_input
        .send_keys(&format!("{}", Key::Backspace))
        .await
        .map_err(|e| e.to_string())?;

    tag_input
        .send_keys(&tags[0])
        .await
        .map_err(|e| e.to_string())?;

    tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;

    tag_input.click().await.map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css(".topic-text"))
        .await
        .map_err(|e| e.to_string())?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;

    c.wait()
        .for_element(Locator::Css(".video-category .cheetah-select-selector"))
        .await
        .map_err(|e| e.to_string())?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .for_element(Locator::Css("li[title='文化']"))
        .await
        .map_err(|e| e.to_string())?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;

    c.wait()
        .for_element(Locator::Css("li[title='网络小说']"))
        .await
        .map_err(|e| e.to_string())?
        .click()
        .await
        .map_err(|e| e.to_string())?;

    c.wait()
        .forever()
        .for_element(Locator::Css("non-existent-element"))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn upload_all(path: &str, tags: &Vec<String>, name: &str) -> Result<(), String> {
    let c = get_client().await?;
    // 上传bilibili
    {
        c.goto("https://member.bilibili.com/platform/upload/video/frame")
            .await
            .map_err(|e| e.to_string())?;

        c.wait()
            .for_element(Locator::Css(".upload-btn"))
            .await
            .map_err(|e| e.to_string())?;

        let input_element = c
            .wait()
            .for_element(Locator::Css("input[type='file']"))
            .await
            .map_err(|e| e.to_string())?;

        c.execute(
            "document.querySelector('input[type=\"file\"]').style.display = 'block';",
            vec![],
        )
        .await
        .map_err(|e| e.to_string())?;

        input_element
            .send_keys(path)
            .await
            .map_err(|e| e.to_string())?;

        c.wait()
            .for_element(Locator::Css(".img"))
            .await
            .map_err(|e| e.to_string())?
            .click()
            .await
            .map_err(|e| e.to_string())?;
        c.wait()
            .for_element(Locator::XPath("//span[contains(text(), ' 完成 ')]"))
            .await
            .map_err(|e| e.to_string())?
            .click()
            .await
            .map_err(|e| e.to_string())?;

        let name_input = c
            .wait()
            .for_element(Locator::Css(".input-val"))
            .await
            .map_err(|e| e.to_string())?;
        name_input.clear().await.map_err(|e| e.to_string())?;
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;
        name_input
            .send_keys(name)
            .await
            .map_err(|e| e.to_string())?;

        c.wait()
            .for_element(Locator::Css(".select-controller"))
            .await
            .map_err(|e| e.to_string())?
            .click()
            .await
            .map_err(|e| e.to_string())?;

        let elements = c
            .find_all(Locator::Css(".f-item-content"))
            .await
            .map_err(|e| e.to_string())?;

        for element in elements {
            let text = element.text().await.map_err(|e| e.to_string())?;
            if text.contains("国创") {
                element.click().await.map_err(|e| e.to_string())?;
                break;
            }
        }

        let elements = c
            .find_all(Locator::Css(".item-main"))
            .await
            .map_err(|e| e.to_string())?;

        for element in elements {
            let text = element.text().await.map_err(|e| e.to_string())?;
            if text.contains("国产原创相关") {
                element.click().await.map_err(|e| e.to_string())?;
                break;
            }
        }

        let tag_input = c
            .wait()
            .for_element(Locator::Css(".tag-input-wrp .input-val")) // 直接使用组合选择器
            .await
            .map_err(|e| e.to_string())?;

        for _ in 0..3 {
            tag_input
                .send_keys(&format!("{}", Key::Backspace))
                .await
                .map_err(|e| e.to_string())?;
        }

        for tag in tags {
            tag_input
                .send_keys(&format!("{}{}", tag, Key::Enter))
                .await
                .map_err(|e| e.to_string())?;
            tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;
        }
    }

    // 上传抖音
    {
        c.execute(
            "window.open('https://creator.douyin.com/creator-micro/content/upload', '_blank');",
            vec![],
        )
        .await
        .map_err(|e| e.to_string())?;

        let windows = c.windows().await.map_err(|e| e.to_string())?;
        c.switch_to_window(windows.last().unwrap().clone())
            .await
            .map_err(|e| e.to_string())?;

        c.wait()
            .for_element(Locator::Css("input[type='file']"))
            .await
            .map_err(|e| e.to_string())?
            .send_keys(path)
            .await
            .map_err(|e| e.to_string())?;

        c.wait()
            .for_element(Locator::Css(".semi-input"))
            .await
            .map_err(|e| e.to_string())?
            .send_keys(name)
            .await
            .map_err(|e| e.to_string())?;

        let tag_input = c
            .wait()
            .for_element(Locator::Css(".zone-container"))
            .await
            .map_err(|e| e.to_string())?;

        for tag in tags {
            tag_input
                .send_keys(&format!("#{}{}", tag, Key::Enter))
                .await
                .map_err(|e| e.to_string())?;
        }

        c.wait()
            .for_element(Locator::XPath("//div[contains(text(), '选择封面')]"))
            .await
            .map_err(|e| e.to_string())?
            .click()
            .await
            .map_err(|e| e.to_string())?;

        c.wait()
            .for_element(Locator::XPath(
                "//span[@class='semi-button-content' and text()='完成']",
            ))
            .await
            .map_err(|e| e.to_string())?
            .click()
            .await
            .map_err(|e| e.to_string())?;
    }

    //上传快手
    {
        c.execute(
            "window.open('https://cp.kuaishou.com/article/publish/video', '_blank');",
            vec![],
        )
        .await
        .map_err(|e| e.to_string())?;

        let windows = c.windows().await.map_err(|e| e.to_string())?;
        c.switch_to_window(windows.last().unwrap().clone())
            .await
            .map_err(|e| e.to_string())?;

        c.wait()
            .for_element(Locator::Css("input[type='file']"))
            .await
            .map_err(|e| e.to_string())?
            .send_keys(path)
            .await
            .map_err(|e| e.to_string())?;

        let tag_input = c
            .wait()
            .for_element(Locator::Id("work-description-edit"))
            .await
            .map_err(|e| e.to_string())?;

        tag_input.send_keys(name).await.map_err(|e| e.to_string())?;

        for tag in tags {
            tag_input
                .send_keys(&format!("#{}{}", tag, Key::Enter))
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    //上传百度
    {
        c.execute(
            "window.open('https://baijiahao.baidu.com/builder/rc/edit?type=videoV2', '_blank');",
            vec![],
        )
        .await
        .map_err(|e| e.to_string())?;

        let windows = c.windows().await.map_err(|e| e.to_string())?;
        c.switch_to_window(windows.last().unwrap().clone())
            .await
            .map_err(|e| e.to_string())?;

        c.wait()
            .for_element(Locator::Css("input[type='file']"))
            .await
            .map_err(|e| e.to_string())?
            .send_keys(path)
            .await
            .map_err(|e| e.to_string())?;

        let name_input = c
            .wait()
            .for_element(Locator::Css(".left-area-content-box .cheetah-input"))
            .await
            .map_err(|e| e.to_string())?;

        for _ in 0..15 {
            name_input
                .send_keys(&format!("{}", Key::Backspace))
                .await
                .map_err(|e| e.to_string())?;
        }

        let formatted_name = if name.chars().count() < 9 {
            format!("【《{}》】", name)
        } else {
            name.to_string()
        };

        name_input
            .send_keys(&formatted_name)
            .await
            .map_err(|e| e.to_string())?;

        let tag_input = c
            .wait()
            .for_element(Locator::Css(".edit-video-topic-input"))
            .await
            .map_err(|e| e.to_string())?;

        tag_input
            .send_keys(&format!("{}", Key::Backspace))
            .await
            .map_err(|e| e.to_string())?;

        tag_input
            .send_keys(&tags[0])
            .await
            .map_err(|e| e.to_string())?;

        tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;

        tag_input.click().await.map_err(|e| e.to_string())?;

        c.wait()
            .for_element(Locator::Css(".topic-text"))
            .await
            .map_err(|e| e.to_string())?
            .click()
            .await
            .map_err(|e| e.to_string())?;

        tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;

        c.wait()
            .for_element(Locator::Css(".video-category .cheetah-select-selector"))
            .await
            .map_err(|e| e.to_string())?
            .click()
            .await
            .map_err(|e| e.to_string())?;

        tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;

        c.wait()
            .for_element(Locator::Css("li[title='文化']"))
            .await
            .map_err(|e| e.to_string())?
            .click()
            .await
            .map_err(|e| e.to_string())?;

        tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.0)).await;

        c.wait()
            .for_element(Locator::Css("li[title='网络小说']"))
            .await
            .map_err(|e| e.to_string())?
            .click()
            .await
            .map_err(|e| e.to_string())?;
    }

    //上传微信
    {
        c.execute(
            "window.open('https://channels.weixin.qq.com/platform/post/create', '_blank');",
            vec![],
        )
        .await
        .map_err(|e| e.to_string())?;

        let windows = c.windows().await.map_err(|e| e.to_string())?;
        c.switch_to_window(windows.last().unwrap().clone())
            .await
            .map_err(|e| e.to_string())?;

        c.wait()
            .forever()
            .for_element(Locator::Css("input[type='file']"))
            .await
            .map_err(|e| e.to_string())?
            .send_keys(path)
            .await
            .map_err(|e| e.to_string())?;

        let tag_input = c
            .wait()
            .for_element(Locator::Css(".input-editor"))
            .await
            .map_err(|e| e.to_string())?;

        tag_input.send_keys(name).await.map_err(|e| e.to_string())?;

        for tag in tags {
            tag_input
                .send_keys(&format!("#{}", tag))
                .await
                .map_err(|e| e.to_string())?;
        }

        let formatted_name = if name.chars().count() < 5 {
            format!("《{}》", name)
        } else {
            name.to_string()
        };
        c.wait()
            .for_element(Locator::Css(".short-title-wrap .weui-desktop-form__input"))
            .await
            .map_err(|e| e.to_string())?
            .send_keys(&formatted_name)
            .await
            .map_err(|e| e.to_string())?;
    }

    //上传qq
    {
        c.execute(
            "window.open('https://qqzz.qq.com/publish', '_blank');",
            vec![],
        )
        .await
        .map_err(|e| e.to_string())?;

        let windows = c.windows().await.map_err(|e| e.to_string())?;
        c.switch_to_window(windows.last().unwrap().clone())
            .await
            .map_err(|e| e.to_string())?;

        c.wait()
            .forever()
            .for_element(Locator::Css("input[type='file']"))
            .await
            .map_err(|e| e.to_string())?
            .send_keys(path)
            .await
            .map_err(|e| e.to_string())?;

        c.wait()
            .forever()
            .for_element(Locator::Css(".video-parent"))
            .await
            .map_err(|e| e.to_string())?;

        println!("上传成功");

        let tag_input = c
            .wait()
            .forever()
            .for_element(Locator::Css(".topic-editor"))
            .await
            .map_err(|e| e.to_string())?;

        for _ in 0..15 {
            tag_input
                .send_keys(&format!("{}", Key::Backspace))
                .await
                .map_err(|e| e.to_string())?;
        }

        tag_input.send_keys(name).await.map_err(|e| e.to_string())?;

        for tag in tags {
            tag_input
                .send_keys(&format!("#{}{}", tag, Key::Enter))
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    c.wait()
        .forever()
        .for_element(Locator::Css("non-existent-element"))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
