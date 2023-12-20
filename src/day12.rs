use chrono::{Datelike, Local, NaiveDateTime, TimeZone, Utc};
use rocket::{get, post, State};
use rocket::serde::json::Json;
use serde_derive::{Deserialize, Serialize};
use ulid::{Ulid};
use uuid::Uuid;
use crate::Day12Save;

#[post("/save/<packet>")]
pub async fn save(day_12_data: &State<Day12Save>, packet: String) {
    day_12_data.map.lock().unwrap().insert(packet, Local::now());
}

#[get("/load/<packet>")]
pub async fn load(day_12_data: &State<Day12Save>, packet: String) -> String {
    match day_12_data.map.lock().unwrap().get(&packet) {
        None => "0".to_string(),
        Some(x) => format!("{}",(Local::now().timestamp_millis() - x.timestamp_millis()) / 1000)
    }
}

#[post("/ulids", data = "<data>")]
pub async fn ulids(data: Json<Vec<String>>) ->  Json<Vec<String>> {
    let ulids = data.0.iter().map(|u| Ulid::from_string(u).unwrap()).collect::<Vec<Ulid>>();
    let uuids = ulids.iter().map(|ulid| Uuid::from_bytes(ulid.to_bytes())).collect::<Vec<Uuid>>();
    Json::from(uuids.iter().rev().map(|uuid| uuid.to_string()).collect::<Vec<String>>())
}


#[derive(Serialize, Deserialize)]
pub struct UlidWeekdayResponse {
    #[serde(rename(serialize = "christmas eve"))]
    christmas_eve: usize,
    weekday: usize,
    #[serde(rename(serialize = "in the future"))]
    future: usize,
    #[serde(rename(serialize = "LSB is 1"))]
    lsb_one: usize
}

#[post("/ulids/<weekday>", data = "<data>")]
pub async fn ulids_weekday(weekday: u32, data: Json<Vec<String>>) ->  Json<UlidWeekdayResponse> {
    let ulids = data.0.iter().map(|u| Ulid::from_string(u).unwrap()).collect::<Vec<Ulid>>();
    let times = ulids.iter().map(|u| Utc.timestamp_millis_opt(u.timestamp_ms() as i64).unwrap().naive_utc()).collect::<Vec<NaiveDateTime>>();

    use chrono::Weekday::*;
    let weekday = match weekday {
        0 => Mon,
        1 => Tue,
        2 => Wed,
        3 => Thu,
        4 => Fri,
        5 => Sat,
        6 => Sun,
        _ => panic!("Not a weekday")
    };
    let weekday = times.iter().filter(|x| x.weekday() == weekday).count();
    let future = times.iter().filter(|x| x.timestamp_millis() > Utc::now().timestamp_millis() ).count();
    let christmas_eve = times.iter().filter(|x| x.date().month() == 12 && x.day() == 24).count();
    let lsb_one = ulids.iter().filter(|x| x.0 & 1 == 1).count();
    Json::from(UlidWeekdayResponse {
        weekday,
        future,
        christmas_eve,
        lsb_one
    })
}