use crate::base::{model::AuthCallback, AuthSource};
use crate::error::AuthError;

/**
 * 授权配置类的校验器
 */
pub struct AuthChecker {}

impl AuthChecker {
    /**
     * 校验回调传回的code
     * <p>
     * {@code v1.10.0}版本中改为传入{@code source}和{@code callback}，对于不同平台使用不同参数接受code的情况统一做处理
     *
     * @param source   当前授权平台
     * @param callback 从第三方授权回调回来时传入的参数集合
     */
    // pub fn check_code(source: AuthSource, callback: AuthCallback) -> Result<(), AuthError> {
        // 推特平台不支持回调 code 和 state
        // if (source == AuthDefaultSource.TWITTER) {
        //     return;
        // }
        // String code = callback.getCode();
        // if (source == AuthDefaultSource.HUAWEI) {
        //     code = callback.getAuthorization_code();
        // }

        // if (StringUtils.isEmpty(code)) {
        //     throw new AuthException(AuthResponseStatus.ILLEGAL_CODE, source);
        // }
    // }
}
