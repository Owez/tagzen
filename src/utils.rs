//! Utility items

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::fmt;

/// Regex pattern for alphanumeric-only regex characters
pub const ALPHANUMERIC_REGEX: &str = r"[^a-zA-z0-9]";

/// A template to respond to requests with, includes status code and message,
/// along with an optional `body` key that may contain anything (but should
/// ideally be as standard as possible)
#[derive(Debug, Serialize)]
pub struct ResponseModel<T: Serialize> {
    pub status: u16,
    pub msg: String,
    pub body: Option<T>,
}

impl<T: Serialize> ResponseModel<T> {
    /// Creates a new [ResponseModel] with all values filled out
    pub fn new(status: u16, msg: impl fmt::Display, body: T) -> Self {
        Self {
            status,
            msg: format!("{}", msg),
            body: Some(body),
        }
    }

    /// Creates a basic response without the `body` field
    pub fn basic(status: u16, msg: impl fmt::Display) -> Self {
        Self {
            status,
            msg: format!("{}", msg),
            body: None,
        }
    }
}

impl<'r, T: Serialize> Responder<'r> for ResponseModel<T> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(Json(&self).respond_to(req)?)
            .status(Status::from_code(self.status).unwrap())
            .ok()
    }
}
