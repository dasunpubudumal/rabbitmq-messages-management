use http_body_util::{BodyExt, Empty};
use hyper::client::conn::http1::Builder;
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use serde::Deserialize;
use tokio::net::TcpStream;

#[derive(Debug, Deserialize)]
struct ResponseData {
    pub origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // Parse or panic
    let url = "http://httpbin.org/ip".parse::<hyper::Uri>()?;

    let host = url.host().expect("URI has no host");
    let port = url.port_u16().unwrap_or(80);    // Default web port
    let address = format!("{}:{}", host, port);

    // Establish a TCP connection with the remote host.
    let stream = TcpStream::connect(address).await?;

    // Required to read from the stream
    let io = TokioIo::new(stream);

    let authority = url.authority().unwrap().clone();
    let path = url.path();
    let req = Request::builder()
                            .uri(path)
                            .header(hyper::header::HOST, authority.as_str())
                            .body(Empty::<Bytes>::new())?;

    // Create hyper HTTP client
    let (mut sender, conn) 
        = Builder::new()
            .handshake::<TokioIo<tokio::net::TcpStream>, http_body_util::Empty<Bytes>>(io)
            .await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    // Obtain the response stream (frame)
    let mut res = sender.send_request(req).await?;

    println!("Response status: {}", res.status());
    println!("Headers: {:#?}", res.headers());

    let mut response_body = Vec::new();

    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            response_body.extend_from_slice(chunk);
        }
    }

    let response_data: ResponseData = serde_json::from_slice(&response_body)?;

    println!("Origin: {}", response_data.origin);

    Ok(())
}
