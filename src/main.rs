use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

 
fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);
    let stdin = std::io::stdin();
    let mut input = String::new();
    let mut response = String::new();

    loop {
 
        response.clear();
        reader.read_line(&mut response)?;
        if response.is_empty() {
            break;
        }
        
        println!("Client: {}", response.trim());

        input.clear();
        stdin.read_line(&mut input)?;
        writer.write_all(input.as_bytes())?;
        writer.flush()?;
       
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
