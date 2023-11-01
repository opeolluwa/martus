use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
/// the jwt module provides helpers amd mechanism
/// for encrypting and decrypting json web token
/// it uses EdSCA algorithm
#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub claim: Claim,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Claim {
    /// the user id, a UUId
    pub id: String,
    /// the user email
    pub email: String,
}

impl Jwt {
    /// take new a new user credential and build the claims
    pub fn new(claim: Claim) -> Self {
        let claim = Claim {
            email: claim.email,
            id: claim.id.to_string(),
        };
        Self { claim }
    }

    pub async fn sign(&self) -> Result<String> {
        let jtw_secret = env::var("JWT_SECRET")?;
        let algorithm = Algorithm::HS512;
        let header = Header::new(algorithm);
        let jwt_token = encode(
            &header,
            self,
            &EncodingKey::from_secret(jtw_secret.as_bytes()),
        );
        Ok(jwt_token?)
    }

    // decode the jwt
    pub async fn decode(token: &str) -> Result<Claim> {
        let jtw_secret = env::var("JWT_SECRET")?;
        let token = decode::<Claim>(
            &token,
            &DecodingKey::from_secret(jtw_secret.as_ref()),
            &Validation::default(),
        )?;
        Ok(token.claims)
    }
}

// tests