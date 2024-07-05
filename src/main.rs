use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
extern crate rand;

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);
    let stdin = std::io::stdin();
    let mut input = String::new();
    let mut response = String::new();
    let mut array: [Vec<i16>; 3] = [vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
    let r;



    if rand::random() {
        r = 1;
    } else {
        r = 2;
    }
    writer.write_all(format!("{}\n", r).as_bytes())?;
    writer.flush()?;
    print_array(&array);
    loop {
        if r == 1 {
            println!("Ваш ход");
            input = write_data(&stdin, &mut writer)?;
            println!("Ждите хода соперника");
            response = read_data(&mut reader)?;
            println!("Client: {}", response.trim());
        }

        if r == 2 {
            println!("Ждите хода соперника");
            response = read_data(&mut reader)?;
            println!("Client: {}", response.trim());
            println!("Ваш ход");
            input = write_data(&stdin, &mut writer)?;
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
fn print_array(array: &[Vec<i16>; 3]) {
    for row in array.iter() {
        for val in row.iter() {
            print!("{} ", val);
        }
        println!();
    }
}
fn read_data(reader: &mut BufReader<TcpStream>) -> std::io::Result<String> {
    let mut response = String::new();
    reader.read_line(&mut response)?;
    Ok(response)
}

fn write_data(stdin: &io::Stdin, writer: &mut BufWriter<TcpStream>) -> std::io::Result<String> {
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    writer.write_all(input.as_bytes())?;
    writer.flush()?;
    Ok(input.trim().to_string())
}
