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

// Struct that will hold information of the request.
// When we use an API Gateway as a proxy, which is the default
// behaviour when we create it from the Lambda website, the request
// will have a specific format with many different parameters.
// We're only going to use `queryStringParameters` to check the
// query string parameters (normally for GET requests) and `body`
// to check for messages usually coming from POST requests.
#[derive(Deserialize, Clone)]
struct CustomEvent {
    // note that we're using serde to help us to change
    // the names of parameters accordingly to conventions.
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

// Struct used for our function's response.
// Note again that we're using `serde`.
// It's also important to notice that you will need to explicitely
// inform these properties for our API Gateway to work.
// If you miss some of these properties you will likely get
// a 502 error.
#[derive(Serialize, Clone)]
struct CustomOutput {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: String,
}

// Just a static method to help us build the `CustomOutput`.
impl CustomOutput {
    fn new(body: String) -> Self {
        CustomOutput {
            is_base64_encoded: false,
            status_code: 200,
            body,
        }
    }
}

// This is our function entry point.
// Note the use of the `lambda!` macro. It will hold our handler:
// pub type Handler<E, O> = fn(E, Context) -> Result<O, HandlerError>
fn main() -> Result<(), Box<dyn Error>> {
    // simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

// Our handler will just check for a query string parameter called `firstName`.
// Note the different behavior depending on the value of the parameter.
// In case there's no query string parameter, we'll check the body of the request.
// The body comes as a string so we'll have to use `Serde` again to deserialize it.
// Finally, if we have no body, we'll return a default response.
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
