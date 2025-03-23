use std::io::{self, Write};
use std::net::TcpStream;

fn send_frame(stream: &mut TcpStream, frame: &[u8]) {
    stream.write_all(frame).unwrap();
}
