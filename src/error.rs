use actix_web::HttpResponse;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    DieselError(diesel::result::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DieselError(e) => e.fmt(f),
        }
    }
}

pub trait InternalErrorWrapper<T> {
    fn internal(self) -> Result<T, HttpResponse>;
}

impl<T> InternalErrorWrapper<T> for Result<T, Error> {
    fn internal(self) -> Result<T, HttpResponse> {
        self.map_err(|_| HttpResponse::InternalServerError().finish())
    }
}
impl<T> InternalErrorWrapper<T> for Option<T> {
    fn internal(self) -> Result<T, HttpResponse> {
        self.ok_or_else(|| HttpResponse::InternalServerError().finish())
    }
}
