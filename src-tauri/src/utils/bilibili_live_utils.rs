use brotli::BrotliDecompress;
use futures::{SinkExt, StreamExt};
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Cursor;
use std::time::Duration;

pub struct WsPacket {
    pub len: Option<i32>,
    pub head_len: Option<i32>,
    pub ver: Option<i32>,
    pub packet_type: Option<i32>,
    pub num: Option<i32>,
    pub body: Option<String>,
}

pub struct bilibiliWs {
    live_id: String,
    cookie: Option<String>,
    callbacks: HashMap<String, Box<dyn Fn(&Value) + Send + Sync>>,
}

impl bilibiliWs {
    pub fn new(live_id: &str, cookie: Option<String>) -> Self {
        bilibiliWs {
            live_id: live_id.to_string(),
            cookie,
            callbacks: HashMap::new(),
        }
    }

    pub async fn start(&mut self) {
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.live.bilibili.com/room/v1/Room/get_info?room_id={}",
            self.live_id
        );
        let response = client.get(&url).send().await.expect("获取房间信息失败");
        let room_info: Value = response.json().await.expect("解析JSON失败");
        let room_id = room_info["data"]["room_id"]
            .as_i64()
            .expect("获取房间ID失败");
        let danmu_info_url = format!(
            "https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo?id={}",
            room_id
        );
        let mut uid = 0;
        let mut buvid = "";
        let danmu_info = match &self.cookie {
            Some(cookie) => {
                let response = client
                    .get(&danmu_info_url)
                    .header("Cookie", cookie)
                    .send()
                    .await
                    .expect("获取弹幕服务器配置失败")
                    .json::<Value>()
                    .await
                    .expect("解析弹幕服务器配置失败");

                let cookies = cookie.split(';').collect::<Vec<_>>();
                uid = cookies
                    .iter()
                    .find(|c| c.trim().starts_with("DedeUserID="))
                    .and_then(|c| c.split('=').nth(1))
                    .and_then(|id| id.parse::<i64>().ok())
                    .unwrap_or(0);
                buvid = cookies
                    .iter()
                    .find(|c| c.trim().starts_with("buvid3="))
                    .and_then(|c| c.split('=').nth(1))
                    .unwrap_or("");
                response
            }
            None => client
                .get(&danmu_info_url)
                .send()
                .await
                .expect("获取弹幕服务器配置失败")
                .json::<Value>()
                .await
                .expect("解析弹幕服务器配置失败"),
        };
        let host = danmu_info["data"]["host_list"]
            .as_array()
            .expect("获取弹幕服务器列表失败")
            .choose(&mut rand::thread_rng())
            .expect("选择弹幕服务器失败");
        let url = format!(
            "wss://{}:{}/sub",
            host["host"].as_str().expect("获取host失败"),
            host["wss_port"].as_i64().expect("获取端口失败")
        );

        let certification = json!({
            "uid": uid,
            "roomid": room_id,
            "protover": 3,
            "buvid": buvid,
            "platform": "web",
            "type": 2,
            "key": danmu_info["data"]["token"].as_str().expect("获取token失败")
        });

        let (ws_stream, _) = tokio_tungstenite::connect_async(url)
            .await
            .expect("连接失败");
        let (mut write, mut read) = ws_stream.split();

