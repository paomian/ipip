extern crate iron;
extern crate time;
extern crate hyper;
extern crate router;

mod ip;
mod locate;

fn main() {
    ip::go();
}
