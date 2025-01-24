use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use colored::Colorize;

fn main() {
    println!("{}", "🚀 Starting The Server...".green().bold());

    let addr: &str = "127.0.0.1";
    let port: i32 = 6000;

    let server: TcpListener =
        TcpListener::bind(format!("{}:{}", addr, port)).expect("Failed to bind server to ADDR");
    server
        .set_nonblocking(true)
        .expect("Failed to set the non-blocking property!");

    println!("{}", "📡 Listening on 127.0.0.1:6000".cyan().italic());

    let mut clients: Vec<TcpStream> = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!(
                "{} {}",
                "✅ New Client connected:".green(),
                addr.to_string().yellow()
            );

            let tx: Sender<String> = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || {
                let mut buffer = [0; 1024];

                loop {
                    match socket.read(&mut buffer) {
                        Ok(n) if n == 0 => {
                            println!("{} {}", "🔌 Client has disconnected:".red(), addr);
                            return;
                        }
                        Ok(n) => {
                            let msg = String::from_utf8_lossy(&buffer[..n]);
                            tx.send(format!("Client {}: {}", addr, msg))
                                .expect("Failed to send message to channel");
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(e) => {
                            eprintln!("{} {}", "❌ Connection error:".red(), e);
                            return;
                        }
                    }

                    thread::sleep(Duration::from_millis(100));
                }
            });
        }

        if let Ok(msg) = rx.try_recv() {
            let broadcast_msg = format!("{} {}", "📢 Broadcast:".yellow(), msg);

            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff: Vec<u8> = broadcast_msg.clone().into_bytes();
                    buff.push(b'\n');

                    return client.write_all(&buff).map(|_| client).ok();
                })
                .collect::<Vec<_>>();
        }
    }
}
