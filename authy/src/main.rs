
mod models {
    pub mod address;
    pub mod user;
}
mod repositories {
    pub mod user_repository;
}


use std::error::Error;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn  Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    println!("Running server on {addr}");

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(tutorial::api::auth::FILE_DESCRIPTOR_SET)
        .build_v1alpha()
        .unwrap();

    Server::builder()
        .add_service(reflection)
        .serve(addr)
        .await?;

    Ok(())
}
