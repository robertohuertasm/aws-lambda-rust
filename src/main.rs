#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;

use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "queryStringParameters")]
    query_string_parameters: Option<QueryString>,
    body: Option<Body>,
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
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if let Some(q) = e.query_string_parameters {
        if let Some(first_name) = q.first_name {
            return match first_name.as_ref() {
                "" => Ok(CustomOutput::new(
                    "Hello from Rust, my dear default user with empty qs parameter!".to_owned(),
                )),
                "error" => Err(c.new_error("Empty first name. Forced error. Qs.")),
                _ => Ok(CustomOutput::new(format!(
                    "Hello from Rust, my dear {} (qs)!",
                    first_name
                ))),
            };
        }
    }
    if let Some(b) = e.body {
        if let Some(first_name) = b.first_name {
            return match first_name.as_ref() {
                "" => Ok(CustomOutput::new(
                    "Hello from Rust, my dear default user with empty body!".to_owned(),
                )),
                "error" => Err(c.new_error("Empty first name. Forced error. Body.")),
                _ => Ok(CustomOutput::new(format!(
                    "Hello from Rust, my dear {} (body)!",
                    first_name
                ))),
            };
        }
    }
    Ok(CustomOutput {
        is_base64_encoded: false,
        status_code: 200,
        body: format!("Hello from Rust, my dear default user! No body or qs"),
    })
}
