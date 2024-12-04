

pub mod authy {
    pub mod protobuf {
        tonic::include_proto!("authy.protobuf");
        const FILE_DESCRIPTOR_SET : &[u8]= tonic::include_file_descriptor_set!("my_descriptor");
    }
}
