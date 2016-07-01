//! Challonge Tournament type.

extern crate serde_json;

use serde_json::Value;
use chrono::*;
use std::fmt;
use std::str::FromStr;
use std::collections::BTreeMap;

use error::Error;


fn into_map(value: Value) -> Result<BTreeMap<String, serde_json::Value>, Error> {
    match value {
        Value::Object(m) => Ok(m),
        value => Err(Error::Decode("Expected object", value)),
    }
}

fn remove(map: &mut BTreeMap<String, Value>, key: &str) -> Result<Value, Error> {
    map.remove(key).ok_or(Error::Decode("Unexpected absent key", Value::String(key.into())))
}

/// Tournament ranking order.
#[derive(Debug, Clone)]
pub enum RankedBy {
    MatchWins,
    GameWins,
    PointsScored,
    PointsDifference,
    Custom,
}
impl fmt::Display for RankedBy {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &RankedBy::MatchWins => {
                try!(fmt.write_str("match wins"));
            },
            &RankedBy::GameWins => {
                try!(fmt.write_str("game wins"));
            },
            &RankedBy::PointsScored => {
                try!(fmt.write_str("points scored"));
            },
            &RankedBy::PointsDifference => {
                try!(fmt.write_str("points difference"));
            },
            &RankedBy::Custom => {
                try!(fmt.write_str("custom"));
            },
        }
        Ok(())
    }
}

/// Tournament ID is an integer value or pair of strings (subdomain and tournament url)
#[derive(Debug, Clone, PartialEq)]
pub enum TournamentId {
    Url(String, String),
    Id(u64)
}
impl fmt::Display for TournamentId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TournamentId::Url(ref subdomain, ref tournament_url) => {
                try!(fmt.write_str(&format!("{}-{}", subdomain, tournament_url)));
            },
            &TournamentId::Id(ref id) => {
                try!(fmt.write_str(&id.to_string()));
            },
        }
        Ok(())
    }
}

/// Structure for creating a tournament.
#[derive(Debug, Clone)]
pub struct TournamentCreate {

    /// Your event's name/title (Max: 60 characters)
    pub name: String,
    pub tournament_type: TournamentType,

    /// challonge.com/url (letters, numbers, and underscores only)
    pub url: String,

    /// subdomain.challonge.com/url (Requires write access to the specified subdomain)
    pub subdomain: String,

    /// Description/instructions to be displayed above the bracket
    pub description: String,

    /// Have Challonge host a sign-up page (otherwise, you manually add all participants)
    pub open_signup: bool,

    /// Single Elimination only
    pub hold_third_place_match: bool,

    /// Only for Swiss system
    pub pts_for_match_win: f64,
    pub pts_for_match_tie: f64,
    pub pts_for_game_win: f64,
    pub pts_for_game_tie: f64,
    pub pts_for_bye: f64,
    pub swiss_rounds: u64,
    pub ranked_by: RankedBy,

    /// Only for Round Robin system
    pub rr_pts_for_match_win: f64,
    pub rr_pts_for_match_tie: f64,
    pub rr_pts_for_game_win: f64,
    pub rr_pts_for_game_tie: f64,

    /// Single &amp; Double Elimination only - Label each round above the bracket (default: false)
    pub show_rounds: bool,

    /// Hide this tournament from the public browsable index and your profile (default: false)
    pub private: bool,

    /// Email registered Challonge participants when matches open up for them (default: false)
    pub notify_users_when_matches_open: bool,

    /// Email registered Challonge participants the results when this tournament ends (default: false)
    pub notify_users_when_the_tournament_ends: bool,

    /// Instead of traditional seeding rules, make pairings by going straight down the list of participants.
    /// First round matches are filled in top to bottom, then qualifying matches (if applicable). (default: false)
    pub sequential_pairings: bool,

    /// Maximum number of participants in the bracket.
    /// A waiting list (attribute on Participant) will capture participants once the cap is reached.
    pub signup_cap: u64,

    /// the planned or anticipated start time for the tournament (Used with check_in_duration to determine participant check-in window). Timezone defaults to Eastern.
    pub start_at: DateTime<UTC>,

    /// Length of the participant check-in window in minutes.
    pub check_in_duration: u64,

    /// This option only affects double elimination. null/blank (default) - give the winners bracket finalist two chances to beat the losers bracket finalist, 'single match' - create only one grand finals match, 'skip' - don't create a finals match between winners and losers bracket finalists 
    pub grand_finals_modifier: Option<String>,
}

