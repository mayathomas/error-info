pub struct ErrorInfo<T> {
    pub app_code: T,  //could be HTTP 400 bad request
    pub code: String, //something like "01E739"
    pub client_msg: String,
    pub server_msg: String,
}

pub trait ToErrorInfo {
    type T;
    fn to_error_info(&self) -> ErrorInfo<Self::T>;
}
