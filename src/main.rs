use std::{env, net::IpAddr};

use client::client;
use server::server;

mod client;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some (a1) => {
            if a1 == "server" {
                match args.get(2) {
                    Some(a2) => match a2.parse::<usize>() {
                        Ok(port) => server(port),
                        Err(_) => display_usage(),
                    },
                    None => {
                        display_usage();
                        println!("Defaulting to port 6662");
                        server(6662);
                    },
                }
            } else if a1 == "client" {
                match args.get(2) {
                    Some (a2) => match a2.parse::<IpAddr>() {
                        Ok(addr) => {
                            match args.get(3) {
                                Some(a3) => match a3.parse::<usize>() {
                                    Ok(port) => client(addr, port),
                                    Err(_) => display_usage(),
                                },
                                None => display_usage(),
                            }
                        },
                        Err(e) => {
                            println!("{e:?}");
                            display_usage()
                        },
                    },
                    None => display_usage(),
                }
            } else {
                display_usage()
            }
        },
        None => display_usage(),
    }
}

fn display_usage() {
    println!("Usage: rirc [client | server] [addr (client)] [port]  [port_number]");
}
