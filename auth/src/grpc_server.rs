use crate::{
    database::{UserInformation, UserInformationBuilder},
    martus_auth::{
        auth_server::Auth, ChangePasswordRequest, ChangePasswordResponse, ForgotPasswordRequest,
        ForgotPasswordResponse, HealthCheckRequest, HealthCheckResponse, LoginRequest,
        LoginResponse, LogoutRequest, LogoutResponse, RefreshTokenRequest, RefreshTokenResponse,
        ResetPasswordRequest, ResetPasswordResponse, SignUpResponse, SignupRequest,
        VerifyEmailRequest, VerifyEmailResponse, VerifyTokenRequest, VerifyTokenResponse,
    },
};
use tonic::Response;

// impl
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
            status: "Ok".to_string(),
            message: "Service up and running".to_string(),
        };
        Ok(Response::new(response))
    }

    async fn sign_up(
        &self,
        request: tonic::Request<SignupRequest>,
    ) -> std::result::Result<tonic::Response<SignUpResponse>, tonic::Status> {
        // get the payload
        let payload = request.into_inner();
        let new_user = UserInformationBuilder::new(&payload.email, &payload.password);

        // create the user
        let user = UserInformation::new(new_user).await;

        if !user.is_ok() {
            return Err(tonic::Status::internal("error creating user"));
        }
        //TODO: send a verification email to the user

        // build the response
        let response = SignUpResponse {
            success: true,
            message: "User created successfully".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn login(
        &self,
        _request: tonic::Request<LoginRequest>,
    ) -> std::result::Result<tonic::Response<LoginResponse>, tonic::Status> {
        todo!()
    }

    async fn logout(
        &self,
        _request: tonic::Request<LogoutRequest>,
    ) -> std::result::Result<tonic::Response<LogoutResponse>, tonic::Status> {
        todo!()
    }

    async fn refresh_token(
        &self,
        _request: tonic::Request<RefreshTokenRequest>,
    ) -> std::result::Result<tonic::Response<RefreshTokenResponse>, tonic::Status> {
        todo!()
    }

    async fn verify_token(
        &self,
        _request: tonic::Request<VerifyTokenRequest>,
    ) -> std::result::Result<tonic::Response<VerifyTokenResponse>, tonic::Status> {
        todo!()
    }

    async fn verify_email(
        &self,
        _request: tonic::Request<VerifyEmailRequest>,
    ) -> std::result::Result<tonic::Response<VerifyEmailResponse>, tonic::Status> {
        todo!()
    }

    async fn forgot_password(
        &self,
        _request: tonic::Request<ForgotPasswordRequest>,
    ) -> std::result::Result<tonic::Response<ForgotPasswordResponse>, tonic::Status> {
        todo!()
    }

    async fn reset_password(
        &self,
        _request: tonic::Request<ResetPasswordRequest>,
    ) -> std::result::Result<tonic::Response<ResetPasswordResponse>, tonic::Status> {
        todo!()
    }

    async fn change_password(
        &self,
        _request: tonic::Request<ChangePasswordRequest>,
    ) -> std::result::Result<tonic::Response<ChangePasswordResponse>, tonic::Status> {
        todo!()
    }
}
