use std::{io::Read, net::{TcpListener, TcpStream}, thread};


pub fn server(port: usize) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => panic!("{:?}",e),
    };

    let mut handles = Vec::new();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => handles.push(thread::spawn(move || handle_connection(s))),
            Err(e) => println!("{e:?}"),
        }
    }

    for handle in handles {
        match handle.join() {
            Err(e) => println!("{:?}", e),
            _ => (),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf: [u8; 1024] = [0; 1024];
    stream.read(&mut buf);
    let s = convert_buf_to_string(&buf);
    println!("{s}");
}

fn convert_buf_to_string(buff: &[u8]) -> String {
    let mut constructed_string = String::new();
    for byte in buff {
        constructed_string = format!("{}{}", constructed_string, *byte as char);
    }
    constructed_string
}
