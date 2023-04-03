<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [Unofficial Rust zmqRemoteApi](#unofficial-rust-zmqremoteapi)
  - [Porting C++ client to Rust client](#porting-c-client-to-rust-client)
  - [diferences between C++ client and Rust client](#diferences-between-c-client-and-rust-client)
  - [running the opencv example](#running-the-opencv-example)
  - [Get started](#get-started)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->
![example workflow](https://github.com/samuel-cavalcanti/rust_zmqRemoteApi/actions/workflows/rust.yml/badge.svg?branch=main)
# Unofficial Rust zmqRemoteApi

This client support the coppeliasim **4.4.0** version

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

The **RemoteAPIObjects.h** has 3750 lines of code, so to port all the
functions, I create a kind of [c_transpiler](c_transpiler/).

## diferences between C++ client and Rust client

At moment the only difference encountered is in

```c++
std::vector<uint8_t> getStringSignal(std::string signalName);
```

in rust the function returns a std::String. I haven't observed any
examples where the function returns a block of bytes.

## running the opencv example

[![Watch the video](https://img.youtube.com/vi/fo8G43WZQ6c/maxresdefault.jpg)](https://youtu.be/fo8G43WZQ6c)

## Get started

create a new rust project:

```bash
cargo new new_project
```

add this crate at your cargo.toml :

```toml
# the branch is the coppelia version
zmq_remote_api = { git = "https://github.com/samuel-cavalcanti/rust_zmqRemoteApi", branch = "CoppeliaSim_4.4.0"}
```

See this simple [example](examples/get_simulation_time.rs) to understand how to use this create.

### Main Branch

the main branch use the smart pointers
```rust
// main branch
let client = zmq_remote_api::RemoteApiClient::new(RemoteApiClientParams {
host: "localhost".to_string(),
..RemoteApiClientParams::default()
})?;

// Arc means Atomic reference counter, is a smart pointer that counter the number of references
let client = Rc::new(client);
let sim = Sim::new(client.clone());
```
Using smart pointer is easier than dealing with borrowing and life cycle.

```rust
// CoppeliaSim_4.4.0, CoppeliaSim_4.3.0

 let client = zmq_remote_api::RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    let sim = Sim::new(&client);
```