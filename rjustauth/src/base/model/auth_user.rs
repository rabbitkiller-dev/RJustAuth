use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    /**
     * 用户第三方系统的唯一id。在调用方集成该组件时，可以用uuid + source唯一确定一个用户
     */
    pub uuid: String,
    /**
     * 用户名
     */
    username: String,
    /**
     * 用户昵称
     */
    nickname: String,
    /**
     * 用户头像
     */
    avatar: String,
    /**
     * 用户网址
     */
    blog: String,
    /**
     * 所在公司
     */
    company: String,
    /**
     * 位置
     */
    location: String,
    /**
     * 用户邮箱
     */
    email: String,
    /**
     * 用户备注（各平台中的用户个人介绍）
     */
    remark: String,
    /**
     * 性别
     */
    // gender: AuthUserGender,
    /**
     * 用户来源
     */
    source: String,
    /**
     * 用户授权的token信息
     */
    // token: AuthToken,
    /**
     * 第三方平台返回的原始用户信息
     */
    // rawUserInfo: JSONObject,

    /**
     * 微信公众号 - 网页授权的登录时可用
     *
     * 微信针对网页授权登录，增加了一个快照页的逻辑，快照页获取到的微信用户的 uid oid 和头像昵称都是虚拟的信息
     */
    snapshotUser: bool,
}
