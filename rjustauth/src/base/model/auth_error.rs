/**
 * 验证相关的自定义错误包装
 */
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum AuthError {
    /**
     * 未知错误
     */
    UnknownError,
    /**
     * 未知错误
     */
    AuthServerError,
    /**
     * 未知错误
     */
    AuthClientError,
    /**
     * 未知错误
     */
    AuthServerErrorWithMessage(String),
    /**
     * 未知错误
     */
    AuthClientErrorWithMessage(String),
    /**
     * 未知错误
     */
    AuthServerErrorWithCode(String),
    /**
     * 未知错误
     */
    AuthClientErrorWithCode(String),
    /**
     * 未知错误
     */
    AuthServerErrorWithMessageAndCode(String, String),
    /**
     * 未知错误
     */
    AuthClientErrorWithMessageAndCode(String, String),
}
