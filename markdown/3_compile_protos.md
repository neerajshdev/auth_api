
# Step by step guide to build authentication api in Rust using gRPC

In the previous part, we defined our gRPC service interface using Protocol Buffers. In this part, we'll compile the proto file to generate the service stubs and message types and also use the service reflection to make it easy to testing the service from postman.

If you don't know about the postman, it is a popular API client that allows you to test APIs by sending requests and viewing responses. You can download it from [here](https://www.postman.com/downloads/). Developers use Postman to test APIs, document APIs, monitor APIs, and share APIs with others. we can also use it to test our gRPC service.

## Table of Contents

- [Compiling the Protobuf Definitions](#compiling-the-protobuf-definitions)
   - [Creating the build script](#creating-the-build-script)
   - [Installing the protoc compiler](#installing-the-protoc-compiler)


## Compiling the Protobuf Definitions

In rust we can write build scripts that are executed before the build process. We can use this to compile the proto file and generate the service stubs and message types. We will use the `tonic-build` crate to compile the proto file.

In the previous part, we created the `proto_stub` crate. Now we are going to work on this crate to compile the proto file. First, make sure to add the `tonic-build` crate to the `build-dependencies` section of the `Cargo.toml` file in the `proto_stub` crate.

`file-path: proto_stub/Cargo.toml`

```toml
[package]
    ...

[dependencies]
tonic.workspace = true
prost-types.workspace = true
prost.workspace = true

[build-dependencies]
tonic-build.workspace = true

```

In dependencies section, we added the tonic, prost-types, and prost crates and foreach crate we set the `workspace` flag to `true`. This flag is used to tell the cargo that the crate is a workspace crate and its version is defined in the workspace `Cargo.toml` file. Similarly We also added the `tonic-build` crate to the `build-dependencies` section.

`tonic` : The tonic crate is a gRPC library for Rust. It provides a client and server implementation for gRPC services. The tonic crate is built on top of the tokio crate, which is an asynchronous runtime for Rust.

`prost-types` : The prost-types crate provides the well-known types for the prost crate. The well-known types are the types defined by the protobuf standard. The prost crate is a Rust implementation of the Protocol Buffers serialization format.

`prost` : The prost crate is a Rust implementation of the Protocol Buffers serialization format. The prost crate is used to generate Rust code from the Protocol Buffers definitions.

### Creating the build script

Now we need to create a build script to compile the proto file. Create a new file named `build.rs` in the `proto_stub` crate and add the following code to the file.

`file-path: proto_stub/build.rs`

```rust
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

    tonic_build::configure()
        // compile_well_known_types will generate code for well known types like google.protobuf.Empty  
        // we can disable this if we use the prost-types crate, because i am using prost-types crate i will kept this disabled.
        /* .compile_well_known_types(true) */

        // file_descriptor_set_path will generate a file descriptor set in the output directory
        // this file will be used by the server to provide reflection
        .file_descriptor_set_path(out_dir.join("my_descriptor.bin"))
        .compile_protos(
            // the path to the main proto file to compile
            &["proto/authy/protobuf/main.proto"], 
            // the path to the directory containing the proto files dependencies
            // protobuf compiler will look for dependencies in this directory
            &["proto/authy/protobuf"], 
        )?;

    Ok(())
}

```

In the build script, we first get the output directory path using the `std::env::var("OUT_DIR")?` function. The output directory is where the generated code will be placed.

Next, we call the `tonic_build::configure()` function to configure the build process. We can pass various options to the `configure()` function to customize the build process. In this case, we are using the `file_descriptor_set_path()` function to generate a file descriptor set in the output directory. The file descriptor set is used by the server to provide reflection.

Finally, we call the `compile_protos()` function to compile the proto file. The `compile_protos()` function takes two arguments: the path to the main proto file to compile and the path to the directory containing the proto file dependencies.

### Installing the protoc compiler

Now if you run the `cargo build -p proto_stub` command in the terminal, you will see build errors because we don't have the protoc compiler installed. We need to install the protoc compiler to compile the proto file.

In windows, you can install the protoc compiler by using chocolatey package manager. Run the following command in the terminal to install the protoc compiler.

```powershell
choco install protoc
```

In linux, you can install the protoc compiler by running the `sudo apt install protobuf-compiler` command in the terminal. In mac, you can install the protoc compiler by running the `brew install protobuf` command in the terminal.

You can also download the protoc compiler from the [official protobuf release page](https://github.com/protocolbuffers/protobuf/releases/tag/v29.1) and add the path to the protoc compiler to the system environment variable.

After installing the protoc compiler, check the protoc compiler version by running the `protoc --version` command in the terminal. If the protoc compiler is installed correctly, you will see the protoc compiler version.

Now run the `cargo build -p proto_stub` command in the terminal. It will run the build script and compile the proto file to generate the service stubs and message types. And then build the `proto_stub` crate.

## Including the stubs in the proto_stub crate

Now we have the generated service stubs and message types in the output target directory.



pub mod authy {
    pub mod protobuf {
        tonic::include_proto!("authy.protobuf");
        const FILE_DESCRIPTOR_SET : &[u8]= tonic::include_file_descriptor_set!("my_descriptor");
    }
}
