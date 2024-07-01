use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
extern crate rand;

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);
    let stdin = std::io::stdin();
    let mut input = String::new();
    let mut response = String::new();
    let r;
    if rand::random() {
        r = 1;
    } else {
        r = 2;
    }
    writer.write_all(format!("{}\n", r).as_bytes())?;
    writer.flush()?;
   
    loop {

        if r == 1 {
            println!("Ваш ход");
            input.clear();
            stdin.read_line(&mut input)?;
            writer.write_all(input.as_bytes())?;
            writer.flush()?;
            println!("Ждите хода соперника");
            response.clear();
            reader.read_line(&mut response)?;
            if response.is_empty() {
                break;
            }
            println!("Client: {}", response.trim());
        }
        if r == 2 {
            println!("Ждите хода соперника");
            response.clear();
            reader.read_line(&mut response)?;
            if response.is_empty() {
                break;
            }
            println!("Client: {}", response.trim());
            println!("Ваш ход");
            input.clear();
            stdin.read_line(&mut input)?;
            writer.write_all(input.as_bytes())?;
            writer.flush()?;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4545")?;
    println!("Server");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("client connected");
                if let Err(e) = handle_client(stream) {
                    println!("Error handling client: {:?}", e);
                }
            }
            Err(e) => println!("error connection {:?}", e),
        }
    }

    Ok(())
}
