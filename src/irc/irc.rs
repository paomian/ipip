use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::File;
use std::fs::OpenOptions;
use time;

use bufstream::BufStream;

fn now() -> String {
    match time::strftime("%Y-%m-%d %H:%M:%S",&time::now()) {
        Ok(s) => s,
        Err(_) => String::from("0000-00-00 00:00:00"),
    }
}

fn open_file(log_name:&str) -> Option<File> {
    match OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(log_name) {
            Ok(f) => Some(f),
            Err(e) => {
                error!("Open file error:{:?}",e);
                None
            }
        }
}

fn chat_log(s:&str,f:&mut Option<File>, who:&str,to:Option<&str>) {
    let mut new_msg = String::from(s);
    if s.chars().last() == Some('\n') {
        let _ = new_msg.pop();
        let _ = new_msg.pop();
    }
    if let Some(t) = to {
        new_msg = [&now()[..]," ", who, " Say:[", &new_msg[..],"] to ",t,"\n"]
            .concat();
    } else {
        new_msg = [&now()[..]," ", &who[..], " Say:[", &new_msg[..],"]\n"]
            .concat();
    }

    if let &mut Some(ref mut file) = f {
        match file.write(new_msg.as_bytes()) {
            Ok(_) => {},
            Err(e) => error!("{:?}",e),
        }
    } else {
        error!("write file faild {}",new_msg);
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
    let mut log_file = open_file("chat.log");
    loop {
        let _ = buf.read_line(&mut data);
        info!("Get data {}",&data[..data.len()-1]);
        if data.starts_with("PING") {
            let _ = buf.write(&["PONG ", &data[5..]].concat().into_bytes());
            let _ = buf.flush();
        } else {
            let tmp:Vec<&str> = data.splitn(4, ':').collect();
            let who = tmp[1].splitn(2, '!').collect::<Vec<&str>>()[0];
            let mut msg = "";
            if tmp.len() >= 4 {
                msg = &tmp[3];
                if tmp[2] == "BBit" {
                    let commond = ["PRIVMSG #sdut :", who, ":你说的是：",msg].concat();
                    let _ = buf.write(&commond.into_bytes());
                    let _ = buf.flush();
                } else {
                    chat_log(msg,&mut log_file,who,Some(tmp[2]));
                }
            } else if tmp.len() == 3 {
                let msg = &tmp[2];
                chat_log(msg,&mut log_file,who,None);
            }
        }
        data = String::from("");
    }
}
