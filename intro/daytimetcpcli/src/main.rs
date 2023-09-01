use std::{io::Read, process::exit};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: daytimetcpcli <IPAddress>");
        exit(0)
    }

    let server_addr = args[1].trim();
    let server_port = 8013;
    let mut buf = [0u8; 4096];

    let mut stream = std::net::TcpStream::connect((server_addr, server_port))
        .ok()
        .expect("connect server error");

    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => println!("{}", std::str::from_utf8(&mut buf[0..n]).unwrap()),
            Err(e) => {
                eprintln!("read error {}", e);
                panic!();
            }
        }
    }
}
