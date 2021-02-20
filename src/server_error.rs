use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct ServerError<'a> {
    pub message: &'a str,
}
