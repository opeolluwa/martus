#[derive(Debug)]
pub struct Jwt {
    pub claim: Claim,
}
#[derive(Debug)]
pub struct Claim {}

impl Jwt {
    pub fn new(claim: Claim) -> Self {
        Self { claim }
    }

    pub async fn sign(&self) -> String {
        "82f9f4b5-06d8-5ff7-a178-2ce319f9a646".to_string()
    }
}
