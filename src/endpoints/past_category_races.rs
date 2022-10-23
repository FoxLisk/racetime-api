use crate::endpoint::Endpoint;
use crate::endpoints::is_false;
use crate::err::RacetimeError;
use serde::Serialize;
use std::borrow::Cow;

#[derive(serde::Serialize, Builder)]
pub struct PastCategoryRaces {
    #[builder(setter(into))]
    category: String,

    #[serde(skip_serializing_if = "is_false")]
    #[builder(default = "false")]
    show_entrants: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    page: Option<u32>,
}

impl Endpoint for PastCategoryRaces {
    fn endpoint(&self) -> Cow<'static, str> {
        format!("{}/races/data", self.category).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, RacetimeError> {
        serde_urlencoded::to_string(self)
            .map_err(From::from)
            .map(From::from)
    }
}
