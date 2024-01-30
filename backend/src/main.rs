use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = Response::builder()
        .status(200)
        .body(Body::from("Hello, World!"))
        .unwrap();

    Ok(response)
}

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| {
        let service = service_fn(handle_request);
        async move { Ok::<_, Infallible>(service) }
    });

    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
