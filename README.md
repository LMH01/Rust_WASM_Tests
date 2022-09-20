# Rust WASM Tests
This repository contains basic tests on how integration between rust and web assembly could work.

The knowledge gained here will be used in [Acquire_rs_web](https://github.com/LMH01/Acquire_rs_web).

## Idea
The basic idea is that this project contains two rust projects. 

The main project (located in the main directory) containing the code for the server and the wasm project (located in [/wasm](/wasm)) that is used to build the web assembly parts that are then accessed by javascript. This way it will be less of a pain to write javascript and most work can be done comfortably in rust.

## Building and running
To build and run the test server do the following:

1. Clone the repository and cd into the main directory
2. Make sure that the `rust toolchain` and `wasm-pack` are installed
3. Run `./build_wasm.sh`
4. Run `cargo run`

This will start the test server which can be accessed under `127.0.0.1:8000`.

### Script
The [build_wasm.sh](build_wasm.sh) script is used to build the web assembly parts and copy the output files to `/web/wasm`. This way it is easy to update the files once the source code has been modified.