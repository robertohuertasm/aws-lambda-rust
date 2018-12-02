# aws-rust

Testing [this guide](https://aws.amazon.com/es/blogs/opensource/rust-runtime-for-aws-lambda/) to deploy some Rust in AWS Lambda.

## Building the crate in OSX

`cargo build --release --target x86_64-unknown-linux-musl`
`zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap`
