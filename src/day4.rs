use rocket::post;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate="rocket::serde")]
pub struct Reindeer<'a> {
    name: &'a str,
    strength: i64
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate="rocket::serde")]
pub struct ContestReindeer<'a> {
    name: &'a str,
    strength: i64,
    speed: f64,
    height: u64,
    antler_width: u64,
    snow_magic_power: u64,
    favorite_food: &'a str,
    #[serde(alias="cAnD13s_3ATeN-yesT3rdAy")]
    candies: u64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate="rocket::serde")]
pub struct ContestResults {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String
}

#[post("/strength", data = "<reindeers>")]
pub fn strength(reindeers: Json<Vec<Reindeer<'_>>>) -> String {
    let x : i64 = reindeers.iter().map(|x| x.strength).sum();
    format!("{}", x)
}

#[post("/contest", data = "<reindeers>", format="application/json")]
pub fn contest<'a>(reindeers: Json<Vec<ContestReindeer<'a>>>) -> Json<ContestResults> {
    let fastest = reindeers.iter().max_by(|a,b| a.speed.total_cmp(&b.speed)).unwrap();
    let tallest = reindeers.iter().max_by(|a,b| a.height.cmp(&b.height)).unwrap();
    let magician = reindeers.iter().max_by(|a,b| a.snow_magic_power.cmp(&b.snow_magic_power)).unwrap();
    let consumer = reindeers.iter().max_by(|a,b| a.candies.cmp(&b.candies)).unwrap();

    Json::from(ContestResults {
        fastest: format!("Speeding past the finish line with a strength of {} is {}", fastest.strength, fastest.name),
        tallest: format!("{} is standing tall with his {} cm wide antlers", tallest.name, tallest.antler_width),
        magician: format!("{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power),
        consumer: format!("{} ate lots of candies, but also some {}", consumer.name, consumer.favorite_food)
    })
}