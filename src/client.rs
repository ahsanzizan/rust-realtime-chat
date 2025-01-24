use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

use colored::Colorize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🌐 Connecting to Chat Server...".green().bold());

    let addr: &str = "127.0.0.1";
    let port: i32 = 6000;

    let mut client: TcpStream =
        TcpStream::connect(format!("{}:{}", addr, port)).expect("TCP Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        println!("{}", "💬 Enter your messages (type ':quit' to exit)".cyan());

        loop {
            let mut buff: String = String::new();
            io::stdin()
                .read_line(&mut buff)
                .expect("Failed to read from stdin");

            let msg: String = buff.trim().to_string();
            if msg == ":quit" {
                println!("{}", "👋 Exiting chat...".yellow());
                break;
            }

            if tx.send(msg).is_err() {
                break;
            }
        }
    });

    println!("{}", "✅ Connected to server!".green());

    loop {
        let mut buff: Vec<u8> = vec![0; 1024];

        match client.read(&mut buff) {
            Ok(n) => {
                if n == 0 {
                    println!("{}", "🔌 Server closed the connection".red());
                    break;
                }

                print!("{}", String::from_utf8_lossy(&buff[..n]).green());
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("{}", "❌ Connection with server was severed".red());
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff: Vec<u8> = msg.clone().into_bytes();
                buff.push(b'\n');

                if let Err(e) = client.write_all(&buff) {
                    eprintln!("{} {}", "❌ Failed to send your message:".red(), e);
                    break;
                }
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
