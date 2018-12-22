# aws-lambda-rust

Testing [this guide](https://aws.amazon.com/es/blogs/opensource/rust-runtime-for-aws-lambda/) to deploy some Rust in AWS Lambda.

If you need more information you can read my [AWS Lambda Functions written in Rust](https://robertohuertas.com/2018/12/02/aws-lambda-rust/) blog post where I describe all the steps I took to build and publish a Lambda Function in Rust.

## Building the crate in OSX

In order to build this project in OSX you must ensure that you have a folder called `.cargo` with a `config` file in it. This file must contain exactly this:

```sh
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

## Building the crate in Linux

Install the `musl-tools`:

```sh
sudo apt install musl-tools
```

Remove the `.cargo` folder if present or comment the contents of the `config` file.

## Common building steps

```sh
# this will start the build process
cargo build --release --target x86_64-unknown-linux-musl
# this will create the final zip
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
```
