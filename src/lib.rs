extern crate hyper;

use serde::Deserialize;
use tokio::net::TcpStream;
use hyper_util::rt::TokioIo;
use hyper::Request;
use hyper::client::conn::http1::Builder;
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;

/// Sends an HTTP GET request to
#[derive(PartialEq)]
pub enum Authority {
  HTTP,
  HTTPS
}

/// Sends HTTP GET request to authority://host:port/path
/// Returns a response.
pub async fn send_get<T>(uri: &str) -> Result<T, ()>
where
    T: for<'de> Deserialize<'de>,
{

  let url = uri.parse::<hyper::Uri>().unwrap();

    // Get the host and the port
  let host = url.host().expect("uri has no host");
  let port = url.port_u16().unwrap_or(80);

  let address = format!("{}:{}", host, port);

  // Establish a TCP connection with the remote host.
  let stream = TcpStream::connect(address).await.unwrap();

  // Required to read from the stream
  let io = TokioIo::new(stream);

  let authority = url.authority().unwrap().clone();

  let path = url.path();
  let req = Request::builder()
      .uri(path)
      .header(hyper::header::HOST, authority.as_str())
      .body(Empty::<Bytes>::new())
      .unwrap();

  // Create hyper HTTP client
  let (mut sender, conn) = Builder::new()
      .handshake::<_, Empty<Bytes>>(io)
      .await
      .unwrap();

  tokio::task::spawn(async move {
      if let Err(err) = conn.await {
          println!("Connection failed: {:?}", err);
      }
  });

  // Obtain the response stream (frame)
  let mut res = sender.send_request(req).await.unwrap();

  println!("Response status: {}", res.status());

  let mut response_body = Vec::new();

  while let Some(next) = res.frame().await {
      let frame = next.unwrap();
      if let Some(chunk) = frame.data_ref() {
          response_body.extend_from_slice(chunk);
      }
  }

  // Move the deserialization step outside the loop
  let response_data: T = serde_json::from_slice(&response_body).unwrap();

  Ok(response_data)
}


