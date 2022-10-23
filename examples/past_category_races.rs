use racetime_api::client::RacetimeClient;
use racetime_api::endpoint::Query;
use racetime_api::endpoints::{PastCategoryRaces, PastCategoryRacesBuilder};
use racetime_api::types::{PastRaceEntrant, Race, RaceWithEntrants, RacesPaginated};

#[tokio::main]
async fn main() {
    let client = RacetimeClient::new().unwrap();
    let past_category_races: PastCategoryRaces = PastCategoryRacesBuilder::default()
        .category("alttp")
        .build()
        .unwrap();
    let past_races: RacesPaginated<Race> = past_category_races.query(&client).await.unwrap();
    println!("{:?}", past_races);

    let past_category_races_with_entrants: PastCategoryRaces = PastCategoryRacesBuilder::default()
        .category("alttp")
        .show_entrants(true)
        .build()
        .unwrap();

    let past_races_with_entrants: RacesPaginated<RaceWithEntrants> =
        past_category_races_with_entrants
            .query(&client)
            .await
            .unwrap();
    println!("{:?}", past_races_with_entrants);
}
