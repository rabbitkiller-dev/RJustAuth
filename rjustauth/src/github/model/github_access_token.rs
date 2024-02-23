use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubAccessToken {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}
