use super::OutputFormat;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize)]
pub struct HelloRequest  {
    version: String,
    request_type: String,
    output_format: OutputFormat,
}

impl HelloRequest {
    fn new(output_format: OutputFormat) -> Self {
        HelloRequest {
            version: env!("CARGO_PKG_VERSION").to_string(),
            output_format,
            request_type: "hello".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct HelloResponse {
    // maybe rename to Extension info
    name: String,
    version: String,
    description: String,
    errors: Vec<String>, // maybe wrapped in an optional
    warnings: Vec<String>,
    interests: Vec<String>,
    block_support: bool,
    inline_support: bool,
}

pub fn greet() -> String {
    serde_json::to_string(&HelloRequest::new(OutputFormat::Html)).unwrap()
}
