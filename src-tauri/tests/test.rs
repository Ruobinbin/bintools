use bintools_lib::utils;
use tokio;

#[tokio::test]
async fn test_start_gpt_sovits_api() {
    println!("Starting GPT-3 API...");
    match utils::bollard_utils::start_gpt_sovits_api().await {
        Ok(_) => println!("API started successfully"),
        Err(e) => eprintln!("Failed to start API: {}", e),
    }
}
