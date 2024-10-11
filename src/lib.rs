extern crate hyper;

use hyper::header::HeaderName;
use serde::Deserialize;
use tokio::net::TcpStream;
use hyper_util::rt::TokioIo;
use hyper::{header::HeaderValue, Request};
use hyper::client::conn::http1::Builder;
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use std::collections::HashMap;
use std::str::FromStr;

/// Sends an HTTP GET request to
#[derive(PartialEq)]
pub enum Authority {
  HTTP,
  HTTPS
}

/// Sends an HTTP GET request to the specified URI and deserializes the response body into the specified type.
///
/// # Type Parameters
///
/// - `T`: The type to deserialize the response body into. This type must implement the `Deserialize` trait for any lifetime.
///
/// # Arguments
///
/// - `uri`: A string slice that holds the URI to which the GET request will be sent.
/// - `headers`: An optional reference to a `HashMap` containing headers to be included in the request. The keys and values in the `HashMap` must have a `'static` lifetime.
///
/// # Returns
///
/// - `Result<T, ()>`: On success, returns the deserialized response body of type `T`. On failure, returns an empty tuple `()`.
///
/// # Errors
///
/// This function will return an error if:
/// - The URI cannot be parsed.
/// - The TCP connection cannot be established.
/// - The HTTP request fails.
/// - The response body cannot be deserialized into the specified type.
///
/// # Example
///
/// ```rust
/// use serde::Deserialize;
/// use std::collections::HashMap;
///
/// #[derive(Deserialize, Debug)]
/// struct Ip {
///     ip: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     let mut headers = HashMap::new();
///     headers.insert("User-Agent".to_string(), "my-app/1.0".to_string());
///
///     let result: Ip = send_get("http://httpbin.org/ip", Some(&headers)).await.unwrap();
///     println!("{:?}", result);
///     Ok(())
/// }
/// ```
pub async fn send_get<'a, T>(uri: &str, headers: Option<&'a HashMap<String, String>>) -> Result<T, ()>
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
  let mut req = Request::builder()
      .uri(path)
      .header(hyper::header::HOST, authority.as_str())
      .body(Empty::<Bytes>::new())
      .unwrap();

  match headers {
      Some(h) => {
        let headers_mutable = req.headers_mut();
        for key in h.keys() {
          headers_mutable.insert(HeaderName::from_str(key).unwrap(), HeaderValue::from_str(h.get(key).unwrap()).unwrap());
        }
      },
      _ => println!("No headers to attach.")
  }

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

// Constructs a full URL by concatenating the root URI and the path.
///
/// # Arguments
///
/// - `root_uri`: A string slice that holds the root URI.
/// - `path`: A string slice that holds the path to be appended to the root URI.
///
/// # Returns
///
/// - `Result<String, ()>`: On success, returns the full URL as a `String`. On failure, returns an empty tuple `()`.
///
/// # Example
///
/// ```rust
/// let root_uri = "http://example.com";
/// let path = "api/v1/resource";
/// let full_url = prepare_url(root_uri, path).unwrap();
/// assert_eq!(full_url, "http://example.com/api/v1/resource");
/// ```
pub fn prepare_url(root_uri: &str, path: &str) -> Result<String, ()> {
  Ok(format!("{}/{}", root_uri, path))
}


