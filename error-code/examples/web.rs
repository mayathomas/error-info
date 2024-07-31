use axum::{response::IntoResponse, routing::get, Router};
use backtrace::Backtrace;
use error_code::ToErrorInfo;
use http::{Response, StatusCode};
use thiserror::Error;
use tracing::{info, warn};

#[allow(dead_code)]
#[derive(Debug, Error, ToErrorInfo)]
#[error_info(app_type = "StatusCode", prefix = "0A")]
enum AppError {
    #[error("Invalid param: {0}")]
    #[error_info(code = "IP", app_code = "400")]
    InvalidParam(String),

    #[error("Item {0} not found")]
    #[error_info(code = "NF", app_code = "404")]
    NotFound(String),

    #[error("Internal serve error: {0}")]
    #[error_info(
        code = "ISE",
        app_code = "500",
        client_msg = "we had a server problem, please try again later"
    )]
    ServerError(String),

    #[error("Unknown error")]
    #[error_info(code = "USE", app_code = "400")]
    Unknown,
}

// thiserror生成的
// #[allow(unused_qualifications)]
// impl std::error::Error for AppError {}
// #[allow(unused_qualifications)]
// impl ::core::fmt::Display for AppError {
//     fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//         use thiserror::__private::AsDisplay as _;
//         #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
//         match self {
//             AppError::InvalidParam(_0) => {
//                 __formatter
//                     .write_fmt(format_args!("Invalid param: {0}", _0.as_display()))
//             }
//             AppError::NotFound(_0) => {
//                 __formatter
//                     .write_fmt(format_args!("Item {0} not found", _0.as_display()))
//             }
//             AppError::ServerError(_0) => {
//                 __formatter
//                     .write_fmt(
//                         format_args!("Internal serve error: {0}", _0.as_display()),
//                     )
//             }
//             AppError::Unknown {} => __formatter.write_str("Unknown error"),
//         }
//     }
// }

// ToErrorInfo生成的
// use error_code::{ErrorInfo, ToErrorInfo as _};
// impl ToErrorInfo for AppError {
//     type T = StatusCode;
//     fn to_error_info(&self) -> ErrorInfo<Self::T> {
//         match self {
//             AppError::InvalidParam(_) => ErrorInfo::new("400", "0AIP", "", self),
//             AppError::NotFound(_) => ErrorInfo::new("404", "0ANF", "", self),
//             AppError::ServerError(_) => {
//                 ErrorInfo::new(
//                     "500",
//                     "0AISE",
//                     "we had a server problem, please try again later",
//                     self,
//                 )
//             }
//             AppError::Unknown => ErrorInfo::new("400", "0AUE", "", self),
//         }
//     }
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(index_handler));

    let addr = "0.0.0.0:8080";
    info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

async fn index_handler() -> Result<&'static str, AppError> {
    let bt = Backtrace::new();
    Err(AppError::ServerError(format!("{bt:?}")))
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let info = self.to_error_info();

        let status = info.app_code;

        //debug打印server_msg，display打印client_msg
        if status.is_server_error() {
            warn!("{:?}", info);
        } else {
            info!("{:?}", info);
        }

        // using client-facing message
        Response::builder()
            .status(status)
            .body(info.to_string().into())
            .unwrap()
    }
}
