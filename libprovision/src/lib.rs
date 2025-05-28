pub mod hello_world {
    pub(super) mod proto {
        tonic::include_proto!("provision"); // The string specified here must match the proto package name
    }

    pub use proto::{
        greeter_server::{
            GreeterServer,
            Greeter,
        },
        greeter_client::GreeterClient,
        HelloReply,
        HelloRequest
    };
}