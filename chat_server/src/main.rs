use std::{eprintln, error::Error, net::SocketAddr, println, vec};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _rx) = broadcast::channel::<String>(100);

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
    _addr: SocketAddr,
    tx: Sender<String>,
    rx: &mut Receiver<String>,
) {
    let (mut reader, mut writer) = socket.split();

    let mut buf = vec![0; 1024];

    loop {
        tokio::select! {
            result = reader.read(&mut buf) => {
                match result {
                    Ok(0) => break,
                    Ok(n) => {
                        let msg = String::from_utf8_lossy(&buf[..n]) ;
                        println!("{}", msg);
                        tx.send(msg.to_string()).unwrap();
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
                        writer.write_all(msg.as_bytes()).await.unwrap();
                        // writer.write_all(b"\n").await.unwrap();
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
