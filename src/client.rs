use std::{io::{self, Read, Write}, net::{IpAddr, TcpStream}, thread, u128};


pub fn client(addr: IpAddr, port: usize) {
    let addr = format!("{}:{}", addr, port);
    let mut stream = TcpStream::connect(&addr).unwrap();

    let recv_stream = stream.try_clone();
    thread::spawn(move || receive_msg(recv_stream.unwrap()));

    println!("Connected to: {addr}!");
    
    loop {
        let mut input = String::new();
        print!("Message to send: ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => println!("{e:?}"),
        }
        let _ = stream.write(input.as_bytes());
    }
}

fn receive_msg(mut stream: TcpStream) {
    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        let _ = stream.read(&mut buf);

        // Converts bytes to u128
        let (prefix, aligned, suffix) = unsafe {buf.align_to::<u128>()};
        if prefix.iter().all(|&x| x == 0) 
            && suffix.iter().all(|&x| x == 0)
            && aligned.iter().all(|&x| x == 0)
        {
            continue;
        }
        let s = convert_buf_to_string(&buf);
        println!("\nReceived: {s}");
    }
}

fn convert_buf_to_string(buff: &[u8]) -> String {
    let mut constructed_string = String::new();
    for byte in buff {
        constructed_string = format!("{}{}", constructed_string, *byte as char);
    }
    constructed_string
}
