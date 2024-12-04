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
