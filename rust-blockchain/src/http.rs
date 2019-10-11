extern crate futures;
extern crate hyper;

use hyper::header::HeaderValue;
use hyper::rt::{self, Future, Stream};
use hyper::service::service_fn_ok;
use hyper::Client;
use hyper::{Body, Method, Request, Response, Server};

use futures::future;
use std::io::{self, Write};

const PHRASE: &str = "Hello, World!";

fn hello_world(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(PHRASE))
}

fn run_hello_server() {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(hello_world)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}

fn client_function() -> impl Future<Item = (), Error = ()> {
    let client = Client::new();
    let payload = "12345".to_string();
    let uri: hyper::Uri = "http://www.baidu.com".parse().unwrap();
    let mut req = Request::new(Body::from(payload));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut()
        .insert(hyper::header::CONTENT_TYPE, HeaderValue::from_static(""));
    let post = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());
        return future::ok(());
    });

    post.map_err(|err| {
        println!("Error: {}", err);
    })
    /*request(req) http://127.0.0.1:3000
    .map(|res| {
        println!("POST: {}", res.status());
    })
    .map_err(|err| {
        println!("Error: {}", err);
    })
    */
}
fn testclient_function() -> impl Future<Item = (), Error = ()> {
    let url = String::from("http://www.baidu.com")
        .parse::<hyper::Uri>()
        .unwrap();
    let client = Client::new()
        .get(url)
        .map_err(|_| ())
        .and_then(|res| {
            println!("{}", res.status());
            return future::ok(());
        })
        .map_err(|_| ());
    return client;
}
fn run_client() {
    hyper::rt::run(client_function());
}
#[cfg(test)]
mod test {
    fn test_http() {
        //run_hello_server();
        run_client();
        println!("haha");
    }
}
