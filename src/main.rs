mod bot;
use async_std::{task, fs};

async fn async_main() {
    let img = fs::read("./code.png").await.unwrap();
    let img = img.as_slice();
    let mut b = bot::Bot::new("ws://192.168.2.103:3001/event").await;
    println!("Connected");
    // b.send_image(123456, img, "user").await;
    // println!("Sended");
    while  let m = b.rec_msg().await {
        println!("{:#?}", m);
    }
}

fn main() {
    let f = async_main();
    task::block_on(f);
}

