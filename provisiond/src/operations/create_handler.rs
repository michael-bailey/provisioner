// use std::path::Path;
// use tonic::{Code, Response, Status};
// use libprovision::hello_world::{CreateRequest, CreateResponse};
// 
// 
// pub fn create_handler(
//     
//     req: CreateRequest
// ) -> Result<Response<CreateResponse>, Status> {
// 
//     let CreateRequest {
//         service_name
//     } = req;
//     
//     
// 
//     let home_folder = Path::new("srv").join(service_name);
//     let folder_exists = home_folder.exists();
//     
//     if folder_exists { return Err(Status::new(Code::AlreadyExists, "User already exists.")); }
//     
//     
//     let create_res = tokio::fs::create_dir(home_folder);
//     
//     if let Err(err) = create_res {
//         
//     }
//     
//     
//     
//     home_folder.
//     
//     Ok(Response::new(CreateResponse {}))
// }