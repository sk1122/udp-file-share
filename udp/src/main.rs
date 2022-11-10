use std::net::UdpSocket;
use std::io::{
    Result,
};
use std::str;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> Result<()> {
    // let contents = fs::read_to_string("image.txt")?;
    // println!("{}", contents);

    // let img = image::open("profile.png").expect("file not found");
    // let (w, h) = img.dimensions();
    // let mut output = ImageBuffer::new(w, h);

    // for (x, y, pixels) in img.pixels() {
    //     output.put_pixel(x, y, pixels.map(|p| p.saturating_sub(65)));
    // }

    // println!("{:?}", img.as_bytes());

    // Ok(())

    let socket = UdpSocket::bind("[::]:2000")?;
    let mut buf = [0; 2048];

    loop {
        let (_, _) = socket.recv_from(&mut buf)?;
        // println!("{:?}", &buf[..amt]);
        let img_type = &buf[0];
        // let index = &buf[1];
        
        println!("buf received {:?}", &buf);
        
        let mut end = 3;
        for (idx, val) in buf.iter().enumerate() {
            if *val == 0 {
                end = idx;
                break;
            }
        }
        println!("{} {}", end, &buf[end]);
        let image_name = str::from_utf8(&buf[2..end]).unwrap();
        // let echo = str::from_utf8(&buf[1..]).unwrap();
        // println!("{}", echo);

        if img_type == &u8::from(1) {
            match image::load_from_memory_with_format(&buf[(end + 1)..], image::ImageFormat::Png) {
                Ok(_) => {
                    std::fs::write(image_name, &buf[(end + 1)..]).unwrap();
                },
                Err(err) => {
                    println!("failed {:?}", err);
                }
            }
        } else if img_type == &u8::from(2) {
            let file = OpenOptions::new().create(true).append(true).write(true).read(true).open(image_name).unwrap();
            let echo = str::from_utf8(&buf[(end + 1)..]).unwrap();
            if let Err(e) = writeln!(&file, "{echo}") {
                eprintln!("Couldn't write to file: {}", e);
            }
        }


        // if let Err(e) = writeln!(file, "{echo}") {
        //     eprintln!("Couldn't write to file: {}", e);
        // }


        // let buf = "rececived";
        // socket.send_to(buf.as_bytes(), &src)?;
    }
}