use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub access_toekn: String,
    pub expire_in: i64,
    pub refresh_token: String,
    pub refresh_token_expire_in: i64,
    pub uid: String,
    pub open_id: String,
    pub access_code: String,
    pub union_id: String,
}
