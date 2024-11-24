use super::bilibili_live_utils::BilibiliWs;
use super::default_utils::ensure_path_exists;
use super::douyin_live_utils::DouyinWs;
use crate::user_files_dir;
use regex::Regex;
use std::io::{self, BufRead, Write};
use std::process::{ChildStdin, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MinecraftLive {
    stdin: Arc<Mutex<ChildStdin>>,
}

impl MinecraftLive {
    fn clone(&self) -> Self {
        MinecraftLive {
            stdin: Arc::clone(&self.stdin),
        }
    }
    pub async fn start() -> Result<Self, String> {
        let mc_server_dir = user_files_dir().join("mc_live");
        ensure_path_exists(&mc_server_dir)
            .map_err(|e| format!("创建Minecraft服务器目录失败: {}", e))?;
        let driver_path = mc_server_dir.join("server.jar");
        if !driver_path.exists() {
            return Err("未找到server.jar".to_string());
        }

        let mut child = Command::new("java")
            .current_dir(&mc_server_dir)
            .args(["-Xms1G", "-Xmx6G", "-jar", "server.jar"])
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("启动Minecraft服务器失败: {}", e))?;

        if let Some(stdout) = child.stdout.take() {
            let reader = io::BufReader::new(stdout);

            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("You need to agree to the EULA in order to run the server") {
                        return Err("需要同意EULA才能运行服务器".to_string());
                    }
                    if line.contains("For help, type \"help\"") {
                        break;
                    }
                }
            }
        }

        let stdin = Arc::new(Mutex::new(child.stdin.take().unwrap()));

        Ok(MinecraftLive { stdin })
    }

    pub async fn connect_douyin(&self, douyin_wss_url: String) -> Result<(), String> {
        let mut douyin_ws = DouyinWs::new(&douyin_wss_url);

        douyin_ws
            .on("WebcastMemberMessage", {
                let self_arc = self.clone();
                move |data| {
                    let user = data["user"].to_string();
                    self_arc.welcome(user.clone());
                    self_arc.summon_entity(user.clone(), "minecraft:iron_golem", 1, None);
                    self_arc.summon_entity(user.clone(), "minecraft:zombie", 10, None)
                }
            })
            .await;

        douyin_ws
            .on("WebcastSocialMessage", {
                let self_arc = self.clone();
                move |data| {
                    let user = data["user"].to_string();
                    self_arc.thank_follow(user.clone());
                    self_arc.summon_entity(user.clone(), "minecraft:iron_golem", 1, None);
                    self_arc.summon_entity(user.clone(), "minecraft:zombie", 10, None)
                }
            })
            .await;

        douyin_ws
            .on("WebcastChatMessage", {
                let self_arc = self.clone();
                move |data| {
                    let user = data["user"].to_string();
                    let content = data["content"].to_string();
                    if content.contains("铁傀儡") {
                        self_arc.summon_entity(user.clone(), "minecraft:iron_golem", 1, None);
                    }
                    if content.contains("僵尸") {
                        self_arc.summon_entity(user.clone(), "minecraft:zombie", 10, None)
                    }
                }
            })
            .await;

        douyin_ws
            .on("WebcastLikeMessage", {
                let self_arc = self.clone();
                move |data| {
                    let count = data["count"].as_u64().unwrap();
                    let user = data["user"].to_string();
                    self_arc.summon_entity(user, "minecraft:zombie", count, None);
                }
            })
            .await;

        douyin_ws
            .on("WebcastGiftMessage", {
                let self_arc = self.clone();
                move |data| {
                    let count = data["count"].as_u64().unwrap();
                    let user = data["user"].to_string();
                    let gift_name = data["gift_name"].to_string();
                    self_arc.thank_gift(user.clone(), gift_name.clone(), count);
                    if gift_name.contains("小心心") {
                        self_arc.summon_entity(
                            format!("{}【{}】", user, gift_name),
                            "minecraft:zombie",
                            count * 100,
                            Some("ArmorItems:[{Count:1b,id:netherite_boots},{Count:1b,id:netherite_leggings},{Count:1b,id:netherite_chestplate},{Count:1b,id:netherite_helmet}], HandItems:[{Count:1b,id:netherite_sword}]".to_string()),
                        )
                    } else if gift_name.contains("玫瑰") {
                        self_arc.summon_entity(
                            format!("{}【{}】", user, gift_name),
                            "minecraft:iron_golem",
                            count * 100,
                            None,
                        )
                    } else {
                        self_arc.summon_entity(
                            format!("{}【{}】", user, gift_name),
                            "minecraft:iron_golem",
                            count*100,
                            None,
                        )
                    }
                }
            })
            .await;
        douyin_ws.start().await;
        Ok(())
    }

    pub async fn connect_bilibili(
        &self,
        room_id: String,
        cookie: Option<String>,
    ) -> Result<(), String> {
        let mut bilibili_ws = BilibiliWs::new(room_id.as_str(), cookie);
        bilibili_ws
            .on("INTERACT_WORD", {
                let self_arc = self.clone();
                move |data| {
                    let user = data["user"].as_str().unwrap().to_string();
                    let msg_type = data["msg_type"].as_i64().unwrap();
                    if msg_type == 1 {
                        self_arc.welcome(user.clone());
                        self_arc.summon_entity(user.clone(), "minecraft:iron_golem", 1, None);
                        self_arc.summon_entity(user.clone(), "minecraft:zombie", 10, None);
                    } else if msg_type == 2 {
                        self_arc.thank_follow(user.clone());
                        self_arc.summon_entity(user.clone(), "minecraft:iron_golem", 1, None);
                        self_arc.summon_entity(user.clone(), "minecraft:zombie", 10, None);
                    }
                }
            })
            .await;

        bilibili_ws
            .on("LIKE_INFO_V3_CLICK", {
                let self_arc = self.clone();
                move |data| {
                    let user = data["user"].as_str().unwrap().to_string();
                    self_arc.summon_entity(user.clone(), "minecraft:zombie", 50, None);
                }
            })
            .await;
        bilibili_ws
            .on("DANMU_MSG", {
                let self_arc = self.clone();
                move |data| {
                    let user = data["user"].to_string();
                    let content = data["content"].to_string();
                    if content.contains("铁傀儡") {
                        self_arc.summon_entity(user.clone(), "minecraft:iron_golem", 1, None);
                    }
                    if content.contains("僵尸") {
                        self_arc.summon_entity(user.clone(), "minecraft:zombie", 10, None);
                    }
                }
            })
            .await;

        bilibili_ws
            .on("SEND_GIFT", {
                let self_arc = self.clone();
                move |data| {
                    let user = data["user"].to_string();
                    let gift_name = data["gift_name"].to_string();
                    let count = data["count"].as_u64().unwrap();
                    self_arc.thank_gift(user.clone(), gift_name.clone(), count);
                    if gift_name.contains("小花花") {
                        self_arc.summon_entity(
                            format!("{}【{}】", user, gift_name),
                            "minecraft:zombie",
                            count * 100,
                            Some("ArmorItems:[{Count:1b,id:netherite_boots},{Count:1b,id:netherite_leggings},{Count:1b,id:netherite_chestplate},{Count:1b,id:netherite_helmet}], HandItems:[{Count:1b,id:netherite_sword}]".to_string()),
                        )
                    } else if gift_name.contains("人气票") {
                        self_arc.summon_entity(
                            user.clone(),
                            "minecraft:iron_golem",
                            count * 100,
                            None,
                        );
                    } else {
                        self_arc.summon_entity(
                            format!("{}【{}】", user, gift_name),
                            "minecraft:iron_golem",
                            count * 100,
                            None,
                        );
                    }
                }
            })
            .await;
        bilibili_ws.start().await;
        Ok(())
    }
    pub fn summon_entity(
        &self,
        username: String,
        entity_id: &str,
        count: u64,
        tag: Option<String>,
    ) {
        let username = format_username(&username);
        let stdin = self.stdin.clone();
        let entity_id = entity_id.to_string();
        tokio::spawn(async move {
            // CustomNameVisible:1b
            let command = format!(
                "/summon {} 0 5 0 {{CustomName:\"{}\",{}}}",
                entity_id,
                username,
                tag.unwrap_or_default()
            );
            for _ in 0..count {
                let mut stdin_lock = stdin.lock().await;
                writeln!(stdin_lock, "{}", command).unwrap();
                stdin_lock.flush().unwrap();
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        });
    }
    pub fn welcome(&self, user: String) {
        let user = format_username(&user);
        let command = format!(
            "/title @a title {{\"text\":\"欢迎【{}】\",\"color\":\"gold\"}}",
            user
        );
        let stdin = self.stdin.clone();
        tokio::spawn(async move {
            let mut stdin_lock = stdin.lock().await;
            writeln!(stdin_lock, "{}", command).unwrap();
            stdin_lock.flush().unwrap();
        });
    }

    pub fn thank_follow(&self, user: String) {
        let user = format_username(&user);
        let command = format!(
            "/title @a title {{\"text\":\"感谢【{}】关注\",\"color\":\"gold\",\"bold\":true}}",
            user
        );
        let stdin = self.stdin.clone();
        tokio::spawn(async move {
            let mut stdin_lock = stdin.lock().await;
            writeln!(stdin_lock, "{}", command).unwrap();
            stdin_lock.flush().unwrap();
        });
    }

    pub fn thank_gift(&self, user: String, gift_name: String, count: u64) {
        let user = format_username(&user);
        let gift_name = format_username(&gift_name);
        let command = format!(
            "/title @a title {{\"text\":\"感谢【{}】的{} x{}\",\"color\":\"gold\",\"bold\":true}}",
            user, gift_name, count
        );
        let stdin = self.stdin.clone();
        tokio::spawn(async move {
            let mut stdin_lock = stdin.lock().await;
            writeln!(stdin_lock, "{}", command).unwrap();
            stdin_lock.flush().unwrap();
        });
    }
}
pub fn format_username(username: &str) -> String {
    let re = Regex::new(r"[a-zA-Z0-9\u4e00-\u9fa5]").unwrap();
    let filtered_username: String = username
        .chars()
        .filter(|c| re.is_match(&c.to_string()))
        .take(8)
        .collect();

    if filtered_username.is_empty() {
        "XXX".to_string()
    } else {
        filtered_username
    }
}
