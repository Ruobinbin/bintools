use super::bilibili_live_utils::bilibiliWs;
use super::default_utils::ensure_path_exists;
use super::douyin_live_utils::douyinWs;
use crate::user_files_dir;
use reqwest::cookie;
use std::io::{self, BufRead};
use std::io::{stdin, Write};
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
pub async fn start_minecraft_live() -> Result<(), String> {
    // let mc_server_dir = user_files_dir().join("mc_live");
    // ensure_path_exists(&mc_server_dir).unwrap();
    let path = "D:\\-code\\bintools\\src-tauri\\target\\debug\\user_files\\mc_live";
    let driver_path = Path::new(path).join("server.jar");
    if !driver_path.exists() {
        println!("server.exe not found");
        return Err("server.exe not found".to_string());
    }

    let mut child = Command::new("java")
        .current_dir(std::path::Path::new(path))
        .args(["-Xms1G", "-Xmx6G", "-jar", "server.jar"])
        .stdout(Stdio::piped()) // 确保 stdout 被设置为管道
        .stdin(Stdio::piped()) // 确保 stdin 被设置为管道
        .spawn()
        .map_err(|e| format!("启动Minecraft服务器失败: {}", e))?;

    if let Some(stdout) = child.stdout.take() {
        let reader = io::BufReader::new(stdout);
        let mut server_started = false;
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("服务器输出: {}", line);
                if line.contains("For help, type \"help\"") {
                    println!("服务器启动完成！");
                    server_started = true;
                    break;
                }
            }
        }
        if !server_started {
            return Err("服务器启动失败".to_string());
        }
    }
    // 获取 stdin 句柄
    let stdin = Arc::new(Mutex::new(child.stdin.take().unwrap()));
    // let arc_stdin_dy = Arc::clone(&stdin);
    // tokio::spawn(async move {
    //     let ws_url = "wss://webcast5-ws-web-hl.douyin.com/webcast/im/push/v2/?app_name=douyin_web&version_code=180800&webcast_sdk_version=1.0.14-beta.0&update_version_code=1.0.14-beta.0&compress=gzip&device_platform=web&cookie_enabled=true&screen_width=1920&screen_height=1080&browser_language=zh-CN&browser_platform=Win32&browser_name=Mozilla&browser_version=5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/130.0.0.0%20Safari/537.36%20Edg/130.0.0.0&browser_online=true&tz_name=Asia/Shanghai&cursor=t-1731208260251_r-1_d-1_u-1_fh-7435482025014170651&internal_ext=internal_src:dim|wss_push_room_id:7435441451519232783|wss_push_did:7349101092921476660|first_req_ms:1731208260149|fetch_time:1731208260251|seq:1|wss_info:0-1731208260251-0-0|wrds_v:7435482859265067035&host=https://live.douyin.com&aid=6383&live_id=1&did_rule=3&endpoint=live_pc&support_wrds=1&user_unique_id=7349101092921476660&im_path=/webcast/im/fetch/&identity=audience&need_persist_msg_count=15&insert_task_id=&live_reason=&room_id=7435441451519232783&heartbeatDuration=0&signature=6DVWraXJM4KhZ8y2";
    //     let mut douyin_ws = douyinWs::new(ws_url);

    //     let stdin_clone = Arc::clone(&arc_stdin_dy);
    //     douyin_ws.on("WebcastChatMessage", move |data| {
    //         send_command(
    //             &mut stdin_clone.lock().unwrap(),
    //             data["user"].as_str().unwrap(),
    //             "minecraft:iron_golem",
    //             1,
    //         )
    //         .unwrap();
    //     });

    //     let stdin_clone = Arc::clone(&arc_stdin_dy);
    //     douyin_ws.on("WebcastLikeMessage", move |data| {
    //         let count = data["count"].as_u64().unwrap();
    //         send_command(
    //             &mut stdin_clone.lock().unwrap(),
    //             data["user"].as_str().unwrap(),
    //             "minecraft:zombie",
    //             count,
    //         )
    //         .unwrap();
    //     });

    //     let stdin_clone = Arc::clone(&arc_stdin_dy);
    //     douyin_ws.on("WebcastGiftMessage", move |data| {
    //         let count = data["count"].as_u64().unwrap();
    //         let gift_id = data["gift_id"].as_u64().unwrap();
    //         println!(
    //             "收到来自 {} 的礼物: {}",
    //             data["user"].as_str().unwrap(),
    //             gift_id
    //         );
    //         send_command(
    //             &mut stdin_clone.lock().unwrap(),
    //             data["user"].as_str().unwrap(),
    //             "minecraft:drowned",
    //             count,
    //         )
    //         .unwrap();
    //     });

    //     douyin_ws.start().await;
    // });

    let live_id = "27820329";
    let cookie = Some(String::from("buvid3=DF900F0C-6939-5789-48F3-7C751C2F8EE045412infoc; b_nut=1710410245; i-wanna-go-back=-1; b_ut=7; _uuid=ECCEA10FF-31012-382B-4F2A-15BFEFD710F5746371infoc; buvid4=0C13F63D-892E-F466-2EB5-6D2A0AB1AC6748532-024031409-9XY8ycWccVDgWVkZ5Fi%2FQQ%3D%3D; rpdid=|(u)~mmuul~J0J'u~u|Ru|km~; hit-dyn-v2=1; LIVE_BUVID=AUTO1917104150894577; buvid_fp_plain=undefined; is-2022-channel=1; CURRENT_BLACKGAP=0; FEED_LIVE_VERSION=V_WATCHLATER_PIP_WINDOW3; enable_web_push=DISABLE; header_theme_version=CLOSE; fingerprint=ca11c94267fa5b66c8a4f226281de73d; CURRENT_QUALITY=80; bili_ticket=eyJhbGciOiJIUzI1NiIsImtpZCI6InMwMyIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MzEyNjM1NTIsImlhdCI6MTczMTAwNDI5MiwicGx0IjotMX0.57_sk2FlvqNcgGnS1MqwvnxMDgsuyMOuoK8G_877tyI; bili_ticket_expires=1731263492; bp_t_offset_3493293971212575=997152949270478848; SESSDATA=53e5b208%2C1746558195%2Cff7b7%2Ab1CjCGcAeb4cZzb2ElATwpwyVPl6h59tAfOJjSRcs9vZv_XpFomIYrT4lxNfLUl9KQb58SVnpqM0xDSUdGWjBRMkxxbUFlSzVZeFlmOWN3NlV4LUpEU0UtWG1XVUg0NEdEbzRUM2FoTmRhOVRONnRvVXJ6Z2ozODc4V2FYTUs4eU5JSTJYT1FEbUpBIIEC; bili_jct=0615911a3938adb8359aec204d2eaacf; DedeUserID=27489840; DedeUserID__ckMd5=115c002c23796687; sid=7io7llk6; buvid_fp=ca11c94267fa5b66c8a4f226281de73d; home_feed_column=5; browser_resolution=1912-920; Hm_lvt_8a6e55dbd2870f0f5bc9194cddf32a02=1730685184,1730731213,1730748905,1731127464; CURRENT_FNVAL=4048; b_lsid=D54AAFBB_193140D7133; bp_t_offset_27489840=998022770342232064; PVID=3"));
    let mut bilibili_ws = bilibiliWs::new(live_id, cookie);
    let stdin_clone = Arc::clone(&stdin);
    bilibili_ws.on("DANMU_MSG", move |data| {
        let content = data["content"].as_str().unwrap();
        if content.contains("铁傀儡") {
            send_command(
                &mut stdin_clone.lock().unwrap(),
                data["user"].as_str().unwrap(),
                "minecraft:iron_golem",
                3,
                None,
            )
            .unwrap();
            return;
        }
        if content.contains("僵尸") {
            send_command(
                &mut stdin_clone.lock().unwrap(),
                data["user"].as_str().unwrap(),
                "minecraft:zombie",
                10,
                None,
            )
            .unwrap();
            return;
        }
    });
    let stdin_clone = Arc::clone(&stdin);
    bilibili_ws.on("SEND_GIFT", move |data| {
        let count = data["count"].as_u64().unwrap();
        let gift_name = data["gift_name"].as_str().unwrap();
        if gift_name == "辣条" {
            send_command(
                &mut stdin_clone.lock().unwrap(),
                data["user"].as_str().unwrap(),
                "minecraft:iron_golem",
                count * 10,
                None,
            )
            .unwrap();
        }
        if gift_name == "小花花" {
            send_command(
                &mut stdin_clone.lock().unwrap(),
                data["user"].as_str().unwrap(),
                "minecraft:zombie",
                99,
                None,
            )
            .unwrap();
        }
        if gift_name == "人气票" {
            send_command(
                &mut stdin_clone.lock().unwrap(),
                data["user"].as_str().unwrap(),
                "mutantmonsters:mutant_snow_golem",
                99,
                None,
            )
            .unwrap();
        }
        if gift_name == "这个好诶" {
            send_command(
                &mut stdin_clone.lock().unwrap(),
                data["user"].as_str().unwrap(),
                "minecraft:iron_golem",
                199,
                None,
            )
            .unwrap();
        }
        if gift_name == "666" {
            send_command(
                &mut stdin_clone.lock().unwrap(),
                data["user"].as_str().unwrap(),
                "mutantmonsters:mutant_zombie",
                99,
                None,
            )
            .unwrap();
        }
        if gift_name == "粉丝团灯牌" {
            send_command(
                &mut stdin_clone.lock().unwrap(),
                data["user"].as_str().unwrap(),
                "minecraft:tnt",
                99,
                Some("{fuse:80}"),
            )
            .unwrap();
        }
    });
    let stdin_clone = Arc::clone(&stdin);
    bilibili_ws.on("INTERACT_WORD", move |data| {
        let msg_type = data["msg_type"].as_u64().unwrap();
        if msg_type == 1 {
            let (entity, count) = if rand::random() {
                ("mutantmonsters:mutant_zombie", 5)
            } else {
                ("minecraft:iron_golem", 10)
            };
            send_command(
                &mut stdin_clone.lock().unwrap(),
                data["user"].as_str().unwrap(),
                entity,
                count,
                None,
            )
            .unwrap();
        }
        if msg_type == 2 {
            send_command(
                &mut stdin_clone.lock().unwrap(),
                data["user"].as_str().unwrap(),
                "mutantmonsters:mutant_zombie",
                99,
                None,
            )
            .unwrap();
        }
    });
    let stdin_clone = Arc::clone(&stdin);
    bilibili_ws.on("LIKE_INFO_V3_CLICK", move |data| {
        send_command(
            &mut stdin_clone.lock().unwrap(),
            data["user"].as_str().unwrap(),
            "minecraft:zombie",
            30,
            None,
        )
        .unwrap();
    });
    bilibili_ws.start().await;

    Ok(())
}
fn send_command(
    stdin: &mut std::process::ChildStdin,
    username: &str,
    entity_id: &str,
    count: u64,
    tag: Option<&str>, // 使用 Option 来表示可选的 tag
) -> Result<(), String> {
    for _ in 0..count {
        let command = match tag {
            Some(t) => format!(
                "/summon {} 0 5 0 {{CustomName:\"{}\",CustomNameVisible:1b}}, {}",
                entity_id, username, t
            ),
            None => format!(
                "/summon {} 0 5 0 {{CustomName:\"{}\",CustomNameVisible:1b}}",
                entity_id, username
            ),
        };

        writeln!(stdin, "{}", command).map_err(|e| format!("发送命令失败: {}", e))?;
        stdin.flush().map_err(|e| format!("刷新stdin失败: {}", e))?;

        // 使用同步的 sleep
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    Ok(())
}
