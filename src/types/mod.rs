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
    id: String,
    full_name: String,
    name: String,
    discriminator: String,
    url: String,
    avatar: Option<String>,
    pronouns: Option<String>,
    /// note this is actually a space-separated list
    flair: String,
    twitch_name: Option<String>,
    twitch_channel: Option<String>,
    can_moderate: bool,
}

/*
{
      "team": null,
      "status": {
        "value": "done",
        "verbose_value": "Finished",
        "help_text": "Finished the race."
      },
      "finish_time": "P0DT01H29M21.729469S",
      "finished_at": "2022-10-23T04:03:17.923Z",
      "place": 1,
      "place_ordinal": "1st",
      "score": 1586,
      "score_change": null,
      "comment": null,
      "has_comment": false,
      "stream_live": true,
      "stream_override": false,
      "actions": [
        "add_comment"
      ]
    },
 */
#[derive(Deserialize, Debug)]
/// note: i'm omitting Team in part because it's not documented and I don't have one handy
pub struct PastRaceEntrant {
    status: Status,
    finish_time: Option<String>,
    finished_at: Option<String>,
    place: Option<u32>,
    place_ordinal: Option<String>,
    /// score when the user _entered_ the race
    score: Option<u32>,
    /// null until recorded
    score_change: Option<i32>,
    comment: Option<String>,
    has_comment: bool,
    stream_override: bool,
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
