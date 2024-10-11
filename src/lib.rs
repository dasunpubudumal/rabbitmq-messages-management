#[macro_use]
extern crate dotenv_codegen;

use base64::prelude::*;

use isahc::{prelude::*, Request};

use serde::Deserialize;
use std::collections::HashMap;

/// Sends an HTTP GET request to
#[derive(PartialEq)]
pub enum Authority {
    HTTP,
    HTTPS,
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
pub async fn send_get<'a, T>(
    uri: &str,
    headers: Option<&'a HashMap<String, String>>,
) -> Result<T, ()>
where
    T: for<'de> Deserialize<'de>,
{
    let mut request_builder = Request::get(uri);

    // Attach headers if provided
    if let Some(h) = headers {
        for (key, value) in h.iter() {
            request_builder = request_builder.header(key.as_str(), value.as_str());
        }
    }

    let request = request_builder.body(()).unwrap();
    let mut response = isahc::send_async(request).await.unwrap();

    let response_body = response.text().await.unwrap();

    dbg!(&response_body);

    // Move the deserialization step outside the loop
    let response_data: T = serde_json::from_str(&response_body).unwrap();

    Ok(response_data)
}

pub fn prepare_authorization_headers() -> HashMap<String, String> {
    HashMap::from([(
        "Authorization".to_string(),
        format!(
            "Basic {}",
            BASE64_STANDARD.encode(format!(
                "{}:{}",
                dotenv!("RABBITMQ_MANAGEMENT_USERNAME"),
                dotenv!("RABBITMQ_MANAGEMENT_PASSWORD")
            ))
        ),
    )])
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
