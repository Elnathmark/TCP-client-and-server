use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::str::from_utf8;

fn main() {

    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                        println!("Please Send A Message To The Server And wait The Server Reply The Same Messag To You");
                        let mut stream = TcpStream::connect("127.0.0.1:3333").expect("Could not connect to server");
                        loop {
                            let mut input = String::new();
                            let mut buffer: Vec<u8> = Vec::new();
                            io::stdin().read_line(&mut input).expect("Failed to read from stdin");
                            stream.write(input.as_bytes()).expect("Faild to write to server");
                            let mut reader = BufReader::new(&stream);
                            reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
                            print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as tring"));
                        }

                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}