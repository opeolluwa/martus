use crate::martus_auth::{
    auth_server::Auth, ChangePasswordRequest, ChangePasswordResponse, ForgotPasswordRequest,
    ForgotPasswordResponse, HealthCheckRequest, HealthCheckResponse, LoginRequest, LoginResponse,
    LogoutRequest, LogoutResponse, RefreshTokenRequest, RefreshTokenResponse, ResetPasswordRequest,
    ResetPasswordResponse, SignUpResponse, SignupRequest, VerifyEmailRequest, VerifyEmailResponse,
    VerifyTokenRequest, VerifyTokenResponse,
};
use c_enum::c_enum;
use tonic::Response;

c_enum! {
   pub  enum Status : &'static str {
    Ok = "OK",
    Failed = "FAILED",
}
}

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

    async fn sign_up(
        &self,
        request: tonic::Request<SignupRequest>,
    ) -> std::result::Result<tonic::Response<SignUpResponse>, tonic::Status> {
        todo!()
    }

    async fn login(
        &self,
        request: tonic::Request<LoginRequest>,
    ) -> std::result::Result<tonic::Response<LoginResponse>, tonic::Status> {
        todo!()
    }

    async fn logout(
        &self,
        request: tonic::Request<LogoutRequest>,
    ) -> std::result::Result<tonic::Response<LogoutResponse>, tonic::Status> {
        todo!()
    }

    async fn refresh_token(
        &self,
        request: tonic::Request<RefreshTokenRequest>,
    ) -> std::result::Result<tonic::Response<RefreshTokenResponse>, tonic::Status> {
        todo!()
    }

    async fn verify_token(
        &self,
        request: tonic::Request<VerifyTokenRequest>,
    ) -> std::result::Result<tonic::Response<VerifyTokenResponse>, tonic::Status> {
        todo!()
    }

    async fn verify_email(
        &self,
        request: tonic::Request<VerifyEmailRequest>,
    ) -> std::result::Result<tonic::Response<VerifyEmailResponse>, tonic::Status> {
        todo!()
    }

    async fn forgot_password(
        &self,
        request: tonic::Request<ForgotPasswordRequest>,
    ) -> std::result::Result<tonic::Response<ForgotPasswordResponse>, tonic::Status> {
        todo!()
    }

    async fn reset_password(
        &self,
        request: tonic::Request<ResetPasswordRequest>,
    ) -> std::result::Result<tonic::Response<ResetPasswordResponse>, tonic::Status> {
        todo!()
    }

    async fn change_password(
        &self,
        request: tonic::Request<ChangePasswordRequest>,
    ) -> std::result::Result<tonic::Response<ChangePasswordResponse>, tonic::Status> {
        todo!()
    }
}
