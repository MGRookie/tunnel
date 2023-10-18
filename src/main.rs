use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    // 客户端关闭了连接
                    break;
                }
                let received_data = String::from_utf8_lossy(&buffer[0..n]);
                println!("Received: {}", received_data);
            }
            Err(e) => {
                eprintln!("Error while receiving data: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let ip = env::args().skip(1).nth(0).expect("ip is error");
    let listener = TcpListener::bind(ip.as_str()).expect("Failed to bind");
    println!("Server listening on {}", ip);

    // 启动一个线程来处理客户端连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 启动一个新线程来处理连接
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    // 在主线程中处理用户输入并发送数据
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed_input = input.trim();

        if !trimmed_input.is_empty() {
            let mut client = TcpStream::connect(ip.as_str()).expect("Failed to connect");
            client.write_all(trimmed_input.as_bytes()).expect("Failed to send data");
        }
    }
}
