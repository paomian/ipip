use std::io::Read;

use hyper;
use hyper::Client;
use hyper::header::{Connection,UserAgent};
use rustc_serialize::json::Json;

pub fn locate(s:&str) -> Json {

    let client = Client::new();

    let mut body:Vec<u8> = Vec::new();
    let url = ["http://freeapi.ipip.net/", s].concat();
    let _ = client.get(&url)
        .header(Connection::keep_alive())
        .header(UserAgent(String::from("Something")))
        .send().map(|mut res|{
            match res.status {
                hyper::Ok => {let _ = res.read_to_end(&mut body);},
                _ => {let _ = res.read_to_end(&mut body);},
            }
        });
    let _ = match String::from_utf8(body.clone()) {
        Ok(o) => {
            match Json::from_str(&o) {
                Ok(data) => return data,
                Err(e) => {
                    error!("Parse String to Json error {:?}:{:?}",e,&o);
                    return Json::String(String::from("Your IP is zhale!"));
                },
            }
        },
        Err(e) => {
            error!("Parse Vec to String error {:?}:{:?}",e,body);
            return Json::Null;
        },
    };
}
