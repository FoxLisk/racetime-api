use crate::endpoint::Endpoint;
use std::borrow::Cow;

pub struct AllRaces {}

impl Endpoint for AllRaces {
    fn endpoint(&self) -> Cow<'static, str> {
        Cow::from("/races/data")
    }
}
