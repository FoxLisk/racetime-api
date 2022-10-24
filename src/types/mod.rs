/*!
Some types for deserializing rt.gg response data. These are provided for convenience, but you can
easily write your own deserialization targets.
*/
use serde::Deserialize;

/*
   {
     "name": "ff7/gnarly-waluigi-2626",
     "status": {
       "value": "in_progress",
       "verbose_value": "In progress",
       "help_text": "Race is in progress"
     },
     "url": "/ff7/gnarly-waluigi-2626",
     "data_url": "/ff7/gnarly-waluigi-2626/data",
     "goal": {
       "name": "No Major Glitches",
       "custom": false
     },
     "info": "The NMS Showdown!!!",
     "entrants_count": 9,
     "entrants_count_finished": 5,
     "entrants_count_inactive": 3,
     "opened_at": "2022-10-22T13:40:37.747Z",
     "started_at": "2022-10-22T14:45:42.204Z",
     "time_limit": "P1DT00H00M00S",
     "category": {
       "name": "Final Fantasy VII",
       "short_name": "FF7",
       "slug": "ff7",
       "url": "/ff7",
       "data_url": "/ff7/data",
       "image": "https://racetime.gg/media/cover-2561.png"
     }
*/

#[derive(Deserialize, Debug)]
pub struct Status {
    pub value: String,
    pub verbose_value: String,
    pub help_text: String,
}

#[derive(Deserialize, Debug)]
pub struct Goal {
    pub name: String,
    pub custom: bool,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    /// name with scrim (e.g. FoxLisk#8582)
    pub full_name: String,
    /// name without scrim (e.g. FoxLisk)
    pub name: String,
    /// scrim without name (e.g. 8582)
    pub discriminator: String,
    pub url: String,
    pub avatar: Option<String>,
    pub pronouns: Option<String>,
    /// note this is actually a space-separated list
    pub flair: String,
    pub twitch_name: Option<String>,
    pub twitch_channel: Option<String>,
    pub can_moderate: bool,
}

#[derive(Deserialize, Debug)]
/// note: i'm omitting Team in part because it's not documented and I don't have one handy
pub struct PastRaceEntrant {
    pub status: Status,
    pub finish_time: Option<String>,
    pub finished_at: Option<String>,
    pub place: Option<u32>,
    pub place_ordinal: Option<String>,
    /// score when the user _entered_ the race
    pub score: Option<u32>,
    /// null until recorded
    pub score_change: Option<i32>,
    pub comment: Option<String>,
    pub has_comment: bool,
    pub stream_override: bool,
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct PartialCategory {
    pub name: String,
    pub short_name: String,
    pub slug: String,
    pub url: String,
    pub data_url: String,
    pub image: String,
}

#[derive(Deserialize, Debug)]
pub struct Race {
    pub name: String,
    pub status: Status,
    pub url: String,
    pub data_url: String,
    pub goal: Goal,
    pub info: String,
    pub entrants_count: u16,
    pub entrants_count_finished: u16,
    pub entrants_count_inactive: u16,
    pub opened_at: String,
    pub started_at: Option<String>,
    pub time_limit: String,
}

#[derive(Deserialize, Debug)]
pub struct RaceWithPartialCategory {
    #[serde(flatten)]
    pub race: Race,
    pub category: PartialCategory,
}

#[derive(Deserialize, Debug)]
/// This is suitable for use as Races<Race> or Races<RaceWithPartialCategory>,
/// depending on endpoint
pub struct Races<T> {
    pub races: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct RacesPaginated<T> {
    pub count: u32,
    pub num_pages: u32,
    #[serde(flatten)]
    pub races: Races<T>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RaceWithEntrants {
    #[serde(flatten)]
    pub race: Race,
    pub entrants: Vec<PastRaceEntrant>,
}

#[cfg(test)]
mod tests {
    use crate::types::{Race, RaceWithEntrants, RaceWithPartialCategory, Races, RacesPaginated};
    use std::fs::read_to_string;

    #[test]
    fn test_deserialize_all_races() {
        let json_blob = read_to_string("test_data/all_races_01.json").unwrap();
        let races: Result<Races<Race>, _> = serde_json::from_str(&json_blob);
        assert!(races.is_ok());
    }

    #[test]
    fn test_deserialize_flatten() {
        let json_blob = read_to_string("test_data/all_races_01.json").unwrap();
        let races: Result<Races<RaceWithPartialCategory>, _> = serde_json::from_str(&json_blob);
        assert!(races.is_ok());
        let r = races.unwrap();
    }

    #[test]
    fn test_deserialize_races_with_past_entrants() {
        let json_blob = read_to_string("test_data/races_with_past_entrants.json").unwrap();
        let parsed_blob: serde_json::Value = serde_json::from_str(&json_blob).unwrap();
        let races = parsed_blob.get("races").unwrap();
        for race in races.as_array().unwrap() {
            let re_unparsed = serde_json::to_string(race).unwrap();
            let rwe: Result<RaceWithEntrants, _> = serde_json::from_str(&re_unparsed);
            assert!(rwe.is_ok(), "{}", re_unparsed);
        }

        let races: Result<RacesPaginated<RaceWithEntrants>, _> = serde_json::from_str(&json_blob);
        assert!(races.is_ok(), "{:?}", races);
    }
}
