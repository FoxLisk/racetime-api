use racetime_api::client::RacetimeClient;
use racetime_api::endpoint::Query;
use racetime_api::endpoints::AllRaces;
use racetime_api::types::Races;

#[tokio::main]
async fn main() {
    let client = RacetimeClient::new().unwrap();
    let all_races = AllRaces {};
    let something: Races = all_races.query(&client).await.unwrap();
    println!("{:?}", something);
}
