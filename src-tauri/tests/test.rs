use bintools_lib::utils;
use tokio;

#[tokio::test]
async fn test() {
    let _ = utils::fantoccini_utils::execute_browser_operations().await;
}
