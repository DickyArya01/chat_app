use chat_client::ui::ChatInterface;
use chat_client::ui::write_msg_view;
use clap::Parser;
use cursive::CbSink;
use cursive::Cursive;
use cursive::CursiveExt;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use std::{error::Error, vec};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, },
    net::TcpStream,
    runtime::Runtime,
};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    username: String,
}

fn main() -> Result<(), Box<dyn Error>>{
    let args = Args::parse();
    let rt = Runtime::new().unwrap();
        
    let username = args.username;

    let stream = rt.block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
        stream.write_all(username.as_bytes()).await?;
        Ok::<_, std::io::Error>(stream)
    })?;

    let (tx, rx) = mpsc::unbounded_channel::<String>();

    let mut ci = ChatInterface{}.init_chat(tx);
    let sink = ci.cb_sink().clone();

    rt.spawn(network_loop(stream, rx, sink));

    ci.run();

    Ok(())
}

async fn network_loop(mut stream: TcpStream, mut rx: UnboundedReceiver<String>, sink: CbSink) {
    let (mut reader, mut writer) = stream.split();
    let mut buf = vec![0; 1024];

    loop {
        tokio::select! {
            result = reader.read(&mut buf) => {
                match result {
                   Ok(0) => {
                    show(&sink, "Server closed the connection".to_string());
                    break;
                   },
                   Ok(n) => {
                    show(&sink, String::from_utf8_lossy(&buf[..n]).to_string());
                   },
                   Err(err) => {
                    show(&sink, format!("Error: {}", err));
                    break;
                   } 
                }
            }
            msg = rx.recv() => {
                match msg {
                    Some(msg) => {
                        if let Err(err) = writer.write_all(msg.as_bytes()).await {
                            show(&sink, format!("Error: {}", err));
                            break
                        }

                    }
                    None => break
                }
            }
        }
    }
}

fn show (sink: &CbSink, msg: String) {
    sink.send(Box::new(move |s: &mut Cursive| write_msg_view(s, &msg))).unwrap();
}

