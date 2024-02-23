use serde::{Deserialize, Serialize};

pub trait AuthCallbackTrait {
    fn get_code(&self) -> String;
    fn get_state(&self) -> String;
}

/**
 * 授权回调时的参数类
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCallback {
    /**
     * 访问AuthorizeUrl后回调时带的参数code
     */
    pub code: String,

    /**
     * 访问AuthorizeUrl后回调时带的参数state，用于和请求AuthorizeUrl前的state比较，防止CSRF攻击
     */
    pub state: String,
}

impl AuthCallbackTrait for AuthCallback {
    fn get_code(&self) -> String {
        self.code.clone()
    }

    fn get_state(&self) -> String {
        self.state.clone()
    }
}
