# Rust zmqRemoteApi

A Rust ZeroMQ remote API for coppeliasim

to run tests

```bash
cargo test
```

to run an example:
make sure to open the correct coppeliasim scene and
run the cargo command:

```bash
cargo run --example=simple_test
```

Perhaps you want to see the zmq communication logs.
There is two logs levels:

- level 1: debug, debug level you will see the request in json format
- level 2: trace, trace level you will see the request in json and bytes format

```bash
RUST_LOG="trace" cargo run --example=simple_test
```

## Porting C++ client to Rust client

The __RemoteAPIObjects.h__ has 3750 lines of code, so to port all the
functions, I create a kind of [c_transpiler](c_transpiler/).

## diferences between C++ client and Rust client

At moment the only difference encountered is in

```c++
std::vector<uint8_t> getStringSignal(std::string signalName);
```

in rust the function returns a std::String. I haven't observed any
examples where the function returns a block of bytes.
