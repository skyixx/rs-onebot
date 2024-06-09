use async_std::net::TcpStream;
use async_tungstenite::{async_std::connect_async, tungstenite::Message::Text, WebSocketStream};
use base64::{engine::general_purpose::STANDARD as base64, Engine};
use futures::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    action: String,
    params: HashMap<String, String>,
    echo: String,
}

struct _RecMessage {}
pub struct Bot {
    _addr: String,
    socket: WebSocketStream<TcpStream>,
}

impl Bot {
    /// Create a new bot object
    /// `addr: websocket server address`
    /// `qid`: your bot qq id
    pub async fn new(addr: &str) -> Bot {
        let (socket, _) = connect_async(addr).await.unwrap();
        Bot {
            _addr: addr.to_string(),
            socket: socket,
        }
    }

    async fn send(&mut self, m: Message) {
        let ws = &mut self.socket;

        let item = serde_json::to_string(&m).unwrap();
        ws.send(Text(item)).await.unwrap();
    }

    async fn send_msg(&mut self, message: &str, id: u32, r#type: &str) {
        let params = HashMap::from([
            (format!("{}_id", r#type).to_string(), id.to_string()),
            ("message".to_string(), message.to_string()),
        ]);

        let obj = if r#type == "user" {
            "private"
        } else if r#type == "group" {
            "group"
        } else {
            panic!("Unkown Sender Tpye")
        };

        let m = Message {
            action: format!("send_{}_msg", obj),
            params,
            echo: "".to_string(),
        };
        let _ = self.send(m).await;
    }

    /// 发送私聊消息
    ///
    pub async fn send_private_msg(&mut self, message: &str, user: u32) {
        self.send_msg(message, user, "user").await;
    }

    /// 发送群聊消息
    ///
    pub async fn send_group_msg(&mut self, message: &str, group: u32) {
        self.send_msg(message, group, "group").await;
    }
    pub async fn send_image(&mut self, user: u32, image: &[u8], target: &str) {
        let message = format!(r#"[CQ:image,file=base64://{}]"#, base64.encode(image));
        let message = message.as_str();
        match target {
            "group" => self.send_group_msg(message, user).await,
            "user" => self.send_private_msg(message, user).await,
            _ => panic!("Unkown target type"),
        }
    }
    pub async fn rec_msg(&mut self) -> Option<String> {
        let ws = &mut self.socket;
        let msg = ws.next().await.unwrap().ok();
        match msg {
            Some(m) => match m {
                Text(t) => Some(t),
                _ => None
            }
            None => None
        }
    }
}

