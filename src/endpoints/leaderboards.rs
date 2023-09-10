use crate::endpoint::Endpoint;
use crate::err::RacetimeError;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize)]
pub struct Leaderboards {
    category: String,

}

impl Leaderboards {
    pub fn new<S: Into<String>>(category: S) -> Self {
        Self {
            category: category.into()
        }
    }
}

impl Endpoint for Leaderboards {
    fn endpoint(&self) -> Cow<'static, str> {
        format!("{}/leaderboards/data", self.category).into()
    }

}
