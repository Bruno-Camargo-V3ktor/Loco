use std::{
    io::{self, Read},
    net::{IpAddr, TcpListener},
};

use loco_h2::{connection::PREFACE, frame::Frame};

pub struct Server {
    addr: IpAddr,
    port: u16,
}

impl Server {
    pub fn new(addr: &str, port: u16) -> Self {
        Self {
            addr: addr.parse().expect("Not Address IP Valid"),
            port,
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let listener = TcpListener::bind(format!("{:?}:{}", self.addr, self.port))?;

        for stream in listener.incoming() {
            let mut stream = stream?;

            let mut preface = [0; 24];
            stream.read_exact(&mut preface)?;

            println!("PREFACE: {preface:?}");

            if preface == *PREFACE {
                let mut header_bytes = [0; 9];
                stream.read_exact(&mut header_bytes)?;

                let length =
                    u32::from_be_bytes([0, header_bytes[0], header_bytes[1], header_bytes[2]]);
                let mut payload_bytes = vec![0; length as usize];
                stream.read_exact(&mut payload_bytes)?;

                let mut total = header_bytes.to_vec();
                total.append(&mut payload_bytes.to_vec());

                let frame_parse = Frame::new(&total);
                println!("{frame_parse:?}");
            }
        }

        Ok(())
    }
}
