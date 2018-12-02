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
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    let x = e.first_name.as_str();
    match x {
        "" => Ok(CustomOutput {
            is_base64_encoded: false,
            status_code: 200,
            body: format!("Hello from Rust, my dear default user!"),
        }),
        "error" => Err(c.new_error("Empty first name")),
        _ => Ok(CustomOutput {
            is_base64_encoded: false,
            status_code: 200,
            body: format!("Hello from Rust, my dear {}!", e.first_name),
        }),
    }
}