/// Challonge `Tournament` definition.
#[derive(Debug, Clone)]
pub struct Tournament {
    pub accept_attachments: bool,
    pub allow_participant_match_reporting: bool,
    pub anonymous_voting: bool,
    // category: ??,
    // check_in_duration: ??,
    // completed_at: ??,
    pub created_at: DateTime<FixedOffset>,
    pub created_by_api: bool,
    pub credit_capped: bool,
    pub description: String,
    pub game_id: u64,
    pub group_stages_enabled: bool,
    pub hide_forum: bool,
    pub hide_seeds: bool,
    pub hold_third_place_match: bool,
    pub id: u64,
    pub max_predictions_per_user: u64,
    pub name: String,
    pub notify_users_when_matches_open: bool,
    pub notify_users_when_the_tournament_ends: bool,
    pub open_signup: bool,
    pub participants_count: u64,
    pub prediction_method: u64,
    // <predictions-opened-at nil="true"/>
    pub private: bool,
    pub progress_meter: u64,
    pub pts_for_bye: f64, //>1.0</pts-for-bye>
    pub pts_for_game_tie: f64, //>0.0</pts-for-game-tie>
    pub pts_for_game_win: f64, //>0.0</pts-for-game-win>
    pub pts_for_match_tie: f64, //>0.5</pts-for-match-tie>
    pub pts_for_match_win: f64, //>1.0</pts-for-match-win>
    pub quick_advance: bool,
    // <ranked-by>match wins</ranked-by>
    pub require_score_agreement: bool,
    pub rr_pts_for_game_tie: f64, // >0.0</rr-pts-for-game-tie>
    pub rr_pts_for_game_win: f64, //>0.0</rr-pts-for-game-win>
    pub rr_pts_for_match_tie: f64, //>0.5</rr-pts-for-match-tie>
    pub rr_pts_for_match_win: f64, //>1.0</rr-pts-for-match-win>
    pub sequential_pairings: bool,
    pub show_rounds: bool,
    // <signup-cap nil="true"/>
    // <start-at nil="true"/>
    pub started_at: Option<DateTime<FixedOffset>>, //2015-01-19T16:57:17-05:00</started-at>
    // <started-checking-in-at nil="true"/>
    // <state>underway</state>
    pub swiss_rounds: u64,
    pub teams: bool,
    // <tie-breaks type="array">
    // <tie-break>match wins vs tied</tie-break>
    // <tie-break>game wins</tie-break>
    // <tie-break>points scored</tie-break>
    // </tie-breaks>
    pub tournament_type: TournamentType,
    pub updated_at: DateTime<FixedOffset>, //>2015-01-19T16:57:17-05:00</updated-at>
    pub url: String,
    pub description_source: String,
    // <subdomain nil="true"/>
    pub full_challonge_url: String,
    pub live_image_url: String,
    // <sign-up-url nil="true"/>
    pub review_before_finalizing: bool,
    pub accepting_predictions: bool,
    pub participants_locked: bool,
    pub game_name: String,
    pub participants_swappable: bool,
    pub team_convertable: bool,
    pub group_stages_were_started: bool,
}
impl Tournament {
    pub fn decode(value: Value) -> Result<Tournament, Error> {
        let mut value = try!(into_map(value));
        let t = try!(remove(&mut value, "tournament"));
        let mut tv = try!(into_map(t));

        let mut started_at = None;
        if let Some(dt_str) = try!(remove(&mut tv, "started_at")).as_string() {
            if let Ok(dt) = DateTime::parse_from_rfc3339(dt_str) {
                started_at = Some(dt);
            }
        }

        Ok(Tournament {
            accept_attachments: try!(remove(&mut tv, "accept_attachments")).as_boolean().unwrap_or(false),
            allow_participant_match_reporting: try!(remove(&mut tv, "allow_participant_match_reporting")).as_boolean().unwrap_or(false),
            anonymous_voting: try!(remove(&mut tv, "anonymous_voting")).as_boolean().unwrap_or(false),
            created_at: DateTime::parse_from_rfc3339(try!(remove(&mut tv, "created_at")).as_string().unwrap_or("")).unwrap(),
            created_by_api: try!(remove(&mut tv, "created_by_api")).as_boolean().unwrap_or(false),
            credit_capped: try!(remove(&mut tv, "credit_capped")).as_boolean().unwrap_or(false),
            description: try!(remove(&mut tv, "description")).as_string().unwrap_or("").to_string(),
            game_id: try!(remove(&mut tv, "game_id")).as_u64().unwrap_or(0),
            id: try!(remove(&mut tv, "id")).as_u64().unwrap_or(0),
            name: try!(remove(&mut tv, "name")).as_string().unwrap_or("").to_string(),
            group_stages_enabled: try!(remove(&mut tv, "group_stages_enabled")).as_boolean().unwrap_or(false),
            hide_forum: try!(remove(&mut tv, "hide_forum")).as_boolean().unwrap_or(false),
            hide_seeds: try!(remove(&mut tv, "hide_seeds")).as_boolean().unwrap_or(false),
            hold_third_place_match: try!(remove(&mut tv, "hold_third_place_match")).as_boolean().unwrap_or(false),
            max_predictions_per_user: try!(remove(&mut tv, "max_predictions_per_user")).as_u64().unwrap_or(0),
            notify_users_when_matches_open: try!(remove(&mut tv, "notify_users_when_matches_open")).as_boolean().unwrap_or(false),
            notify_users_when_the_tournament_ends: try!(remove(&mut tv, "notify_users_when_the_tournament_ends")).as_boolean().unwrap_or(false),
            open_signup: try!(remove(&mut tv, "open_signup")).as_boolean().unwrap_or(false),
            participants_count: try!(remove(&mut tv, "participants_count")).as_u64().unwrap_or(0),
            prediction_method: try!(remove(&mut tv, "prediction_method")).as_u64().unwrap_or(0),
            private: try!(remove(&mut tv, "private")).as_boolean().unwrap_or(false),
            progress_meter: try!(remove(&mut tv, "progress_meter")).as_u64().unwrap_or(0),
            pts_for_bye: try!(remove(&mut tv, "pts_for_bye")).as_string().unwrap_or("").to_owned().parse::<f64>().unwrap_or(0.0f64),
            pts_for_game_tie: try!(remove(&mut tv, "pts_for_game_tie")).as_string().unwrap_or("").to_owned().parse::<f64>().unwrap_or(0.0f64),
            pts_for_game_win: try!(remove(&mut tv, "pts_for_game_win")).as_string().unwrap_or("").to_owned().parse::<f64>().unwrap_or(0.0f64),
            pts_for_match_tie: try!(remove(&mut tv, "pts_for_match_tie")).as_string().unwrap_or("").to_owned().parse::<f64>().unwrap_or(0.0f64),
            pts_for_match_win: try!(remove(&mut tv, "pts_for_match_win")).as_string().unwrap_or("").to_owned().parse::<f64>().unwrap_or(0.0f64),
            quick_advance: try!(remove(&mut tv, "quick_advance")).as_boolean().unwrap_or(false),
            require_score_agreement: try!(remove(&mut tv, "require_score_agreement")).as_boolean().unwrap_or(false),
            rr_pts_for_game_tie: try!(remove(&mut tv, "rr_pts_for_game_tie")).as_string().unwrap_or("").to_owned().parse::<f64>().unwrap_or(0.0f64),
            rr_pts_for_game_win: try!(remove(&mut tv, "rr_pts_for_game_win")).as_string().unwrap_or("").to_owned().parse::<f64>().unwrap_or(0.0f64),
            rr_pts_for_match_tie: try!(remove(&mut tv, "rr_pts_for_match_tie")).as_string().unwrap_or("").to_owned().parse::<f64>().unwrap_or(0.0f64),
            rr_pts_for_match_win: try!(remove(&mut tv, "rr_pts_for_match_win")).as_string().unwrap_or("").to_owned().parse::<f64>().unwrap_or(0.0f64),
            sequential_pairings: try!(remove(&mut tv, "sequential_pairings")).as_boolean().unwrap_or(false),
            show_rounds: try!(remove(&mut tv, "show_rounds")).as_boolean().unwrap_or(false),
            started_at: started_at,
            swiss_rounds: try!(remove(&mut tv, "swiss_rounds")).as_u64().unwrap_or(0),
            teams: try!(remove(&mut tv, "teams")).as_boolean().unwrap_or(false),
            tournament_type: TournamentType::from_str(try!(remove(&mut tv, "tournament_type")).as_string().unwrap_or("")).unwrap_or(TournamentType::SingleElimination),
            updated_at: DateTime::parse_from_rfc3339(try!(remove(&mut tv, "updated_at")).as_string().unwrap()).unwrap(),
            url: try!(remove(&mut tv, "url")).as_string().unwrap_or("").to_string(),
            description_source: try!(remove(&mut tv, "description_source")).as_string().unwrap_or("").to_string(),
            full_challonge_url: try!(remove(&mut tv, "full_challonge_url")).as_string().unwrap_or("").to_string(),
            live_image_url: try!(remove(&mut tv, "live_image_url")).as_string().unwrap_or("").to_string(),
            review_before_finalizing: try!(remove(&mut tv, "review_before_finalizing")).as_boolean().unwrap_or(false),
            accepting_predictions: try!(remove(&mut tv, "accepting_predictions")).as_boolean().unwrap_or(false),
            participants_locked: try!(remove(&mut tv, "participants_locked")).as_boolean().unwrap_or(false),
            game_name: try!(remove(&mut tv, "game_name")).as_string().unwrap_or("").to_string(),
            participants_swappable: try!(remove(&mut tv, "participants_swappable")).as_boolean().unwrap_or(false),
            team_convertable: try!(remove(&mut tv, "team_convertable")).as_boolean().unwrap_or(false),
            group_stages_were_started: try!(remove(&mut tv, "group_stages_were_started")).as_boolean().unwrap_or(false),
        })
    }
}
fn decode_tournaments(value: Value) -> Vec<Tournament> {
    let mut ts = Vec::new();
    if let Some(arr) = value.as_array() {
        for o in arr {
            if let Ok(t) = Tournament::decode(o.clone().to_owned()) {
                ts.push(t);
            }
        }
    }
    ts
}

