use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use core::fmt;
pub use error_code_derive::ToErrorInfo;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    str::FromStr,
};

pub struct ErrorInfo<T> {
    pub app_code: T,        //could be HTTP 400 bad request
    pub code: &'static str, //something like "01E739"
    pub hash: String,
    pub client_msg: &'static str,
    pub server_msg: String,
}

pub trait ToErrorInfo {
    type T: FromStr;
    fn to_error_info(&self) -> ErrorInfo<Self::T>;
}

impl<T> ErrorInfo<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug, //expect 和 unwrap都需要这个Err type实现Debug
{
    pub fn new(
        app_code: &str,
        code: &'static str,
        client_msg: &'static str,
        server_msg: impl fmt::Display,
    ) -> Self {
        //server_msg传入的是enum，thiserror已经自动实现了Display，这里的server_msg就是thiserror的error，把它直接用作server_msg
        let server_msg = server_msg.to_string();
        let mut hasher = DefaultHasher::new();
        server_msg.hash(&mut hasher);
        let hash = hasher.finish();
        let hash = URL_SAFE_NO_PAD.encode(hash.to_be_bytes());
        Self {
            app_code: T::from_str(app_code).expect("Can not parse app_code"),
            code,
            hash,
            client_msg,
            server_msg,
        }
    }
}

impl<T> ErrorInfo<T> {
    pub fn client_msg(&self) -> &str {
        if self.client_msg.is_empty() {
            &self.server_msg
        } else {
            self.client_msg
        }
    }
}

// Display: for client facing error message
impl<T> fmt::Display for ErrorInfo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{}] {}", self.code, self.hash, self.client_msg())
    }
}

impl<T> fmt::Debug for ErrorInfo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{}] {}", self.code, self.hash, self.server_msg)
    }
}
