use std::io::Read;

use hyper;
use hyper::Client;
use hyper::header::Connection;

pub fn locate(s:&str) -> String {

    let client = Client::new();

    let mut body:Vec<u8> = Vec::new();
    let url = ["http://freeapi.ipip.net/", s].concat();
    let _ = client.get(&url)
        .header(Connection::keep_alive())
        .send().map(|mut res|{
            match res.status {
                hyper::Ok => {let _ = res.read_to_end(&mut body);},
                _ => body = String::from("Oh").into_bytes(),
            }
        });
    let _ = match String::from_utf8(body) {
        Ok(o) => return o,
        Err(e) => {
            println!("{:?}",e);
            return String::from("Oh");
        },
    };
}
