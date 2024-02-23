use std::convert::From;
use serde::{Deserialize, Serialize};
use crate::base::model::AuthCallback;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubCallback {
    pub code: String,
    pub state: String,
}

impl From<GithubCallback> for AuthCallback {
    fn from(github_callback: GithubCallback) -> Self {
        AuthCallback {
            code: github_callback.code,
            state: github_callback.state,
        }
    }
}

impl From<AuthCallback> for GithubCallback {
    fn from(auth_callback: AuthCallback) -> Self {
        GithubCallback {
            code: auth_callback.code,
            state: auth_callback.state,
        }
    }
}
