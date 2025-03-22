use std::borrow::Cow;

use serde::Serialize;

use crate::endpoint::Endpoint;

#[derive(Serialize)]
pub struct UserData {
    id: String,
}

impl UserData {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self { id: id.into() }
    }
}

impl Endpoint for UserData {
    fn endpoint(&self) -> Cow<'static, str> {
        format!("user/{}/data", self.id).into()
    }
}
