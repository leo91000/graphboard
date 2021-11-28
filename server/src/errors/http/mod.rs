pub mod postgres;
pub mod serde_qs;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use derive_more::{Display, Error};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Error, Display, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
#[display(fmt = "{}", self)]
pub struct HttpError {
    #[serde(skip_serializing)]
    http_code: StatusCode,
    err_code: String,
    data: Option<Value>,
}

impl HttpError {
    pub fn internal_server_error(err_code: &str, data: Option<Value>) -> HttpError {
        HttpError {
            http_code: StatusCode::INTERNAL_SERVER_ERROR,
            err_code: String::from(err_code),
            data,
        }
    }

    pub fn not_found(err_code: &str, data: Option<Value>) -> HttpError {
        HttpError {
            http_code: StatusCode::NOT_FOUND,
            err_code: String::from(err_code),
            data,
        }
    }

    pub fn bad_request(err_code: &str, data: Option<Value>) -> HttpError {
        HttpError {
            http_code: StatusCode::BAD_REQUEST,
            err_code: String::from(err_code),
            data,
        }
    }
}

impl From<&str> for HttpError {
    fn from(from: &str) -> Self {
        HttpError {
            http_code: StatusCode::INTERNAL_SERVER_ERROR,
            err_code: String::from(from),
            data: None,
        }
    }
}

impl From<HttpError> for HttpResponse {
    fn from(http_error: HttpError) -> Self {
        HttpResponseBuilder::new(http_error.http_code).json(http_error)
    }
}

impl From<&HttpError> for HttpResponse {
    fn from(http_error: &HttpError) -> Self {
        HttpResponseBuilder::new(http_error.http_code).json(http_error)
    }
}

impl actix_web::error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        self.into()
    }

    fn status_code(&self) -> StatusCode {
        self.http_code
    }
}
