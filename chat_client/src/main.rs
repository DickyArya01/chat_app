use clap::Parser;
use std::{eprintln, error::Error, println, vec};

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, stdin},
    net::TcpStream,
};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    username: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    let (mut reader, mut writer) = stream.split();

    let args = Args::parse();

    let username = args.username;

    writer.write_all(username.as_bytes()).await?;

    let mut stdin_lines = BufReader::new(stdin()).lines();
    let mut buf = vec![0; 1024];

    loop {
        tokio::select! {
            result = reader.read(&mut buf)=> {
                match result {
                    Ok(0) => break,
                    Ok(n) => {
                        println!("{}", String::from_utf8_lossy(&buf[..n]))
                    },
                    Err(err) => {
                        eprintln!("Failed reading buf: {}", err);
                        break;
                    }
                }
            }
            result = stdin_lines.next_line() => {
                match result? {
                    Some(line) => {
                        writer.write_all(line.as_bytes()).await?;
                        // writer.write_all(b"\n").await?;
                    }
                    None => { break; }

                }
            }
        }
    }

    Ok(())
}
