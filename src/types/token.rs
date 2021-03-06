use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginToken {
    pub id: i32,
    pub email: String,
}
