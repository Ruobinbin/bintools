use bintools_lib::utils;
use tokio;

#[tokio::test]
async fn test() {
    let _ = utils::edge_tts_utils::edge_tts(
        "Microsoft Server Speech Text to Speech Voice (sq-AL, AnilaNeural)",
        "d:\\123456798.wav",
        50,
        "hello world",
    );
    // let voices = utils::edge_tts_utils::get_voices_name_list();
    // for voice in voices {
    //     println!("{}", voice);
    // }
}
