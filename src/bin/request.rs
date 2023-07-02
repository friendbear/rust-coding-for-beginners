use hyper::client::connect::HttpConnector;
use hyper::client::ResponseFuture;
use hyper::{self, Body, Method, Request};
use hyper_tls::HttpsConnector;
// type Result<T> = std::result::Result<T, Error>;

// #[cfg(feature = "native_tls")]
fn new_https_connector() -> hyper_tls::HttpsConnector<HttpConnector> {
    hyper_tls::HttpsConnector::new()
}
const API_ENDPOINT: &str = "https://echo.paw.cloud/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connector = new_https_connector();
    let client = hyper::Client::builder().build(connector);

    let echo_request = request_echo(API_ENDPOINT);
    let resp = client.request(echo_request).await?;

    let (parts, body) = resp.into_parts();
    let body: Vec<_> = hyper::body::to_bytes(body).await?.to_vec();
    //let headers = resp.headers()
    dbg!(parts);
    dbg!(body);
    Ok(())
}

fn request_echo(url: &str) -> Request<Body> {
    let request = Request::builder()
        .method("GET")
        .uri(url)
        .header("X-Echo", "helth ok")
        .body(Body::empty())
        .unwrap();

    return request;
}
