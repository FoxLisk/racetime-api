use racetime_api::client::RacetimeClient;
use racetime_api::endpoint::Query;
use racetime_api::endpoints::Leaderboards;
use racetime_api::types::LeaderboardsResult;

#[tokio::main]
async fn main() {
    let client = RacetimeClient::new().unwrap();
    let lbs_q = Leaderboards::new("alttp");
    let lbs: LeaderboardsResult = lbs_q.query(&client).await.unwrap();
    println!("{lbs:?}");
}