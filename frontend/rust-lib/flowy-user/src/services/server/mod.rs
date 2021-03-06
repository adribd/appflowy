mod server_api;
mod server_api_mock;

pub use server_api::*;
pub use server_api_mock::*;

use std::sync::Arc;
pub(crate) type Server = Arc<dyn UserServerAPI + Send + Sync>;
use crate::{
    entities::{SignInParams, SignInResponse, SignUpParams, SignUpResponse, UpdateUserParams, UserProfile},
    errors::FlowyError,
};
use backend_service::configuration::ClientServerConfiguration;
use lib_infra::future::FutureResult;

pub trait UserServerAPI {
    fn sign_up(&self, params: SignUpParams) -> FutureResult<SignUpResponse, FlowyError>;
    fn sign_in(&self, params: SignInParams) -> FutureResult<SignInResponse, FlowyError>;
    fn sign_out(&self, token: &str) -> FutureResult<(), FlowyError>;
    fn update_user(&self, token: &str, params: UpdateUserParams) -> FutureResult<(), FlowyError>;
    fn get_user(&self, token: &str) -> FutureResult<UserProfile, FlowyError>;
    fn ws_addr(&self) -> String;
}

pub(crate) fn construct_user_server(config: &ClientServerConfiguration) -> Arc<dyn UserServerAPI + Send + Sync> {
    if cfg!(feature = "http_server") {
        Arc::new(UserHttpServer::new(config.clone()))
    } else {
        Arc::new(UserServerMock {})
    }
}
