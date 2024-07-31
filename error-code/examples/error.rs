use error_code::ToErrorInfo;

#[derive(Debug, thiserror::Error, ToErrorInfo)]
#[error_info(app_type = "http::StatusCode", prefix = "01")]
pub enum MyError {
    #[error("Invalid command: {0}")]
    #[error_info(code = "IC", app_code = "400")]
    InvalidCommand(String),

    #[error("Invalid argument: {0}")]
    #[error_info(code = "IA", app_code = "400", client_msg = "friendly msg")]
    InvalidArgument(String),

    #[error("{0}")]
    #[error_info(code = "RE", app_code = "500")]
    RespError(#[from] std::io::Error),
}

// use error_code::{ErrorInfo, ToErrorInfo as _};
// impl ToErrorInfo for MyError {
//     type T = StatusCode;
//     fn to_error_info(&self) -> Result<ErrorInfo<Self::T>, <Self::T as std::str::FromStr>::Err> {
//         match self {
//             MyError::InvalidCommand(_) => ErrorInfo::try_new("400", "01IC", "", self),
//             MyError::InvalidArgument(_) => ErrorInfo::try_new("400", "01IA", "friendly msg", self),
//             MyError::RespError(_) => ErrorInfo::try_new("500", "01RE", "", self),
//         }
//     }
// }

fn main() {
    let err = MyError::InvalidCommand("cmd".to_string());
    let info = err.to_error_info();
    println!("{:?}", info);
}
