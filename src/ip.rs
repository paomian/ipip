use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::status;
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;
use router::Router;

use locate;

use std::collections::BTreeMap;
//use rustc_serialize::json::{self, Json};
use rustc_serialize::json::{Json, ToJson};

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
        info!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn hello_world(req: &mut Request) -> IronResult<Response> {
    let ct = Header(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])));
    let mut d = BTreeMap::new();
    let resp = match req.headers.get_raw("X-Real-IP") {
        Some(x) => match String::from_utf8(x[0].clone()) {
            Ok(o) =>  {
                d.insert(String::from("ip"), o.to_json());
                d.insert(String::from("locate"), locate::locate(&o));
                d.insert(String::from("error"), Json::Null);
                d
            },
            Err(e) => {
                d.insert(String::from("error"),format!("Get Host error: {:?}",e).to_json());
                d
            },
        },
        None => {d.insert(String::from("error"),"HaHa".to_json());d},
    };

    let _ = req.headers.get_raw("X-Forwarded-For").map(|x| {
        let mut tmp:Vec<String> = vec![];
        for i in x {
            match String::from_utf8(i.clone()) {
                Ok(o) => tmp.push(o),
                _ => error!("我就是牛逼!!!!!"),
            }
        }
        info!("X-Forwarded-For: {}",tmp.join(","));
    });

    let resp_json = Json::Object(resp).to_string();
    info!("Request: {}",resp_json);
    Ok(Response::with((status::Ok,
                       resp_json,
                       ct)))
}

pub fn go() {
    let mut router = Router::new();
    router.get("/", hello_world);
    let _ = Iron::new(router).http("localhost:3000");
}
