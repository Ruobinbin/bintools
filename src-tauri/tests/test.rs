use futures::SinkExt;
use std::io::Write;
use tokio;

#[tokio::test]
async fn test() {
    // listen_bilibili_danmu(544853).await;
    // let json_value = serde_json::json!({
    //     "uid": 3493293971212575_i64,
    //     "roomid": 31456606,
    //     "protover": 3,
    //     "buvid": "DF900F0C-6939-5789-48F3-7C751C2F8EE045412infoc",
    //     "platform": "web",
    //     "type": 2,
    //     "key": "vNwVShhJCPgkX-g6y45MjcPp6gxGRyDG_I-2kXMuMAuNPlGtlcLIxMlTllMln2oXVEpqNg57rI43zLVUqiLNdDsayLNJcyRLRuVN4aUfcOmXLFvX-mpiqOwC6OBS7zf2ndpUSRm-uMtIlviaud5n2oEBBwwuC5pVYEq_LXUu2gC4ZT0k4mafHyztl_lY"
    // });
    // let buff = get_certification(&json_value);
    // println!("{}", buff);
    listen_bilibili_danmu(31456606).await;
}

async fn listen_bilibili_danmu(room_id: u64) {
    // 定义基础URL常量
    const ROOM_INIT_URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";

    // 获取弹幕服务器配置
    let conf_response = match reqwest::Client::new()
        .get(format!(
            "https://api.live.bilibili.com/room/v1/Danmu/getConf?room_id={}&platform=pc&player=web",
            room_id
        ))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            println!("请求弹幕服务器配置失败: {}", e);
            return;
        }
    };

    let conf_json = match conf_response.json::<serde_json::Value>().await {
        Ok(json) => json,
        Err(e) => {
            println!("解析弹幕服务器配置失败: {}", e);
            return;
        }
    };
    let token = match conf_json["data"]["token"].as_str() {
        Some(token) => token,
        None => {
            println!("获取token失败");
            return;
        }
    };
    println!("{}", token);

    let host_server_list = match conf_json["data"]["host_server_list"].as_array() {
        Some(list) => list,
        None => {
            println!("获取host_server_list失败");
            return;
        }
    };
    println!("{:?}", host_server_list);

    // 随机选择一个服务器
    let random_host = &host_server_list[rand::random::<usize>() % host_server_list.len()];
    let host = match random_host["host"].as_str() {
        Some(h) => h,
        None => {
            println!("获取host失败");
            return;
        }
    };
    let port = match random_host["port"].as_u64() {
        Some(p) => p,
        None => {
            println!("获取port失败");
            return;
        }
    };
    let ws_url = format!("wss://{}:{}/sub", host, port);
    let ws_url = "wss://broadcastlv.chat.bilibili.com:443/sub";
    println!("{}", ws_url);

    use futures::StreamExt;
    use tokio_tungstenite::{connect_async, tungstenite::Message};

    // 连接WebSocket
    let (ws_stream, _) = match connect_async(ws_url).await {
        Ok(conn) => conn,
        Err(e) => {
            println!("连接WebSocket失败: {}", e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();

    let json_value = serde_json::json!({
        "uid": 3493293971212575_i64,
        "roomid": 31456606,
        "protover": 3,
        "buvid": "DF900F0C-6939-5789-48F3-7C751C2F8EE045412infoc",
        "platform": "web",
        "type": 2,
        "key": token
    });
    let buff = get_certification(&json_value);
    println!("{}", buff);

    let auth_packet = Message::Text(buff);
    if let Err(e) = write.send(auth_packet).await {
        println!("发送认证包失败: {}", e);
        return;
    }

    // 处理接收到的消息
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => match msg {
                Message::Text(text) => {
                    println!("收到文本消息: {}", text);
                }
                Message::Binary(data) => {
                    println!("收到二进制消息, 长度: {}", data.len());
                }
                _ => {}
            },
            Err(e) => {
                println!("接收消息错误: {}", e);
                break;
            }
        }
    }
}

fn get_certification(json_value: &serde_json::Value) -> String {
    // 将 JSON 对象序列化为字符串
    let json_str = json_value.to_string();
    let json_bytes = json_str.as_bytes();
    let mut buff = Vec::with_capacity(json_bytes.len() + 16);

    // 整个数据包长度
    buff.extend_from_slice(&(json_bytes.len() as u32 + 16).to_be_bytes());
    // 头部长度
    buff.extend_from_slice(&16u16.to_be_bytes());
    // 协议版本
    buff.extend_from_slice(&1u16.to_be_bytes());
    // 类型, 7为加入房间认证
    buff.extend_from_slice(&7u32.to_be_bytes());
    // 填1
    buff.extend_from_slice(&1u32.to_be_bytes());

    // 填入数据
    buff.extend_from_slice(json_bytes);

    // 转换为16进制字符串
    buff.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}
