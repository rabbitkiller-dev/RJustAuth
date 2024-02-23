use std::collections::HashMap;

/**
 * JustAuth通用的状态码对照表
 */
pub enum AuthResponseStatus {
    /**
     * 2000：正常；
     * other：调用异常，具体异常内容见 self.get_msg()
     */
    Success,
    Failure,
    NotImplemented,
    ParameterIncomplete,
    Unsupported,
    NoAuthSource,
    UnidentifiedPlatform,
    IllegalRedirectUri,
    IllegalRequest,
    IllegalCode,
    IllegalStatus,
    RequiredRefreshToken,
    IllegalToken,
}

impl AuthResponseStatus {
    pub fn get_code(&self) -> i32 {
        match self {
            AuthResponseStatus::Success => 2000,
            AuthResponseStatus::Failure => 5000,
            AuthResponseStatus::NotImplemented => 5001,
            AuthResponseStatus::ParameterIncomplete => 5002,
            AuthResponseStatus::Unsupported => 5003,
            AuthResponseStatus::NoAuthSource => 5004,
            AuthResponseStatus::UnidentifiedPlatform => 5005,
            AuthResponseStatus::IllegalRedirectUri => 5006,
            AuthResponseStatus::IllegalRequest => 5007,
            AuthResponseStatus::IllegalCode => 5008,
            AuthResponseStatus::IllegalStatus => 5009,
            AuthResponseStatus::RequiredRefreshToken => 5010,
            AuthResponseStatus::IllegalToken => 5011,
        }
    }
    pub fn get_msg(&self) -> String {
        match self {
            AuthResponseStatus::Success => "Success".to_string(),
            AuthResponseStatus::Failure => "Failure".to_string(),
            AuthResponseStatus::NotImplemented => "Not Implemented".to_string(),
            AuthResponseStatus::ParameterIncomplete => "Parameter incomplete".to_string(),
            AuthResponseStatus::Unsupported => "Unsupported operation".to_string(),
            AuthResponseStatus::NoAuthSource => "AuthDefaultSource cannot be null".to_string(),
            AuthResponseStatus::UnidentifiedPlatform => "Unidentified platform".to_string(),
            AuthResponseStatus::IllegalRedirectUri => "Illegal redirect uri".to_string(),
            AuthResponseStatus::IllegalRequest => "Illegal request".to_string(),
            AuthResponseStatus::IllegalCode => "Illegal code".to_string(),
            AuthResponseStatus::IllegalStatus => "Illegal state".to_string(),
            AuthResponseStatus::RequiredRefreshToken => {
                "The refresh token is required; it must not be null".to_string()
            }
            AuthResponseStatus::IllegalToken => "Invalid token".to_string(),
        }
    }
}
