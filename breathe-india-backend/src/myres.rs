use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Responder;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::error::Error;

use crate::slog_nested::WrapSerde;

pub trait HasStatusCode {
    fn get_status(&self) -> Status;
}

pub enum MyRes<T, E> {
    Ok(T),
    Err(E),
    Fail(Box<dyn Error>),
}

impl<'r, T: Serialize, E: Serialize + HasStatusCode> Responder<'r, 'static> for MyRes<T, E> {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            MyRes::Ok(r) => Json(r).respond_to(req),
            MyRes::Err(r) => (r.get_status(), Json(r)).respond_to(req),
            MyRes::Fail(e) => {
                let logger = crate::LOGGER.get().unwrap();
                // todo!();
                let route_name = req
                    .route()
                    .and_then(|r| r.name.as_ref().map(|r| r.to_string()))
                    .unwrap_or_default()
                    .to_string();
                let uri = req.uri().to_string();
                let method = req.method().to_string();

                let error_chain: Vec<_> = e.chain().map(|e| e.to_string()).collect();
                let error_chain = WrapSerde(error_chain);

                slog::error!(logger, "E500"; "method" => method, "route_name" => route_name, "uri" => uri, "error_chain" => error_chain);
                Err(rocket::http::Status::InternalServerError)
            }
        }
    }
}

#[macro_export]
macro_rules! fail {
    ($e:expr) => {
        match $e {
            Ok(ok) => ok,
            Err(err) => {
                return MyRes::Fail(err.into());
            }
        }
    };
}

#[macro_export]
macro_rules! bail {
    ($e:expr, $f: expr) => {
        match $e {
            Ok(ok) => ok,
            Err(err) => {
                return MyRes::Err($f(err));
            }
        }
    };
}