/// A list of tournaments of the account/organization.
#[derive(Debug, Clone)]
pub struct Index {
    pub tournaments: Vec<Tournament>
}
impl Index {
    pub fn decode(value: Value) -> Index {
        Index {
            tournaments: decode_tournaments(value)
        }
    }
}

/// A type of a tournament. 
#[derive(Debug, Clone, PartialEq)]
pub enum TournamentType {
    SingleElimination,
    DoubleElimination,
    RoundRobin,
    Swiss
}
impl TournamentType {
    pub fn to_get_param(&self) -> &'static str {
        match self {
            &TournamentType::SingleElimination => {
                "single_elimination"
            },
            &TournamentType::DoubleElimination => {
                "double_elimination"
            },
            &TournamentType::RoundRobin => {
                "round_robin"
            },
            &TournamentType::Swiss => {
                "swiss"
            },
        }
    }
}
impl fmt::Display for TournamentType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TournamentType::SingleElimination => {
                try!(fmt.write_str("single elimination"));
            },
            &TournamentType::DoubleElimination => {
                try!(fmt.write_str("double elimination"));
            },
            &TournamentType::RoundRobin => {
                try!(fmt.write_str("round robin"));
            },
            &TournamentType::Swiss => {
                try!(fmt.write_str("swiss"));
            },
        }
        Ok(())
    }
}
impl FromStr for TournamentType {
    type Err = ();
    fn from_str(s: &str) -> Result<TournamentType, ()> {
        match s {
            "single_elimination" => return Ok(TournamentType::SingleElimination),
            "single elimination" => return Ok(TournamentType::SingleElimination),
            "double_elimination" => return Ok(TournamentType::DoubleElimination),
            "double elimination" => return Ok(TournamentType::DoubleElimination),
            "round_robin" => return Ok(TournamentType::RoundRobin),
            "round robin" => return Ok(TournamentType::RoundRobin),
            "swiss" => return Ok(TournamentType::Swiss),
            _ => Err(()),
        }
    }
}

