extern crate futures;
extern crate hyper;
extern crate tokio_core;

//use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use hyper::{Method, Request};
use hyper::header::{ContentLength, ContentType};

fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let json = r#"{"library":"hyper"}"#;
    let uri = "http://httpbin.org/post".parse().unwrap();
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);
    let post = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());

        res.body().concat2()
    });
    // Multiple request
    //let get_uri = "http://httpbin.org/headers".parse().unwrap();
    //let get = client.get(get_uri).and_then(|res| {
    //    println!("GET: {}", res.status());

    //    res.body().concat2();
    //});
    //let work = post.join(get);
    core.run(post).unwrap();
}

