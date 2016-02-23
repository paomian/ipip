use std::io::prelude::*;
use std::net::TcpStream;

pub fn irc_bot() {
    info!("Starting IRC Server");
    let server = "irc.freenode.net:6667";
    let nick_name = String::from("NICK BB8\r\n").into_bytes();
    let gecos = String::from("USER Hello Everyone\r\n").into_bytes();
    let channel = "#sdut";
    let mut stream = match TcpStream::connect(server) {
        Ok(s) => s,
        Err(_) => panic!("Soket error")
    };
    let _ = stream.write(&nick_name);
    let _ = stream.write(&gecos);
    let mut data = Vec::new();
    loop {
        let _ = stream.read_to_end(&mut data);
        match String::from_utf8(data.clone()) {
            Ok(s) => info!("Get data {}",s),
            Err(e) => error!("Get Data error:{:?} data:{:?}",e,&data),
        }
    }
}
