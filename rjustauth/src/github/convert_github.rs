use crate::base::model::AuthCallback;

use super::model::GithubCallback;

pub struct ConvertGithub;

impl ConvertGithub {
    pub fn auth_callback(github_callback: GithubCallback) -> AuthCallback {
        AuthCallback {
            code: github_callback.code,
            state: github_callback.state,
        }
    }
}
