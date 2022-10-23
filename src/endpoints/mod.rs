mod all_races;
mod past_category_races;

pub use all_races::AllRaces;
pub use past_category_races::{PastCategoryRaces, PastCategoryRacesBuilder};

// this stupid function is for use with serde::skip_serializing_if

fn is_false(b: &bool) -> bool {
    !(b.clone())
}
