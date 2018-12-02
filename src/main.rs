#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
// #[macro_use]
// extern crate log;
// extern crate simple_logger;

use lambda::error::HandlerError;
use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "queryStringParameters")]
    query_string_parameters: Option<QueryString>,
    body: Option<String>,
}

#[derive(Deserialize, Clone)]
struct QueryString {
    #[serde(rename = "firstName")]
    first_name: Option<String>,
}

#[derive(Deserialize, Clone)]
struct Body {
    #[serde(rename = "firstName")]
    first_name: Option<String>,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: String,
}

impl CustomOutput {
    fn new(body: String) -> Self {
        CustomOutput {
            is_base64_encoded: false,
            status_code: 200,
            body,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    // checking the query string
    if let Some(q) = e.query_string_parameters {
        if let Some(first_name) = q.first_name {
            return match first_name.as_ref() {
                "" => Ok(CustomOutput::new(format!(
                    "Hello from Rust, my dear default user with empty parameter! (qs)"
                ))),
                "error" => Err(c.new_error("Empty first name (qs)")),
                _ => Ok(CustomOutput::new(format!(
                    "Hello from Rust, my dear {}! (qs)",
                    first_name
                ))),
            };
        }
    }

    // cheking the body
    if let Some(b) = e.body {
        let parsed_body: Result<Body, serde_json::Error> = serde_json::from_str(&b);
        if let Ok(result) = parsed_body {
            return match result.first_name.as_ref().map(|s| &s[..]) {
                Some("") => Ok(CustomOutput::new(format!(
                    "Hello from Rust, my dear default user with empty parameter! (body)"
                ))),
                Some("error") => Err(c.new_error("Empty first name (body)")),
                _ => Ok(CustomOutput::new(format!(
                    "Hello from Rust, my dear {}! (body)",
                    result.first_name.unwrap_or("".to_owned())
                ))),
            };
        }
    }

    Ok(CustomOutput {
        is_base64_encoded: false,
        status_code: 200,
        body: format!("Hello from Rust, my dear default user! No parameters"),
    })
}
