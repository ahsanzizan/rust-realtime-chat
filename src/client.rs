use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let addr: &str = "127.0.0.1";
    let port: i32 = 6000;

    let mut client: TcpStream =
        TcpStream::connect(format!("{}:{}", addr, port)).expect("TCP Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff: String = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Failed to read from stdin");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    });

    println!("Connected to server!");
    println!("Type your message (or ':quit' to exit)");

    loop {
        let mut buff: Vec<u8> = vec![0; 1024];

        match client.read(&mut buff) {
            Ok(n) => {
                if n == 0 {
                    println!("Server has closed the connection.");
                    break;
                }
                println!("{}", String::from_utf8_lossy(&buff[..n]));
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server was severed.");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff: Vec<u8> = msg.clone().into_bytes();
                buff.push(b'\n');
                client.write_all(&buff).expect("Writing to socket failed");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    }
}
