use crate::{
    database::{UserInformation, UserInformationBuilder},
    jwt::{Claim, Jwt},
    mailer::{EmailTemplate, Mailer},
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

        //see if creds exist
        let account_exist = UserInformation::creds_exists(&payload.email).await.unwrap();
        if account_exist {
            return Err(tonic::Status::already_exists("email already exist"));
        }

        // create the user
        let new_user = UserInformationBuilder::new(&payload.email, &payload.password);
        let user: Result<UserInformation, anyhow::Error> = UserInformation::new(new_user).await;

        if !user.is_ok() {
            return Err(tonic::Status::internal("error creating user"));
        }

        // send a verification email to the user
        let _ = Mailer::new(&payload.email, EmailTemplate::Signup)
            .send()
            .await
            .unwrap();

        // build the response
        let user = user.unwrap();
        let claim = Claim {
            id: user.id.to_string(),
            email: user.email,
        };
        let jwt = Jwt::new(claim).sign().await;
        let response = SignUpResponse {
            success: true,
            message: "User created successfully".to_string(),
            token: jwt.unwrap(),
        };

        Ok(Response::new(response))
    }

    async fn login(
        &self,
        request: tonic::Request<LoginRequest>,
    ) -> std::result::Result<tonic::Response<LoginResponse>, tonic::Status> {
        let LoginRequest { email, password } = request.into_inner();

        // validate the user credentials
        let user = UserInformation::fetch(&email).await;
        if !user.is_ok() {
            return Err(tonic::Status::already_exists(
                "no user with proved credentials was found",
            ));
        }

        let user = user.unwrap();
        let Some(is_correct_password) = user.validate_password(&password).await.ok() else {
            return Err(tonic::Status::invalid_argument(
                "invalid username or password",
            ));
        };

        if !is_correct_password {
            return Err(tonic::Status::invalid_argument(
                "invalid username or password",
            ));
        }

        // sign the JWT
        let claim = Claim {
            id: user.id.to_string(),
            email: user.email,
        };
        let jwt = Jwt::new(claim).sign().await;
        let response = LoginResponse {
            success: true,
            message: "user successfully logged in".to_string(),
            token: jwt.unwrap(),
        };

        Ok(Response::new(response))
    }

    async fn logout(
        &self,
        request: tonic::Request<LogoutRequest>,
    ) -> std::result::Result<tonic::Response<LogoutResponse>, tonic::Status> {
        let payload = request.into_inner();
        let user_data = Jwt::decode(&payload.token).ok();

        if user_data.is_none() {
            return Err(tonic::Status::unauthenticated(
                "invalid or expired authorization token",
            ));
        }

        let _ = UserInformation::logout(&payload.token).await;
        let response = LogoutResponse {
            success: true,
            message: "user successfully logged out".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn refresh_token(
        &self,
        request: tonic::Request<RefreshTokenRequest>,
    ) -> std::result::Result<tonic::Response<RefreshTokenResponse>, tonic::Status> {
        let payload = request.into_inner();

        let user_data = Jwt::decode(&payload.token).ok();
        if user_data.is_none() {
            return Err(tonic::Status::aborted(
                "invalid or expired JWT, please login to continue",
            ));
        }

        let user_data = user_data.unwrap();
        let refresh_token = Jwt::new(Claim {
            email: user_data.email,
            id: user_data.id,
        })
        .sign()
        .await
        .ok();

        if refresh_token.is_none() {
            return Err(tonic::Status::internal(
                "something unexpected happened, please try again after some time",
            ));
        }

        let refresh_token = refresh_token.unwrap();
        let response = RefreshTokenResponse {
            success: true,
            message: "successfully generate a new refresh token".to_string(),
            token: refresh_token,
        };

        Ok(Response::new(response))
    }

    async fn verify_token(
        &self,
        _request: tonic::Request<VerifyTokenRequest>,
    ) -> std::result::Result<tonic::Response<VerifyTokenResponse>, tonic::Status> {
        todo!()
    }

    async fn verify_email(
        &self,
        request: tonic::Request<VerifyEmailRequest>,
    ) -> std::result::Result<tonic::Response<VerifyEmailResponse>, tonic::Status> {
        let payload = request.into_inner();
        let user_data = Jwt::decode(&payload.token).ok();

        if user_data.is_none() {
            return Err(tonic::Status::unauthenticated(
                "invalid or expired authorization token",
            ));
        }

        let user_data = user_data.unwrap();
        let _ = UserInformation::set_verified(&user_data.email).await;

        let response = VerifyEmailResponse {
            success: true,
            message: "account successfully verified".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn forgot_password(
        &self,
        request: tonic::Request<ForgotPasswordRequest>,
    ) -> std::result::Result<tonic::Response<ForgotPasswordResponse>, tonic::Status> {
        let payload = request.into_inner();

        let user_data = UserInformation::fetch(&payload.email).await.ok();
        if user_data.is_none() {
            return Err(tonic::Status::not_found(
                "user with the provided email was not found",
            ));
        }

        let _ = Mailer::new(&payload.email, EmailTemplate::ForgottenPassword)
            .send()
            .await
            .unwrap();

        let response = ForgotPasswordResponse {
            success: true,
            message: "see email for further instructions".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn reset_password(
        &self,
        request: tonic::Request<ResetPasswordRequest>,
    ) -> std::result::Result<tonic::Response<ResetPasswordResponse>, tonic::Status> {
        let payload = request.into_inner();
        let user_data = Jwt::decode(&payload.token).ok();

        if user_data.is_none() {
            return Err(tonic::Status::internal("invalid username or password"));
        }
        let user_data = user_data.unwrap();

        let user = UserInformation::fetch(&user_data.email).await;
        if !user.is_ok() {
            return Err(tonic::Status::internal(
                "no user with proved credentials was found",
            ));
        }
        let response = ResetPasswordResponse {
            success: true,
            message: "please see your email for further instruction".to_string(),
        };
        Ok(Response::new(response))
    }

    async fn change_password(
        &self,
        request: tonic::Request<ChangePasswordRequest>,
    ) -> std::result::Result<tonic::Response<ChangePasswordResponse>, tonic::Status> {
        let payload = request.into_inner();
        let user_data = Jwt::decode(&payload.token).ok();

        if user_data.is_none() {
            return Err(tonic::Status::internal("invalid username or password"));
        }
        let user_data = user_data.unwrap();

        let user = UserInformation::fetch(&user_data.email).await;
        if !user.is_ok() {
            return Err(tonic::Status::internal(
                "no user with proved credentials was found",
            ));
        }

        // compare the strings
        if &payload.new_password != &payload.confirm_password {
            return Err(tonic::Status::internal("password does not match"));
        }

        let new_password = payload.new_password.trim();
        let _ = UserInformation::change_password(&user_data.email, new_password).await;

        let response = ChangePasswordResponse {
            success: true,
            message: "password updates successfully".to_string(),
        };

        Ok(Response::new(response))
    }
}
