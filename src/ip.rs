extern crate iron;
extern crate time;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn hello_world(req: &mut Request) -> IronResult<Response> {
    let tmp = req.headers.get_raw("X-Real-IP").map(|x| {
        match String::from_utf8(x[0].clone()) {
            Ok(o) =>  format!("Your IP is: {:?}",o),
            Err(e) => format!("Get Host error: {:?}",e),
        }
    });
    let resp = match tmp {
        Some(x) => x,
        None => String::from("HaHa"),
    };
    println!("Request: {:?}",resp);
    Ok(Response::with((iron::status::Ok, resp)))
}

pub fn go() {
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    let _ = Iron::new(chain).http("localhost:3000");
}
