use flate2::read::GzDecoder;
use futures::{SinkExt, StreamExt};
use prost::Message;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

mod douyin {
    include!("protobuf/douyin.rs");
}

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0";
pub struct DouyinWs {
    url: String,
    callbacks: Arc<Mutex<HashMap<String, Box<dyn Fn(&Value) + Send + Sync>>>>,
    handle: Option<JoinHandle<()>>, // callbacks: HashMap<String, Box<dyn Fn(&Value) + Send + Sync>>,
}

impl DouyinWs {
    pub fn new(url: &str) -> Self {
        DouyinWs {
            url: url.to_string(),
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            handle: None,
            // callbacks: HashMap::new(),
        }
    }

    pub async fn start(&mut self) {
        let url = self.url.clone();
        let callbacks = self.callbacks.clone();
        let handle = tokio::spawn(async move {
            let mut request =
                tokio_tungstenite::tungstenite::client::IntoClientRequest::into_client_request(url)
                    .expect("创建请求失败");

            request.headers_mut().insert(
                "User-Agent",
                USER_AGENT.parse().expect("User-Agent 格式错误"),
            );

            let ttwid = get_ttwid().await.expect("获取ttwid失败");

            request.headers_mut().insert(
                "Cookie",
                format!("ttwid={}", ttwid).parse().expect("Cookie 格式错误"),
            );

            let (ws_stream, _) = tokio_tungstenite::connect_async(request)
                .await
                .expect("连接失败");

            let (mut write, mut read) = ws_stream.split();

            while let Some(msg) = read.next().await {
                match msg {
                    Ok(msg) => {
                        let data = msg.into_data();
                        let bytes = prost::bytes::Bytes::from(data);
                        let package = douyin::PushFrame::decode(bytes).expect("解码失败");
                        let payload = Cursor::new(package.payload);
                        let mut decoder = GzDecoder::new(payload);
                        let mut decompressed_data = Vec::new();
                        decoder
                            .read_to_end(&mut decompressed_data)
                            .expect("解压失败");
                        let bytes = prost::bytes::Bytes::from(decompressed_data);
                        let response = douyin::Response::decode(bytes).expect("解码失败");

                        if response.need_ack {
                            let mut push_frame = douyin::PushFrame::default();
                            push_frame.log_id = package.log_id;
                            push_frame.payload_type = "ack".to_string();
                            push_frame.payload = response.internal_ext.clone().into_bytes();
                            let encoded = push_frame.encode_to_vec();
                            write
                                .send(tokio_tungstenite::tungstenite::Message::Binary(encoded))
                                .await
                                .expect("发送 ack 失败");
                        }

                        for msg in &response.messages_list {
                            let method = &msg.method;
                            if let Some(callback) = callbacks.lock().await.get(method) {
                                match method.as_str() {
                                    "WebcastChatMessage" => {
                                        let chat_msg =
                                            douyin::ChatMessage::decode(msg.payload.as_slice())
                                                .unwrap();
                                        let user = chat_msg.user.unwrap();
                                        let json_msg = json!({
                                            "content": chat_msg.content,
                                            "user": user.nick_name,
                                        });
                                        callback(&json_msg);
                                    }
                                    "WebcastLikeMessage" => {
                                        let msg =
                                            douyin::LikeMessage::decode(msg.payload.as_slice())
                                                .unwrap();
                                        let user = msg.user.unwrap();
                                        let json_msg = json!({
                                            "user": user.nick_name,
                                            "count": msg.count,
                                        });
                                        callback(&json_msg);
                                    }
                                    "WebcastGiftMessage" => {
                                        let msg =
                                            douyin::GiftMessage::decode(msg.payload.as_slice())
                                                .unwrap();
                                        let user = msg.user.unwrap();
                                        let gift = msg.gift.unwrap();
                                        let json_msg = json!({
                                            "user": user.nick_name,
                                            "gift_name": gift.name,
                                            "count": msg.combo_count,
                                        });
                                        callback(&json_msg);
                                    }
                                    "WebcastMemberMessage" => {
                                        let msg =
                                            douyin::MemberMessage::decode(msg.payload.as_slice())
                                                .unwrap();
                                        let user = msg.user.unwrap();
                                        let json_msg = json!({
                                            "user": user.nick_name,
                                        });
                                        callback(&json_msg);
                                    }
                                    "WebcastSocialMessage" => {
                                        let msg =
                                            douyin::SocialMessage::decode(msg.payload.as_slice())
                                                .unwrap();
                                        let user = msg.user.unwrap();
                                        let json_msg = json!({
                                            "user": user.nick_name,
                                        });
                                        callback(&json_msg);
                                    }
                                    _ => {} //                     // "WebcastMemberMessage" => parse_member_msg(&msg.payload), // 进入直播间消息
                                            //                     // "WebcastSocialMessage" => parse_social_msg(&msg.payload), // 关注消息
                                            //                     // "WebcastRoomUserSeqMessage" => parse_room_user_seq_msg(&msg.payload), // 直播间统计
                                            //                     // "WebcastFansclubMessage" => parse_fansclub_msg(&msg.payload), // 粉丝团消息
                                            //                     // "WebcastControlMessage" => parse_control_msg(&msg.payload), // 直播间状态消息
                                            //                     // "WebcastEmojiChatMessage" => parse_emoji_chat_msg(&msg.payload), // 聊天表情包消息
                                            //                     // "WebcastRoomStatsMessage" => parse_room_stats_msg(&msg.payload), // 直播间统计信息
                                            //                     // "WebcastRoomMessage" => parse_room_msg(&msg.payload), // 直播间信息
                                            //                     // "WebcastRoomRankMessage" => parse_rank_msg(&msg.payload), // 直播间排行榜信息
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("读取消息错误: {}", e);
                        break;
                    }
                }
            }
        });
        self.handle = Some(handle);
    }

    pub async fn on<F>(&self, message_type: &str, callback: F)
    where
        F: Fn(&Value) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().await;
        callbacks.insert(message_type.to_string(), Box::new(callback));
    }

    pub async fn close(&mut self) {
        self.handle.take().unwrap().abort();
    }
}

async fn get_ttwid() -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://live.douyin.com/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }
    if let Some(cookies) = response.headers().get("set-cookie") {
        let cookie_str = cookies.to_str().map_err(|e| e.to_string())?;
        for cookie in cookie_str.split(';') {
            if cookie.trim().starts_with("ttwid=") {
                return Ok(cookie.trim()[6..].to_string());
            }
        }
    }
    Err("未能获取到ttwid".to_string())
}
