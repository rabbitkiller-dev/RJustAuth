use serde::{Deserialize, Serialize};

/**
 * Github请求返回的错误信息
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubError {
    pub error: String,
    pub error_description: String,
}