        // 发送认证包
        let mut buff = Vec::with_capacity(certification.to_string().len() + 16);
        // 整个数据包长度
        buff.extend_from_slice(&((certification.to_string().len() + 16) as u32).to_be_bytes());
        // 头部长度
        buff.extend_from_slice(&16u16.to_be_bytes());
        // 协议版本
        buff.extend_from_slice(&1u16.to_be_bytes());
        // 类型,7为加入房间认证
        buff.extend_from_slice(&7u32.to_be_bytes());
        // 填1
        buff.extend_from_slice(&1u32.to_be_bytes());
        // 填入数据
        buff.extend_from_slice(certification.to_string().as_bytes());
        write
            .send(tokio_tungstenite::tungstenite::Message::Binary(buff))
            .await
            .expect("发送认证包失败");

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                let mut buff = Vec::with_capacity(16);
                // 整个封包长度
                buff.extend_from_slice(&16u32.to_be_bytes());
                // 头部长度
                buff.extend_from_slice(&16u16.to_be_bytes());
                // 协议版本
                buff.extend_from_slice(&1u16.to_be_bytes());
                // 操作码,2为心跳包
                buff.extend_from_slice(&2u32.to_be_bytes());
                // 填1
                buff.extend_from_slice(&1u32.to_be_bytes());
                write
                    .send(tokio_tungstenite::tungstenite::Message::Binary(buff))
                    .await
                    .expect("发送心跳包失败");
            }
        });

        // 处理接收到的消息
        while let Some(msg) = read.next().await {
            match msg {
                Ok(msg) => {
                    let data = msg.into_data();
                    let mut offset = 0;
                    while offset < data.len() {
                        let packet_len =
                            u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap())
                                as usize;
                        let head_len =
                            u16::from_be_bytes(data[offset + 4..offset + 6].try_into().unwrap())
                                as usize;
                        let packet_ver =
                            u16::from_be_bytes(data[offset + 6..offset + 8].try_into().unwrap());
                        let _packet_type =
                            u32::from_be_bytes(data[offset + 8..offset + 12].try_into().unwrap());
                        let _num =
                            u32::from_be_bytes(data[offset + 12..offset + 16].try_into().unwrap());
                        let data_array = &data[offset + head_len..offset + packet_len];
                        if packet_ver == 3 {
                            let mut decompressed = Vec::new();
                            let mut reader = Cursor::new(data_array);
                            BrotliDecompress(&mut reader, &mut decompressed)
                                .expect("brotli解压失败");
                            let mut offset_ver3 = 0;
                            while offset_ver3 < decompressed.len() {
                                let packet_len = u32::from_be_bytes(
                                    decompressed[offset_ver3..offset_ver3 + 4]
                                        .try_into()
                                        .unwrap(),
                                ) as usize;
                                let head_len = u16::from_be_bytes(
                                    decompressed[offset_ver3 + 4..offset_ver3 + 6]
                                        .try_into()
                                        .unwrap(),
                                ) as usize;
                                let _packet_ver = u16::from_be_bytes(
                                    decompressed[offset_ver3 + 6..offset_ver3 + 8]
                                        .try_into()
                                        .unwrap(),
                                );
                                let _packet_type = u32::from_be_bytes(
                                    decompressed[offset_ver3 + 8..offset_ver3 + 12]
                                        .try_into()
                                        .unwrap(),
                                );
                                let _num = u32::from_be_bytes(
                                    decompressed[offset_ver3 + 12..offset_ver3 + 16]
                                        .try_into()
                                        .unwrap(),
                                );
                                let data_array =
                                    &decompressed[offset_ver3 + head_len..offset_ver3 + packet_len];
                                let body = String::from_utf8_lossy(data_array);
                                let json_value: Value = serde_json::from_str(&body).unwrap();
                                let cmd = json_value["cmd"].as_str().unwrap();
                                if let Some(callback) = self.callbacks.get(cmd) {
                                    match cmd {
                                        "DANMU_MSG" => {
                                            let json_msg = json!({
                                                "user": json_value["info"][2][1].as_str().unwrap(),
                                                "content": json_value["info"][1].as_str().unwrap(),
                                            });
                                            callback(&json_msg);
                                        }
                                        "SEND_GIFT" => {
                                            let json_msg = json!({
                                                "user": json_value["data"]["uname"].as_str().unwrap(),
                                                "gift_name": json_value["data"]["giftName"].as_str().unwrap(),
                                                "count": json_value["data"]["num"].as_i64().unwrap(),
                                            });
                                            callback(&json_msg);
                                        }
                                        "INTERACT_WORD" => {
                                            let json_msg = json!({
                                                "user": json_value["data"]["uname"].as_str().unwrap(),
                                                "msg_type": json_value["data"]["msg_type"].as_i64().unwrap(),
                                            });
                                            callback(&json_msg);
                                        }
                                        "LIKE_INFO_V3_CLICK" => {
                                            let json_msg = json!({
                                                "user": json_value["data"]["uname"].as_str().unwrap(),
                                            });
                                            callback(&json_msg);
                                        }
                                        _ => {}
                                    }
                                }
                                offset_ver3 += packet_len;
                            }
                        } else {
                            // if packet_type == 3 {
                            //     let body = u32::from_be_bytes(data_array[0..4].try_into().unwrap());
                            // } else {
                            //     let body = String::from_utf8_lossy(data_array);
                            // }
                        }
                        offset += packet_len;
                    }
                }
                Err(e) => {
                    println!("接收消息错误: {:?}", e);
                }
            }
        }
    }

    pub fn on<F>(&mut self, message_type: &str, callback: F)
    where
        F: Fn(&Value) + Send + Sync + 'static,
    {
        self.callbacks
            .insert(message_type.to_string(), Box::new(callback));
    }
}
