extern crate dotenv_codegen;

use base64::prelude::*;

use isahc::{prelude::*, AsyncBody, Request};

use serde::Deserialize;
use std::collections::HashMap;

pub mod constants;
pub mod exceptions;

use constants::{RABBITMQ_MANAGEMENT_PASSWORD, RABBITMQ_MANAGEMENT_USERNAME};

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
/// # #[cfg(doctest)] {
/// use serde::Deserialize;
/// use std::collections::HashMap;
/// use rabbitmq_messages_management::send_get;
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
/// # }
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

    // Send the request and get the response to a string.
    let request = request_builder.body(()).unwrap();
    dbg!(&request);

    let mut response = isahc::send_async(request).await.unwrap();
    let response_body = response.text().await.unwrap();

    // TODO: Error handling

    // Convert to a struct so that accessing the response is easier.
    let response_data: T = serde_json::from_str(&response_body).unwrap();

    Ok(response_data)
}

/// Sends an asynchronous HTTP POST request.
///
/// This function sends an HTTP POST request to the specified URI with optional headers and a body.
/// The response is deserialized into the specified type `T`.
///
/// # Arguments
///
/// * `uri` - A string slice that holds the URI to which the request is sent.
/// * `headers` - An optional reference to a `HashMap` containing the headers to be included in the request.
/// * `body` - The body of the request, which will be converted into an `AsyncBody`.
///
/// # Returns
///
/// * `Result<T, ()>` - On success, returns the deserialized response body of type `T`. On failure, returns an empty tuple `()`.
///
/// # Type Parameters
///
/// * `T` - The type into which the response body will be deserialized. This type must implement the `Deserialize` trait for any lifetime `'de`.
/// * `B` - The type of the request body. This type must implement the `From<B>` trait to be converted into an `AsyncBody`.
///
/// # Example
///
/// ```rust
/// # #[cfg(doctest)] {
/// use std::collections::HashMap;
/// use serde::Deserialize;
/// use isahc::prelude::*;
/// use isahc::Request;
/// use rabbitmq_messages_management::send_post;
///
/// #[derive(Deserialize, Debug)]
/// struct MyResponse {
///     message: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let uri = "http://example.com/api";
///     let mut headers = HashMap::new();
///     headers.insert("Content-Type".to_string(), "application/json".to_string());
///     let body = r#"{"key": "value"}"#;
///
///     let response: MyResponse = send_post(uri, Some(&headers), body).await.unwrap();
///     println!("{:?}", response);
///
///     Ok(())
/// }
/// # }
/// ```
pub async fn send_post<'a, T, B>(
    uri: &str,
    headers: Option<&'a HashMap<String, String>>,
    body: B,
) -> Result<T, ()>
where
    AsyncBody: From<B>,
    T: for<'de> Deserialize<'de>,
{
    let mut request_builder = Request::post(uri);

    // Attach headers if provided
    if let Some(h) = headers {
        for (key, value) in h.iter() {
            request_builder = request_builder.header(key.as_str(), value.as_str());
        }
    }

    let request = request_builder.body(body).unwrap();
    let mut response = isahc::send_async(request).await.unwrap();
    let response_body = response.text().await.unwrap();

    let response_data: T = serde_json::from_str(&response_body).unwrap();

    Ok(response_data)
}

/// Prepares the authorization headers for RabbitMQ management API requests.
///
/// This function reads the RabbitMQ management username and password from environment variables,
/// encodes them in Base64, and constructs the `Authorization` header required for making API requests.
///
/// # Returns
///
/// A `HashMap` containing the `Authorization` header with the encoded credentials.
///
/// # Panics
///
/// This function will panic if the environment variables `RABBITMQ_MANAGEMENT_USERNAME` or
/// `RABBITMQ_MANAGEMENT_PASSWORD` are not set.
///
/// # Example
///
/// ```
/// # #[cfg(doctest)] {
/// use std::collections::HashMap;
/// use rabbitmq_messages_management::prepare_authorization_headers;
/// let headers = prepare_authorization_headers();
/// assert!(headers.contains_key("Authorization"));
/// # }
/// ```
pub fn prepare_authorization_headers() -> HashMap<String, String> {
    // TODO: Error handling
    HashMap::from([(
        "Authorization".to_string(),
        format!(
            "Basic {}",
            BASE64_STANDARD.encode(format!(
                "{}:{}",
                dotenv::var(RABBITMQ_MANAGEMENT_USERNAME)
                    .expect("RABBITMQ_MANAGEMENT_USERNAME not set"),
                dotenv::var(RABBITMQ_MANAGEMENT_PASSWORD)
                    .expect("RABBITMQ_MANAGEMENT_PASSWORD not set")
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
/// use rabbitmq_messages_management::prepare_url;
/// let root_uri = "http://example.com";
/// let path = "api/v1/resource";
/// let full_url = prepare_url(root_uri, path).unwrap();
/// assert_eq!(full_url, "http://example.com/api/v1/resource");
/// ```
pub fn prepare_url(root_uri: &str, path: &str) -> Result<String, ()> {
    Ok(format!("{}/{}", root_uri, path))
}
