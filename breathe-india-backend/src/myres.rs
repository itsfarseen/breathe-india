use crate::slog_nested::WrapSerde;
use rocket::request::Request;
use rocket::response::Responder;
use std::error::Error;

pub enum MyRes<T, E> {
    Ok(T),
    Err(E),
    Fail(Box<dyn Error>),
}

impl<'r, T: Responder<'r, 'static>, E: Responder<'r, 'static>> Responder<'r, 'static>
    for MyRes<T, E>
{
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            MyRes::Ok(r) => r.respond_to(req),
            MyRes::Err(r) => r.respond_to(req),
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

                slog::error!(logger, "E500"; "method" => method, "route_name" => route_name, "uri" => uri, "error_chain" => WrapSerde(error_chain));
                Err(rocket::http::Status::InternalServerError)
            }
        }
    }
}

#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        match $e {
            Ok(ok) => ok,
            Err(err) => {
                return MyRes::Fail(err);
            }
        }
    };
}
