pub mod hello_world {
    pub(super) mod proto {
        tonic::include_proto!("provision"); // The string specified here must match the proto package name
    }

    pub use proto::{
        HelloReply, HelloRequest,
        greeter_client::GreeterClient,
        greeter_server::{Greeter, GreeterServer},
    };

    pub use proto::{
        CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, PullRequest, PullResponse,
        RestartRequest, RestartResponse,
        provisioner_client::ProvisionerClient,
        provisioner_server::{Provisioner, ProvisionerServer},
    };
}
