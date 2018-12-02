#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate simple_logger;

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
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
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
    if let Some(b) = e.body {
        //  if let Some(first_name) = b.first_name {
        return match b.as_ref() {
            "" => Ok(CustomOutput::new(format!(
                "Hello from Rust, my dear default user with empty parameter! (body)"
            ))),
            "error" => Err(c.new_error("Empty first name (body)")),
            _ => {
                let result: Body = serde_json::from_str(&b).unwrap();
                Ok(CustomOutput::new(format!(
                    "Hello from Rust, my dear {}! (body)",
                    result.first_name.unwrap_or("no param".to_owned())
                )))
            }
        };
        // }
    }
    Ok(CustomOutput {
        is_base64_encoded: false,
        status_code: 200,
        body: format!("Hello from Rust, my dear default user! No parameters"),
    })
}

// fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
//     if let Some(x) = e.first_name {
//         let x = x.as_str();
//         match x {
//             "" => Ok(CustomOutput {
//                 message: format!("Hello from Rust, my dear default user with empty parameter!"),
//             }),
//             "error" => Err(c.new_error("Empty first name")),
//             _ => Ok(CustomOutput {
//                 message: format!("Hello from Rust, my dear {}!", x),
//             }),
//         }
//     } else {
//         Ok(CustomOutput {
//             message: format!("Hello from Rust, my dear default user!"),
//         })
//     }
// }
