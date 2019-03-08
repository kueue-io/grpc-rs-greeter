### GRPC Rust Example

* GRPC rust example project for PingCAP [grpc-rs](https://github.com/pingcap/grpc-rs)
* Run generate cert script `gen-certs.sh` before run examples **(NOT READY)**

#### Test client and server with Plain Text

```
# start greeter server implemented in rust
$ cargo run --bin greeter_server

# start greeter client implemented in rust
$ cargo run --bin greeter_client
> message: "Hello world"
```

#### Test client and server with TLS (NOT READY)

```
$ cargo run --bin greeter_server -- --tls
$ cargo run --bin greeter_client -- --tls
```
