use crate::endpoint::Endpoint;
use crate::err::RacetimeError;
use serde::Serialize;
use std::borrow::Cow;
use std::fmt::Display;

#[derive(serde::Serialize)]
pub struct UserSearch {
    term: String,
}

impl UserSearch {
    /// term is one of `name`, `#scrim`, or `name#scrim`
    /// the name search is always a startswith search (e.g. foo matches foobar)
    /// the scrim search is always exact
    pub fn from_term<S: Into<String>>(term: S) -> Self {
        Self {
            term: term.into()
        }
    }

    /// you have to provide at least one of the params. filtering is conjunctive if both are given.
    /// see [Self::from_term]
    pub fn from_name_and_scrim<S: Display>(name: Option<S>, scrim: Option<u32>) -> Result<Self, &'static str> {
        match (name, scrim) {
            (Some(n), Some(s)) =>
                Ok(Self::from_term(format!("{n}#{s:0>4}"))),

            (Some(n), None) => Ok(Self::from_term(n.to_string())),
            (None, Some(s)) => Ok(Self::from_term(format!("#{s:0>4}"))),
            (None, None) => Err("You must provide at least one of name and scrim")
        }
    }
}


#[derive(serde::Serialize)]
struct UserSearchParams {
    term: String,
}

impl Endpoint for UserSearch {
    fn endpoint(&self) -> Cow<'static, str> {
        "/user/search".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, RacetimeError> {
        serde_urlencoded::to_string(self)
            .map_err(From::from)
            .map(From::from)
    }
}
#[cfg(test)]
mod tests {
    use crate::endpoint::Endpoint;
    use crate::endpoints::{UserSearch, };
    #[test]
    fn test_term_name() {
        let us = UserSearch::from_term("fox");
        assert_eq!("term=fox", us.query_parameters().unwrap());
    }

    #[test]
    fn test_term_scrim() {
        let us = UserSearch::from_term("#0123");
        assert_eq!("term=%230123", us.query_parameters().unwrap());
    }

    #[test]
    fn test_name() {
        let us = UserSearch::from_name_and_scrim(Some("fox"), None).unwrap();
        assert_eq!("term=fox", us.query_parameters().unwrap());
    }


    #[test]
    fn test_scrim() {
        let us = UserSearch::from_name_and_scrim::<&str>(None, Some(0123)).unwrap();
        assert_eq!("term=%230123", us.query_parameters().unwrap());
    }

    #[test]
    fn test_name_and_scrim() {
        let us = UserSearch::from_name_and_scrim(Some("fox"), Some(0123)).unwrap();
        assert_eq!("term=fox%230123", us.query_parameters().unwrap());
    }

    #[test]
    fn test_neither() {
        let us = UserSearch::from_name_and_scrim::<&str>(None, None);
        assert!(us.is_err());
    }

}


