#![feature(plugin)]
#![plugin(clippy)]

extern crate iron;
extern crate time;
extern crate hyper;
extern crate router;
extern crate rustc_serialize;
extern crate bufstream;

#[macro_use]
extern crate log;
extern crate env_logger;

mod ip;
mod locate;
mod irc;

use std::thread;

fn main() {
    let _ = env_logger::init();
    info!("Hello World");
    let _ = thread::Builder::new()
        .name(String::from("IRC BOT"))
        .spawn(irc::irc::irc_bot);
    info!("IRC Server started");
    ip::go();
}
