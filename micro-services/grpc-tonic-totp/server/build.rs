fn main() {
    tonic_build::compile_protos("proto/user_totp.proto").unwrap();
}
