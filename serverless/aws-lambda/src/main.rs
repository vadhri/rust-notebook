#[macro_use]
extern crate lambda_runtime as lambda;
use serde_derive::{Serialize, Deserialize};
use simple_logger;
use data_encoding::BASE64;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use] extern crate failure;

use lambda_runtime::{error::HandlerError, lambda, Context};
use std::error::Error;
use std::str;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Op {
    Encode,
    Decode
}

#[derive(Serialize, Deserialize, Clone)]
struct Request {
    op: Op,
    string: String
}

#[derive(Serialize, Deserialize, Clone)]
struct Response {
    output: String,
}

#[derive(Debug, Fail)]
enum Errors {
    #[fail(display = "Invalid input string")]
    InvalidInputString,
    #[fail(display = "Unknown error")]
    OperationError
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(encode_decode_base64);

    Ok(())
}

fn encode_decode_base64(e: Request, c: lambda::Context) -> Result<Response, HandlerError> {
    if e.op == Op::Encode {
        info!("Request to encode {}", c.aws_request_id);
        match e.string.len() {
            0 => Err(HandlerError::from(failure::Error::from(Errors::InvalidInputString))),
            _ => {
                Ok(Response {
                    output: BASE64.encode(e.string.as_bytes()),
                })
            }
        }
    } else {
        info!("Request to decode {}", c.aws_request_id);

        match e.string.len() {
            0 => Err(HandlerError::from(failure::Error::from(Errors::InvalidInputString))),
            _ => {
                Ok(Response {
                    output: String::from_utf8(BASE64.decode(e.string.as_bytes()).unwrap()).unwrap()
                })
            }
        }
    }
}
