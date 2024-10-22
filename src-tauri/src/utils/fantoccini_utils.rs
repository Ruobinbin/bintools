use fantoccini::ClientBuilder;
use serde_json::{json, Map};

pub async fn execute_browser_operations() -> Result<(), fantoccini::error::CmdError> {
    let mut caps = Map::new();

    caps.insert(
        "ms:edgeOptions".to_string(),
        json!({
            "args": [
                "--user-data-dir=C:/Users/17732/AppData/Local/Microsoft/Edge/User Data" // 设置为你的 Edge 浏览器用户数据目录
            ]
        }),
    );

    let c = ClientBuilder::native()
        .capabilities(caps) // 传递 Map 类型的 capabilities
        .connect("http://localhost:51737") // 确保 WebDriver 在此端口上运行
        .await
        .expect("无法连接到 WebDriver");

    // 打开 Bilibili 视频上传页面
    c.goto("https://member.bilibili.com/platform/upload/video/frame")
        .await?;

    // 获取当前页面的 URL 进行确认
    let url = c.current_url().await?;
    println!("当前页面 URL: {}", url);

    // 关闭浏览器
    c.close().await
}
