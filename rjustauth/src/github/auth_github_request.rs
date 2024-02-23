use crate::base::model::AuthCallback;
use crate::base::model::AuthToken;
use crate::base::AuthRequest;

use builder_pattern::Builder;
use url_builder::URLBuilder;

use super::model::{GitHubError, GithubAccessToken, GithubCallback};

#[derive(Builder, Debug)]
pub struct AuthGithubRequest {
    /**
     * 客户端ID: 对应各平台的appKey
     */
    #[into]
    pub client_id: String,
    /**
     * 客户端Secret: 对应各平台的appSecret
     */
    #[into]
    pub client_secret: String,
    /**
     * 登录成功后的回调地址
     */
    #[into]
    pub redirect_uri: String,
}

impl AuthGithubRequest {
    pub fn base_url_builder(&self) -> URLBuilder {
        let mut lb: URLBuilder = URLBuilder::new();
        lb.set_protocol("https").set_host("github.com");
        lb
    }

    pub fn authorize<T: Into<String>>(&self, state: T) -> String {
        let mut lb: URLBuilder = self.base_url_builder();
        lb.add_route("login/oauth/authorize")
            .add_param("response_type", "code")
            .add_param("client_id", &self.client_id)
            .add_param("redirect_uri", &self.redirect_uri)
            .add_param("state", &state.into());
        lb.build()
    }

    pub fn access_token_url(&self, code: String) -> String {
        let mut lb: URLBuilder = self.base_url_builder();
        lb.add_route("login/oauth/access_token")
            .add_param("code", &code)
            .add_param("client_id", &self.client_id)
            .add_param("client_secret", &self.client_secret)
            .add_param("grant_type", "authorization_code")
            .add_param("redirect_uri", &self.redirect_uri);
        lb.build()
    }

    pub async fn login(&self, github_callback: GithubCallback) -> () {
        log::warn!("github login error -----------------------------------");
        tracing::info!("login");
        let auth_token = self.get_access_token(github_callback).await;
        let auth_token = auth_token.unwrap();
        tracing::info!("auth_token: {:?}", auth_token);
    }

    pub async fn get_access_token(&self, github_callback: GithubCallback) -> Result<GithubAccessToken, reqwest::Error> {
        // let auth_token = AuthToken {
        //     access_toekn: "".to_string(),
        //     expire_in: 0,
        //     refresh_token: "".to_string(),
        //     refresh_token_expire_in: 0,
        //     uid: "".to_string(),
        //     open_id: "".to_string(),
        //     access_code: "".to_string(),
        //     union_id: "".to_string(),
        // };
        tracing::info!("github_access_token prefix");
        let github_access_token = self.do_post_authorization_code(github_callback.code).await?;
        tracing::info!("github_access_to/*  */ken: {:?}", github_access_token);
        Ok(github_access_token)
    }

    async fn do_post_authorization_code(&self, code: String) -> Result<GithubAccessToken, reqwest::Error> {
        let resp = reqwest::get(self.access_token_url(code)).await?.text().await?;
        // 先序列化成GithubAccessToken, 失败则序列化成GithubError, 再失败则返回错误信息
        if let Ok(github_access_token) = serde_json::from_str::<GithubAccessToken>(&resp) {
            return Ok(github_access_token);
        } else if let Ok(github_error) = serde_json::from_str::<GitHubError>(&resp) {
            return Err(reqwest::Error::new(
                reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                github_error.error_description,
            ));
        } else {
            return Err(reqwest::Error::new(reqwest::StatusCode::INTERNAL_SERVER_ERROR, resp));
        }
    }
}

#[async_trait::async_trait]
impl AuthRequest for AuthGithubRequest {
    fn authorize<T: Into<String>>(&self, state: T) -> String {
        self.authorize(state)
    }

    fn access_token_url(&self, code: String) -> String {
        self.access_token_url(code)
    }

    async fn login(&self, auth_callback: AuthCallback) -> () {
        self.login(auth_callback.into()).await;
    }

    async fn get_access_token(&self, auth_callback: AuthCallback) -> AuthToken {
        let github_callback: GithubCallback = auth_callback.into();
        let auth_token = self.get_access_token(github_callback).await;
        let auth_token = AuthToken {
            access_toekn: "".to_string(),
            expire_in: 0,
            refresh_token: "".to_string(),
            refresh_token_expire_in: 0,
            uid: "".to_string(),
            open_id: "".to_string(),
            access_code: "".to_string(),
            union_id: "".to_string(),
        };
        auth_token
    }
}
