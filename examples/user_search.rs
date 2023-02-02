use racetime_api::client::RacetimeClient;
use racetime_api::endpoint::Query;
use racetime_api::endpoints::UserSearchBuilder;
use racetime_api::types::{UserSearchResult};

#[tokio::main]
async fn main() {
    let client = RacetimeClient::new().unwrap();
    let user_search = UserSearchBuilder::default().name("fox").build().unwrap();
    let users: UserSearchResult = user_search.query(&client).await.unwrap();
    println!("{users:?}");
}