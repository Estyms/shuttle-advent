use std::collections::BTreeMap;
use rocket::{get};
use rocket::http::CookieJar;
use base64::{Engine as _, engine::general_purpose};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;


#[derive(Debug, Serialize, Deserialize)]
pub struct Unknown {
    #[serde(flatten)]
    other: BTreeMap<String, u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kitchen {
    recipe: Option<Unknown>,
    pantry: Option<Unknown>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    cookies: u64,
    pantry: Unknown
}


fn decode_b64(b64: &str) -> String {
    general_purpose::STANDARD.decode(b64).unwrap().iter().map(|x| *x as char).collect::<String>()
}

#[get("/decode")]
pub fn decode(cookies: &CookieJar<'_>) -> Json<Unknown> {
    let recipe = cookies.get("recipe").unwrap().value();
    let res : Unknown = serde_json::from_str(decode_b64(recipe).as_str()).unwrap();
    Json::from(res)
}

#[get("/bake")]
pub fn bake(cookies: &CookieJar<'_>) -> Json<Result> {
    let recipe = cookies.get("recipe").unwrap().value();
    let kitchen : Kitchen = serde_json::from_str(decode_b64(recipe).as_str()).unwrap();
    let pantry = kitchen.pantry.unwrap().other;
    let recipe = kitchen.recipe.unwrap().other;

    let mut res = Result {
        cookies: 0,
        pantry: Unknown {
            other: BTreeMap::new()
        }
    };

    res.cookies = recipe.keys().fold(u64::MAX, |min, k| {
        let (available, recipe) = (pantry.get(k), recipe.get(k).unwrap());
        match available {
            None => {0}
            Some(x) => {min.min(x / recipe)}
        }
    });

    pantry.keys().for_each(|k| {
        let (available, recipe) = (pantry.get(k).unwrap(), *recipe.get(k).unwrap_or(&0));
        res.pantry.other.insert(k.clone(), available - res.cookies * recipe);
    });
    Json::from(res)
}