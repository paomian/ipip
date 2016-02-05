extern crate iron;
extern crate time;
extern crate hyper;
extern crate router;

#[macro_use]
extern crate log;
extern crate env_logger;

mod ip;
mod locate;

fn main() {
    let _ = env_logger::init();
    info!("Hello World");
    ip::go();
}
