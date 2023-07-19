use actix_web::HttpResponse;
use std::fmt::Debug;

pub fn handle_request<F>(f: F) -> HttpResponse
where
    F: FnOnce() -> Result<HttpResponse, HttpResponse>,
{
    (f()).unwrap_or_else(|e| e)
}

pub fn log_request_no_body<Args>(handler: &str, args: &Args)
where
    Args: Debug,
{
    log::debug!("{handler};Args={args:?};Body=None");
}

pub fn log_request<Args, Body>(handler: &str, args: &Args, body: &Body)
where
    Args: Debug,
    Body: Debug,
{
    log::debug!("{handler};Args={args:?};Body={body:?}");
}
