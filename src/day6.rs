use regex::{Match, Regex};
use rocket::post;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

fn regex_processing<'a>(haystack: &'a str, regex_str: &'a str) -> Vec<Match<'a>> {
    let regex = Regex::new(regex_str).unwrap();
    let mut i = 0;
    let mut matches = vec![];
    while let Some(x) = regex.find_at(haystack, i) {
        dbg!(x);
        matches.push(x);
        i = x.start() + 1;
    };
    matches
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Elven {
    elf: usize,
    #[serde(rename(serialize = "elf on a shelf"))]
    elf_on_a_shelf: usize,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    shelf_with_no_elf_on_it: usize
}

#[post("/", data = "<text>")]
pub fn elf(text: String) -> Json<Elven> {
    let strings = vec!["elf", "elf on a shelf", "shelf"];
    let captures : Vec<Vec<Match>> = strings.iter().map(|x| {
        regex_processing(text.as_str(), x)
    }).collect();

    let elf = captures.get(0).into_iter().flatten().count();
    let elf_on_a_shelf = captures.get(1).into_iter().flatten().count();
    let shelf_with_no_elf_on_it = captures.get(2).into_iter().flatten().filter(|p| {
        !captures.get(1).into_iter().flatten().any(|c|
                p.start() >= c.start()
                    && p.end() <= c.end()
        )
    }).count();

    Json::from(Elven {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf_on_it
    })
}