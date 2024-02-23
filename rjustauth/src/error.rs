use crate::base::enums::AuthResponseStatus;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct AuthError {
    pub code: i32,
    pub msg: String,
}

impl AuthError {
    pub fn new(code: i32, msg: String) -> Self {
        AuthError { code, msg }
    }

    pub fn from_auth_response_status(status: AuthResponseStatus) -> Self {
        AuthError {
            code: status.get_code(),
            msg: status.get_msg(),
        }
    }
}
