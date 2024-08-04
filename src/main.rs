mod bot;
pub mod utils;
use async_std::{fs, task};
mod modules;

async fn async_main() {
    //let img = fs::read("./code.png").await.unwrap();
    //let img = img.as_slice();
    let addr = "ws://192.168.2.103:3001";
    let mut b = bot::Bot::new(addr).await;
    println!("Connected");
    // b.send_image(123456, img, "user").await;
    // println!("Sended");
    while let m = b.rec_msg().await {
        let message;
        if let Some(_m) = m {
            message = _m;
            // b.send_private_msg("这啥啊", 3324298144).await
            use colored::*;
            let text = format!("{}:{}", "解析文本".blue(), message.get_msg().red());
            println!("{text}");
        }
    }
}

fn main() {
    let f = async_main();
    task::block_on(f);
}
