use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::status;
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;
use router::Router;

use locate;

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
    let ct = Header(ContentType(Mime(TopLevel::Text, SubLevel::Plain,
                                     vec![(Attr::Charset, Value::Utf8)])));
    let tmp = req.headers.get_raw("X-Real-IP").map(|x| {
        match String::from_utf8(x[0].clone()) {
            Ok(o) =>  format!("Your IP is: {},{}",o,locate::locate(&o)),
            Err(e) => format!("Get Host error: {:?}",e),
        }
    });
    let resp = match tmp {
        Some(x) => x,
        None => String::from("HaHa"),
    };
    let _ = req.headers.get_raw("X-Forwarded-For").map(|x| {
        let mut tmp:Vec<String> = vec![];
        for i in x {
            match String::from_utf8(i.clone()) {
                Ok(o) => tmp.push(o),
                _ => println!("我就是牛逼!!!!!"),
            }
        }
        println!("X-Forwarded-For: {:?}",tmp.join(","));
    });
    println!("Request: {:?}",resp);
    Ok(Response::with((status::Ok, resp, ct)))
}

pub fn go() {
    let mut router = Router::new();
    router.get("/", hello_world);
    let _ = Iron::new(router).http("localhost:3000");
}
