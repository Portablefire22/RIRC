use std::{io::{self, Read, Write}, net::{TcpListener, TcpStream}, thread};


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
    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        let _ =stream.read(&mut buf);
        if buf[0] == 0 {
            break;
        }
        let s = convert_buf_to_string(&buf);
        print!("{s}");
        let _ = io::stdout().flush();
        let _ = stream.write(&buf);
    }
}

fn convert_buf_to_string(buff: &[u8]) -> String {
    let mut constructed_string = String::new();
    // println!("{:?}", buff);
    for byte in buff {
        constructed_string = format!("{}{}", constructed_string, *byte as char);
    }
    constructed_string
}
