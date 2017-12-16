extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;
extern crate serde_json;
extern crate dotenv;

use futures::{Future, Stream};
use hyper::Client;
use hyper::{Method, Request};
use hyper::header::{ContentType};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use serde_json::Value;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let docbase_token = env::var("DOC_BASE_TOKEN").unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(1, &handle).unwrap())
        .build(&handle);

    let json = r#"{}"#;
    let uri = "https://api.docbase.io/teams".parse().unwrap();
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set_raw("X-Api-Version", "1");
    req.headers_mut().set_raw("X-DocBaseToken", docbase_token);
    req.set_body(json);

    let get = client.request(req).and_then(|res| {
        println!("GET: {}", res.status());
        res.body().concat2().and_then(move |body| {
            let v: Value = serde_json::from_slice(&body).unwrap();
            println!("Your team name is {}.", v[0]["domain"]);
            Ok(())
        })
    });
    core.run(get).unwrap();
}
