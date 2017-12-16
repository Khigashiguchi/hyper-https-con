extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;

use std::io;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use serde_json::Value;

fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = "http://httpbin.org/ip".parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());
        res.body().concat2().and_then(move |body| {
            let v: Value = serde_json::from_slice(&body).unwrap();
            /*TODO: エラーハンドリング
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    e
                );
            });
            */
            println!("Current IP address is {}", v["origin"]);
            Ok(())
        })
    });
    core.run(work);
}
