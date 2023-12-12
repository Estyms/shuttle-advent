use rocket::{get};
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    pub weight: f64,
}

async fn get_pokemon(id: i32) -> Pokemon {
    reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", id)).await.unwrap().json().await.unwrap()
}

#[get("/weight/<id>")]
pub async fn weight(id: i32) -> String {
    let pokemon = get_pokemon(id).await;
    String::from(format!("{}",pokemon.weight/10.0))
}

#[get("/drop/<id>")]
pub async fn drop(id: i32) -> String {
    let weight = get_pokemon(id).await.weight;
    format!("{}", weight * (9.825f64 * 10.0 * 2.0).sqrt() / 10.0)
}