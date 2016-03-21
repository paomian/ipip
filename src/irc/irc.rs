use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::File;

use bufstream::BufStream;

fn now() -> String {
    match time::strftime("%Y-%m-%d %H:%M:%S",&time::now()) {
        Ok(s) => s,
        Err(_) => String::from("0000-00-00 00:00:00"),
    }
}

fn open_file(log_name:&str) -> Option<File> {
    let file:Option<File> = match File::open(log_name) {
        Ok(f) => Some(f),
        Err(e) => {
            error!("Open file error:{:?}",e);
            match File::create(log_name) {
                Ok(f) => Some(f),
                Err(e) => {
                    error!("Create file error:{:?}",e);
                    None
                }
            }
        }
    }
}

fn chat_log(s:&String,f:Option<File>, who:&String){
    let mut new_msg = String::new();
    if s.chars().last() != Some('\n') {
        new_msg = s.push('\n');
    }

    new_msg = [&now()," ", who, " Say:", &new_msg].concat();

    if let Some(f) = file {
        f.write(new_msg.as_bytes());
    } else {
        info!(new_msg);
    }

}

pub fn irc_bot() {
    info!("Starting IRC Server");
    let server = "irc.freenode.net:6667";
    let nick_name = String::from("NICK BBit\r\n").into_bytes();
    let gecos = String::from("USER Bot * 8 :BBit\r\n").into_bytes();
    let channel = String::from("JOIN #sdut\r\n").into_bytes();
    let stream = match TcpStream::connect(server) {
        Ok(s) => s,
        Err(_) => panic!("Soket error")
    };
    let mut buf = BufStream::new(stream);
    let _ = buf.write(&nick_name);
    let _ = buf.write(&gecos);
    let _ = buf.write(&channel);
    let _ = buf.flush();
    let mut data = String::new();
    let log = open_file("chat.log");
    loop {
        let _ = buf.read_line(&mut data);
        if data.starts_with("PING") {
            let _ = buf.write(&["PONG ", &data[5..]].concat().into_bytes());
            let _ = buf.flush();
        } else {
            let tmp:Vec<&str> = data.split(':').collect();
            let who = tmp[1].splitn(2, '!').collect::<Vec<&str>>()[0];
            if tmp.len() == 4 && tmp[2] == "BBit" {
                let msg = &tmp[3][1..];
                let commond = ["PRIVMSG #sdut :", who, ":你说的是：",msg].concat();
                let _ = buf.write(&commond.into_bytes());
                let _ = buf.flush();
            } else if tmp.len() == 3 {
                let msg = &tmp[3];

            }
        }
        info!("Get data {}",&data[..data.len()-1]);
        data = String::from("");
    }
}
