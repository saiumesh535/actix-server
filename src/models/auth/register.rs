use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegister {
    pub email: String,
    pub username: String,
    pub password: String,
}