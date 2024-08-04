use reqwest;

async fn get_bing_img() {
    let mut clinet = reqwest::get(
        "https://s.cn.bing.net/th?id=OHR.ImpalaOxpecker_ZH-CN9652434873_1920x1080.webp&qlt=50",
    )
    .await
    .unwrap()
    .bytes()
    .await
    .unwrap();
}
