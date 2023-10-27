use crate::martus_auth as proto;
use proto::auth_server::Auth;
use proto::{HealthCheckRequest, HealthCheckResponse};
use tonic::Response;

#[derive(Debug, Default)]
pub struct GrpcServer {}

#[tonic::async_trait]
impl Auth for GrpcServer {
    // the health check rpc
    async fn health_check(
        &self,
        _request: tonic::Request<HealthCheckRequest>,
    ) -> std::result::Result<tonic::Response<HealthCheckResponse>, tonic::Status> {
        // build the response
        let response = HealthCheckResponse {
            status: "OK".to_string(),
            message: "Service up and running".to_string(),
        };
        Ok(Response::new(response))
    }
}
