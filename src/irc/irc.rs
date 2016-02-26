use std::io::prelude::*;
use std::net::TcpStream;

use bufstream::BufStream;

pub fn irc_bot() {
    info!("Starting IRC Server");
    let server = "irc.freenode.net:6667";
    let nick_name = String::from("NICK BBit\r\n").into_bytes();
    let gecos = String::from("USER Bot * 8 :BBit\r\n").into_bytes();
    let channel = String::from("JOIN #sdut\r\n").into_bytes();
    let mut stream = match TcpStream::connect(server) {
        Ok(s) => s,
        Err(_) => panic!("Soket error")
    };
    let mut buf = BufStream::new(stream);
    let _ = buf.write(&nick_name);
    let _ = buf.write(&gecos);
    let _ = buf.write(&channel);
    let _ = buf.flush();
    let mut data = String::new();
    loop {
        let _ = buf.read_line(&mut data);
        if data.starts_with("PING") {
            let _ = buf.write(&["PONG ", &data[5..]].concat().into_bytes());
            let _ = buf.flush();
        } else {
            let tmp:Vec<&str> = data.split(':').collect();
            if tmp.len() == 4 && tmp[2] == "BBit" {
                let msg = &tmp[3][1..];
                let who = tmp[1].splitn(2, '!').collect::<Vec<&str>>()[0];
                let commond = ["PRIVMSG #sdut :", who, ":你说的是：",msg].concat();
                let _ = buf.write(&commond.into_bytes());
                let _ = buf.flush();
            }
        }
        info!("Get data {}",&data[..data.len()-1]);
        data = String::from("");
    }
}
