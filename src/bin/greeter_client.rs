extern crate grpcio;
extern crate grpc_greeter;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
use grpc_greeter::helloworld::HelloRequest;
use grpc_greeter::helloworld_grpc::GreeterClient;


fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::new();
    req.set_name("world".to_owned());
    let reply = client.say_hello(&req).expect("rpc");
    println!("Greeter received: {:?}", reply);

}
