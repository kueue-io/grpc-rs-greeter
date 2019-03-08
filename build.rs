extern crate protoc_grpcio;

fn main() {
    let proto_output = "src";
    let proto_include = "proto";
    println!("cargo:rerun-if-changed={}", proto_include);
    protoc_grpcio::compile_grpc_protos(
        &["proto/helloworld.proto"],
        &[proto_include], &proto_output)
        .expect("Failed to compile gRPC definitions!");
}
