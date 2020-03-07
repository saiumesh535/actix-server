use crate::utils::uuid_utils::get_uuid;
use argon2::{ self, Config, verify_encoded };

pub struct PasswordBcrypt {
    pub hash: String
}


pub fn get_password_hash(password: &String) -> PasswordBcrypt {
    let salt = get_uuid();
    let config: Config = Config::default();
    PasswordBcrypt {
        hash: argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
    }
}

pub fn verify_password(hash: &String, plain_password: &String) -> bool {
    verify_encoded(hash, plain_password.as_ref()).unwrap()
}