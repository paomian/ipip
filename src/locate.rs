use std::io::Read;

use hyper;
use hyper::Client;
use hyper::header::Connection;

pub fn locate(s:&str) -> String {

    let client = Client::new();

    let mut body = String::new();
    let url = ["http://freeapi.ipip.net/", s].concat();
    let _ = client.get(&url)
        .header(Connection::keep_alive())
        .send().map(|mut res|{
            println!("Response: {}", res.status);
            match res.status {
                hyper::Ok => {res.read_to_string(&mut body);},
                _ => body = String::from("Oh"),
            }
        });
    return body;
}
