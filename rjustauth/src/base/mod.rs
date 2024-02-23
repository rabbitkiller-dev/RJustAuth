pub mod enums;
pub mod model;

pub enum AuthSource {
    GITHUB,
}

use model::AuthCallback;
use model::AuthToken;

use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait AuthRequest {
    /**
     * 返回带{@code state}参数的授权url，授权回调时会带上这个{@code state}
     *
     * @param state state 验证授权流程的参数，可以防止csrf
     * @return 返回授权地址
     */
    fn authorize<T: Into<String>>(&self, state: T) -> String;

    /**
     * 返回获取accessToken的url
     *
     * @param code 授权码
     * @return 返回获取accessToken的url
     */
    fn access_token_url(&self, code: String) -> String;

    /**
     * 统一的登录入口。当通过{@link AuthDefaultRequest#authorize(String)}授权成功后，会跳转到调用方的相关回调方法中
     * 方法的入参可以使用{@code AuthCallback}，{@code AuthCallback}类中封装好了OAuth2授权回调所需要的参数
     *
     * @param authCallback 用于接收回调参数的实体
     * @return AuthResponse
     */
    async fn login(&self, auth_callback: AuthCallback) -> ();

    /**
     * 获取access token
     *
     * @param authCallback 授权成功后的回调参数
     * @return token
     * @see AuthDefaultRequest#authorize()
     * @see AuthDefaultRequest#authorize(String)
     */
    async fn get_access_token(&self, auth_callback: AuthCallback) -> AuthToken;

    async fn do_post_authorization_code(&self, code: String) -> Result<Value, reqwest::Error> {
        // let req = Request::post("http://example.com")
        //     .body(Body::from("hello world"))
        //     .unwrap();
        // let res = client.request(req).unwrap();
        // return new HttpUtils(config.getHttpConfig()).post(accessTokenUrl(code)).getBody();

        let resp = reqwest::get(self.access_token_url(code)).await?.json::<Value>().await?;

        Ok(resp)
    }
}

pub enum AuthSource2 {
    Github { auth_request: String },
}
