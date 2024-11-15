use bintools_lib::utils::{
    bilibili_live_utils::bilibiliWs, douyin_live_utils::douyinWs,
    minecraft_live_utlis::start_minecraft_live,
};
use tokio;
mod douyin {
    include!("../src/utils/protobuf/douyin.rs");
}
#[tokio::test]
async fn test() {
    // let live_id = "407149";
    // let cookie = Some(String::from("buvid3=DF900F0C-6939-5789-48F3-7C751C2F8EE045412infoc; b_nut=1710410245; i-wanna-go-back=-1; b_ut=7; _uuid=ECCEA10FF-31012-382B-4F2A-15BFEFD710F5746371infoc; buvid4=0C13F63D-892E-F466-2EB5-6D2A0AB1AC6748532-024031409-9XY8ycWccVDgWVkZ5Fi%2FQQ%3D%3D; rpdid=|(u)~mmuul~J0J'u~u|Ru|km~; hit-dyn-v2=1; LIVE_BUVID=AUTO1917104150894577; buvid_fp_plain=undefined; is-2022-channel=1; CURRENT_BLACKGAP=0; FEED_LIVE_VERSION=V_WATCHLATER_PIP_WINDOW3; enable_web_push=DISABLE; header_theme_version=CLOSE; fingerprint=ca11c94267fa5b66c8a4f226281de73d; CURRENT_QUALITY=80; bili_ticket=eyJhbGciOiJIUzI1NiIsImtpZCI6InMwMyIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MzEyNjM1NTIsImlhdCI6MTczMTAwNDI5MiwicGx0IjotMX0.57_sk2FlvqNcgGnS1MqwvnxMDgsuyMOuoK8G_877tyI; bili_ticket_expires=1731263492; bp_t_offset_3493293971212575=997152949270478848; SESSDATA=53e5b208%2C1746558195%2Cff7b7%2Ab1CjCGcAeb4cZzb2ElATwpwyVPl6h59tAfOJjSRcs9vZv_XpFomIYrT4lxNfLUl9KQb58SVnpqM0xDSUdGWjBRMkxxbUFlSzVZeFlmOWN3NlV4LUpEU0UtWG1XVUg0NEdEbzRUM2FoTmRhOVRONnRvVXJ6Z2ozODc4V2FYTUs4eU5JSTJYT1FEbUpBIIEC; bili_jct=0615911a3938adb8359aec204d2eaacf; DedeUserID=27489840; DedeUserID__ckMd5=115c002c23796687; sid=7io7llk6; buvid_fp=ca11c94267fa5b66c8a4f226281de73d; home_feed_column=5; browser_resolution=1912-920; Hm_lvt_8a6e55dbd2870f0f5bc9194cddf32a02=1730685184,1730731213,1730748905,1731127464; CURRENT_FNVAL=4048; b_lsid=D54AAFBB_193140D7133; bp_t_offset_27489840=998022770342232064; PVID=3"));
    // let mut bilibili_ws = bilibiliWs::new(live_id, cookie);
    // bilibili_ws.on("INTERACT_WORD", move |data| {
    //     let msg_type = data["msg_type"].as_u64().unwrap();
    //     if msg_type == 1 {
    //         // println!("{} 进入了直播间", data["user"].as_str().unwrap());
    //     } else if msg_type == 2 {
    //         println!("{} 关注了主播", data["user"].as_str().unwrap());
    //     }
    // });
    // bilibili_ws.start().await;

    start_minecraft_live().await;
    // let ws_url = "wss://webcast5-ws-web-hl.douyin.com/webcast/im/push/v2/?app_name=douyin_web&version_code=180800&webcast_sdk_version=1.0.14-beta.0&update_version_code=1.0.14-beta.0&compress=gzip&device_platform=web&cookie_enabled=true&screen_width=1920&screen_height=1080&browser_language=zh-CN&browser_platform=Win32&browser_name=Mozilla&browser_version=5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/130.0.0.0%20Safari/537.36%20Edg/130.0.0.0&browser_online=true&tz_name=Asia/Shanghai&cursor=t-1731208260251_r-1_d-1_u-1_fh-7435482025014170651&internal_ext=internal_src:dim|wss_push_room_id:7435441451519232783|wss_push_did:7349101092921476660|first_req_ms:1731208260149|fetch_time:1731208260251|seq:1|wss_info:0-1731208260251-0-0|wrds_v:7435482859265067035&host=https://live.douyin.com&aid=6383&live_id=1&did_rule=3&endpoint=live_pc&support_wrds=1&user_unique_id=7349101092921476660&im_path=/webcast/im/fetch/&identity=audience&need_persist_msg_count=15&insert_task_id=&live_reason=&room_id=7435441451519232783&heartbeatDuration=0&signature=6DVWraXJM4KhZ8y2";
    // let mut douyin_ws = douyinWs::new(ws_url);

    // douyin_ws.on("WebcastGiftMessage", move |data| {
    //     let count = data["count"].as_u64().unwrap();
    //     // println!("{:?}", data);
    //     let gift_id = data["gift_id"].as_u64().unwrap();
    //     println!(
    //         "收到来自 {} 的礼物: {}",
    //         data["user"].as_str().unwrap(),
    //         gift_id
    //     );
    // });
    // douyin_ws.start().await;
}
