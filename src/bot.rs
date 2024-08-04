use async_std::net::TcpStream;
use async_tungstenite::{async_std::connect_async, tungstenite::Message::Text, WebSocketStream};
use base64::{engine::general_purpose::STANDARD as base64, Engine};
use core::str;
use futures::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Message {
    action: String,
    params: HashMap<String, String>,
    echo: String,
}

#[derive(Serialize, Deserialize)]
pub enum PostType {
    message,
    mequest,
    notice,
    meta_event,
}
pub mod message {
    use super::PostType;
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize)]
    pub enum MessageType {
        private,
        group,
    }

    #[derive(Serialize, Deserialize)]
    pub enum Sex {
        male,
        famale,
        unknown,
    }
    pub mod private {
        use serde::{Deserialize, Serialize};
        #[derive(Serialize, Deserialize)]
        pub struct Sender {
            user_id: u64,
            nickname: String,
            sex: Option<super::Sex>,
            age: Option<i32>,
            card: Option<String>,
        }
        #[derive(Serialize, Deserialize)]
        pub struct Message {
            time: u64,
            self_id: u64,
            post_type: super::PostType,
            message_type: super::MessageType,
            sub_type: SubType,
            message_id: i64,
            // message: crate::bot::Message,
            user_id: u64,
            raw_message: String,
            font: i32,
            sender: Sender,
        }
        #[derive(Serialize, Deserialize)]
        pub enum SubType {
            friend,
            group,
            other,
        }
        impl Message {
            pub fn from(msg: &str) -> Option<Self> {
                let m = serde_json::from_str(msg);
                match m {
                    Ok(m) => Some(m),
                    Err(_e) => {
                        println!("{_e}");
                        None
                    }
                }
            }
            pub fn printmsg(&self) -> () {
                use colored::*;
                let text = format!("{}:{}", "解析文本".blue(), self.raw_message.red());
                println!("{text}");
            }
            pub fn get_msg(&self) -> &String {
                &self.raw_message
            }
        }
    }
    pub mod group {
        use serde::{Deserialize, Serialize};
        #[derive(Serialize, Deserialize)]
        pub struct Message {
            time: u64,
            self_id: u64,
            post_type: super::PostType,
            message_type: super::MessageType,
            sub_type: SubType,
            message_id: u32,
            group_id: u64,
            user_id: u64,
            anoymous: Option<Anonymous>,
            raw_message: String,
            font: i32,
            sender: Sender,
        }
        #[derive(Serialize, Deserialize)]
        pub enum SubType {
            Normal,
            Anoymous,
            Notice,
        }
        #[derive(Serialize, Deserialize)]
        pub struct Anonymous {
            id: u64,
            name: String,
            flag: String,
        }
        #[derive(Serialize, Deserialize)]
        pub struct Sender {
            user_id: u64,
            nickname: String,
            card: String, // 群名片，备注
            sex: super::Sex,
            age: i32,
            area: String,  // 地区
            level: String, // 成员等级
            role: Role,    // 角色
            title: String, // 群头衔
        }
        #[derive(Serialize, Deserialize)]
        pub enum Role {
            Owner,
            Admin,
            Member,
        }
        impl Message {
            pub fn from(msg: &str) -> Option<Self> {
                let m = serde_json::from_str(msg);
                match m {
                    Ok(m) => Some(m),
                    Err(_e) => None,
                }
            }
            pub fn printmsg(&self) -> () {
                println!("解析文本：{:#?}", self.raw_message);
            }
        }
    }
}

pub struct Bot {
    _addr: String,
    api_socket: WebSocketStream<TcpStream>,
    event_socket: WebSocketStream<TcpStream>,
}

impl Bot {
    /// Create a new bot object
    /// `addr: websocket server address`
    pub async fn new(addr: &str) -> Bot {
        let _addr = if (&addr).chars().last() != Some('/') {
            addr.to_owned() + "/"
        } else {
            addr.to_owned()
        };
        println!("{}", _addr);
        let (api_socket, _) = connect_async(format!("{}api", &_addr)).await.unwrap();
        let (event_socket, _) = connect_async(format!("{}event", &_addr)).await.unwrap();
        Bot {
            _addr: addr.to_string(),
            api_socket,
            event_socket,
        }
    }

    async fn send(&mut self, m: Message) {
        let ws = &mut self.api_socket;

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
    pub async fn rec_msg(&mut self) -> Option<message::private::Message> {
        let ws = &mut self.event_socket;
        let msg = ws.next().await.unwrap().ok();
        match msg {
            Some(m) => match m {
                Text(t) => {
                    let tm = message::private::Message::from((&t).as_str());
                    Some(tm?)
                }
                _ => None,
            },
            None => None,
        }
    }
}
