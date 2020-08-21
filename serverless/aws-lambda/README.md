A simple demonstration of serverless run time environment for a lambda function which can do base64 encoding and decoding of utf8 text.

Prepare rust environment for development in AWS environment
```
https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/
```

Run time documentation
```
https://github.com/awslabs/aws-lambda-rust-runtime
```
Invoke build
```
cargo build --release --target x86_64-unknown-linux-musl
```

Zip the binary to upload into the aws account
```
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
```
