use serde::Serialize;
use uuid::Uuid;
#[derive(Debug, Serialize)]

pub struct UserInformation {
    id: Uuid,
    username: String,
    password: String,
}
