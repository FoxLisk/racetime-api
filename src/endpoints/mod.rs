mod all_races;
mod past_category_races;
mod user_search;

pub use all_races::AllRaces;
pub use past_category_races::{PastCategoryRaces, PastCategoryRacesBuilder, PastCategoryRacesBuilderError};
pub use user_search::{UserSearch};

// this stupid function is for use with serde::skip_serializing_if

fn is_false(b: &bool) -> bool {
    !(b.clone())
}
