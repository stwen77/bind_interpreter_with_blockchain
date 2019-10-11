use hyper::header::HeaderValue;
use hyper::rt::{self, Future, Stream};
use hyper::service::service_fn_ok;
use hyper::Client;
use hyper::{Body, Method, Request, Response, Server};

use futures::future;
use std::io::{self, Write};

static PHRASE: &str = "Hello, World!";
static FILE: &str = "blocks.txt";

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
}

fn run_client() {
    hyper::rt::run(client_function());
}

use block::Block;
use std::sync::{Arc, Mutex};

pub fn get_blocks(chain: &Arc<Mutex<Vec<Block>>>) -> String {
    let chain = chain.lock().unwrap();
    let mut blocks_content = String::new();

    for block in chain.iter() {
        let content = block.get_content();
        blocks_content.push_str(&format!("Previous Hash: {}", block.get_previous()));
        blocks_content.push_str(&format!("Hash: {}", block.get_current()));
        blocks_content.push_str(&format!("Timestamp: {}", content.get_timestamp()));
        blocks_content.push_str(&format!("Data: {:?} \n\n", content.get_data()));
    }
    blocks_content
}

fn display_service(_req: Request<Body>) -> Response<Body> {
    let body = Body::from(PHRASE);
    Response::new(body)
}

pub fn run_display_server(chain: Arc<Mutex<Vec<Block>>>) {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(display_service)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}
#[cfg(test)]
mod test {
    fn test_http() {
        //run_hello_server();
        run_client();
        println!("haha");
    }
}
