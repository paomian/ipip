extern crate iron;
extern crate time;
extern crate hyper;

mod ip;
mod locate;

fn main() {
    ip::go();
}
