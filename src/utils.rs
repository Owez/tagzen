//! Utility items

use regex::Regex;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::fmt;

/// Rgex pattern for converting special characters to spaces
const TO_SPACE_REGEX: &str = r"(\.|-| )+";

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

/// Attempts to capture `filename` used and `ext` used from a given `file_path`
/// by splitting
pub fn cap_filename_ext(file_path: impl AsRef<str>) -> (String, Option<String>) {
    let split: Vec<&str> = file_path.as_ref().split('.').collect();

    (
        split[..split.len() - 1].join(".").to_string(),
        if split.len() > 1 {
            Some(format!(".{}", split.last().unwrap()))
        } else {
            None
        },
    )
}

/// Formats name by elimintating non alphanumeric characters with the use of
/// regex and replacing characters with spaces
pub fn format_name(name: impl AsRef<str>) -> String {
    Regex::new(TO_SPACE_REGEX)
        .unwrap()
        .replace_all(name.as_ref(), " ")
        .as_ref().trim().to_string()
}
