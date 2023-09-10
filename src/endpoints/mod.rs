mod all_races;
mod past_category_races;
mod user_search;
mod leaderboards;

pub use all_races::AllRaces;
pub use past_category_races::{PastCategoryRaces, PastCategoryRacesBuilder, PastCategoryRacesBuilderError};
pub use user_search::{UserSearch};
pub use leaderboards::Leaderboards;

// this stupid function is for use with serde::skip_serializing_if

fn is_false(b: &bool) -> bool {
    !(b.clone())
}