/// Current tournament state. 
#[derive(Debug, Clone)]
pub enum TournamentState {
    All,
    Pending,
    InProgress,
    Ended
}
impl fmt::Display for TournamentState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TournamentState::All => {
                try!(fmt.write_str("all"));
            },
            &TournamentState::Pending => {
                try!(fmt.write_str("pending"));
            },
            &TournamentState::InProgress => {
                try!(fmt.write_str("in_progress"));
            },
            &TournamentState::Ended => {
                try!(fmt.write_str("ended"));
            },
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use tournament::{ Tournament, TournamentType };

    #[test]
    fn test_tournament_parse() {
        let string = r#"{
          "tournament": {
            "accept_attachments": false,
            "allow_participant_match_reporting": true,
            "anonymous_voting": false,
            "category": null,
            "check_in_duration": null,
            "completed_at": null,
            "created_at": "2015-01-19T16:47:30-05:00",
            "created_by_api": false,
            "credit_capped": false,
            "description": "sample description",
            "game_id": 600,
            "group_stages_enabled": false,
            "hide_forum": false,
            "hide_seeds": false,
            "hold_third_place_match": false,
            "id": 1086875,
            "max_predictions_per_user": 1,
            "name": "Sample Tournament 1",
            "notify_users_when_matches_open": true,
            "notify_users_when_the_tournament_ends": true,
            "open_signup": false,
            "participants_count": 4,
            "prediction_method": 0,
            "predictions_opened_at": null,
            "private": false,
            "progress_meter": 0,
            "pts_for_bye": "1.0",
            "pts_for_game_tie": "0.0",
            "pts_for_game_win": "0.0",
            "pts_for_match_tie": "0.5",
            "pts_for_match_win": "1.0",
            "quick_advance": false,
            "ranked_by": "match wins",
            "require_score_agreement": false,
            "rr_pts_for_game_tie": "0.0",
            "rr_pts_for_game_win": "0.0",
            "rr_pts_for_match_tie": "0.5",
            "rr_pts_for_match_win": "1.0",
            "sequential_pairings": false,
            "show_rounds": true,
            "signup_cap": null,
            "start_at": null,
            "started_at": "2015-01-19T16:57:17-05:00",
            "started_checking_in_at": null,
            "state": "underway",
            "swiss_rounds": 0,
            "teams": false,
            "tie_breaks": [
              "match wins vs tied",
              "game wins",
              "points scored"
            ],
            "tournament_type": "single elimination",
            "updated_at": "2015-01-19T16:57:17-05:00",
            "url": "sample_tournament_1",
            "description_source": "sample description source",
            "subdomain": null,
            "full_challonge_url": "http://challonge.com/sample_tournament_1",
            "live_image_url": "http://images.challonge.com/sample_tournament_1.png",
            "sign_up_url": null,
            "review_before_finalizing": true,
            "accepting_predictions": false,
            "participants_locked": true,
            "game_name": "Table Tennis",
            "participants_swappable": false,
            "team_convertable": false,
            "group_stages_were_started": false
          }
        }"#;
        let json_r = serde_json::from_str(string);
        assert!(json_r.is_ok());
        let json = json_r.unwrap();
        if let Ok(t) = Tournament::decode(json) {
            assert_eq!(t.accept_attachments, false);
            assert_eq!(t.allow_participant_match_reporting, true);
            assert_eq!(t.anonymous_voting, false);
            // assert_eq!(t.created_at, DateTime<);
            assert_eq!(t.created_by_api, false);
            assert_eq!(t.description, "sample description");
            assert_eq!(t.credit_capped, false);
            assert_eq!(t.game_id, 600);
            assert_eq!(t.id, 1086875);
            assert_eq!(t.name, "Sample Tournament 1");
            assert_eq!(t.group_stages_enabled, false);
            assert_eq!(t.hide_forum, false);
            assert_eq!(t.hide_seeds, false);
            assert_eq!(t.hold_third_place_match, false);
            assert_eq!(t.max_predictions_per_user, 1);
            assert_eq!(t.notify_users_when_matches_open, true);
            assert_eq!(t.notify_users_when_the_tournament_ends, true);
            assert_eq!(t.open_signup, false);
            assert_eq!(t.participants_count, 4);
            assert_eq!(t.prediction_method, 0);
            assert_eq!(t.private, false);
            assert_eq!(t.progress_meter, 0);
            assert_eq!(t.pts_for_bye, 1.0f64);
            assert_eq!(t.pts_for_game_tie, 0.0f64);
            assert_eq!(t.pts_for_game_win, 0.0f64);
            assert_eq!(t.pts_for_match_tie, 0.5f64);
            assert_eq!(t.pts_for_match_win, 1.0f64);
            assert_eq!(t.quick_advance, false);
            assert_eq!(t.require_score_agreement, false);
            assert_eq!(t.rr_pts_for_game_tie, 0.0f64);
            assert_eq!(t.rr_pts_for_game_win, 0.0f64);
            assert_eq!(t.rr_pts_for_match_tie, 0.5f64);
            assert_eq!(t.rr_pts_for_match_win, 1.0f64);
            assert_eq!(t.sequential_pairings, false);
            assert_eq!(t.show_rounds, true);
            // assert_eq!(t.started_at, DateTime<);
            assert_eq!(t.swiss_rounds, 0);
            assert_eq!(t.teams, false);
            assert_eq!(t.tournament_type, TournamentType::SingleElimination);
            // assert_eq!(t.updated_at, DateTime<);
            assert_eq!(t.url, "sample_tournament_1");
            assert_eq!(t.description_source, "sample description source");
            assert_eq!(t.full_challonge_url, "http://challonge.com/sample_tournament_1");
            assert_eq!(t.live_image_url, "http://images.challonge.com/sample_tournament_1.png");
            assert_eq!(t.review_before_finalizing, true);
            assert_eq!(t.accepting_predictions, false);
            assert_eq!(t.participants_locked, true);
            assert_eq!(t.game_name, "Table Tennis");
            assert_eq!(t.participants_swappable, false);
            assert_eq!(t.team_convertable, false);
            assert_eq!(t.group_stages_were_started, false);
        } else {
            assert!(false);
        }
    }
}