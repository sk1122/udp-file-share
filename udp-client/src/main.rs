use std::net::UdpSocket;
use std::str;
use std::io::Result;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    filename: std::path::PathBuf,
    hostname: String
}

// fn 

fn main() -> Result<()> {
    let cli = Cli::parse();
    let file = cli.filename;
    let contents = std::fs::read(&file).unwrap();
    let split_filename: Vec<&str> = file.file_name().unwrap().to_str().unwrap().split(".").collect();
    
    let filetype = match split_filename[split_filename.len() - 1] {
        "png" => 1,
        "svg" => 2,
        _ => 3
    };
    let filename = file.file_name().unwrap().to_str().unwrap().as_bytes().to_vec();
    println!("{:?}", filename);
    let hostname = cli.hostname;
    let socket = UdpSocket::bind("[::]:0")?;

    let mut total = 0;
    let mut copy_number = 1;

    while total < contents.len() {
        let mut end = total + 2048;
        println!("{}  {}", total, (end - (filename.len() + 3)));
        
        if end >= contents.len() {
            end = contents.len();
        }
        
        let packet: Vec<u8> = [vec![filetype, copy_number], filename.to_vec(), vec![0], contents[total..end].to_vec()].concat();
        // println!("{:?}", packet);
        socket.send_to(&packet, hostname.to_string() + &":2000")
                .expect("Error on send");

        total = (total + 2048) - (filename.len() + 3);
        println!("{} => {}, {}", total, contents.len(), filename.len());
        copy_number += 1;
    }

    let mut buf = [0; 2048];
    let (amt, _src) = socket.recv_from(&mut buf)?;
    
    let echo = str::from_utf8(&buf[..amt]).unwrap();

    println!("Echo {}", echo);

    Ok(())
}