use std::{eprintln, error::Error, format, net::SocketAddr, vec};

use chat_common::chat_message::{ChatMessage, UserIdTag, MessageType};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _rx) = broadcast::channel::<ChatMessage>(100);

    loop {
        let (mut socket, addr) = listener.accept().await?;

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            handle_client(&mut socket, addr, tx, &mut rx).await;
        });
    }
}

async fn handle_client(
    socket: &mut TcpStream,
    addr: SocketAddr,
    tx: Sender<ChatMessage>,
    rx: &mut Receiver<ChatMessage>,
) {
    let (mut reader, mut writer) = socket.split();

    let mut buf = vec![0; 1024];

    let n = reader.read(&mut buf).await.unwrap();
    let username = String::from_utf8_lossy(&buf[..n]).to_string();

    let user_id = UserIdTag::new(&addr).hash();

    let enter_chat_notif_msg = ChatMessage{
        id: user_id,
        username: "".to_string(),
        content: format!("{} enter the chat", username),
        message_type: MessageType::SystemNotification
    };

    writer.write_all(enter_chat_notif_msg.to_chat_str().as_bytes()).await.unwrap();
    
    tx.send(enter_chat_notif_msg).unwrap();

    loop {
        tokio::select! {
            result = reader.read(&mut buf) => {
                match result {
                    Ok(0) => {
                        let msg_content = format!("{} has leave the chat", username.clone());
                        let notif_msg = ChatMessage {
                            id: 0,
                            username: String::from("system"),
                            content: msg_content,
                            message_type: MessageType::SystemNotification
                        };
                        tx.send(notif_msg).unwrap();

                        break;
                    },
                    Ok(n) => {
                        let sent_msg = String::from_utf8_lossy(&buf[..n]).to_string() ;

                        let msg = ChatMessage {
                            id: user_id.clone(),
                            username: username.clone(),
                            content: sent_msg,
                            message_type: MessageType::UserMessage
                        };

                        tx.send(msg).unwrap();
                    },
                    Err(err) => {
                        eprintln!("Failed reading buf {}", err);
                        break;
                    }
                }
            },
            result = rx.recv() => {

                match result {
                    Ok(msg) => {

                        if msg.id == user_id {
                            continue;
                        }

                        writer.write_all(msg.to_chat_str().as_bytes()).await.unwrap();
                    },
                    Err(err) => {
                        eprintln!("Err receive: {}", err);
                        break;
                    }
                }
            }
        }
    }
}
