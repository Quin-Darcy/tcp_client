use std::str;
use std::fs::File;
use std::net::TcpStream;
use std::io::{BufReader, Read, Write};

pub fn start(file_path: &str, socket: &str) {
    let mut message_bytes: Vec<u8> = Vec::new();
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_e) => panic!("Error. Could not open{}", file_path),
    };

    let mut reader = BufReader::new(file);

    let _num_of_bytes: usize = match reader.read_to_end(&mut message_bytes) {
        Ok(nob) => nob,
        Err(_e) => panic!("Error. Could not read bytes from {}", file_path),
    };

    let mut stream = TcpStream::connect(socket)
                                .expect("Could not connect to server");
    message_bytes.push(b'\0');
    stream.write(&message_bytes[..])
        .expect("Failed to write to server");
}
