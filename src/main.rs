mod day1;
mod day4;
mod day6;
mod day7;
mod day8;
mod day11;

use rocket::{get, routes};
use rocket::http::Status;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

//noinspection ALL
#[get("/-1/error")]
fn error() -> Status {
    Status::InternalServerError
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![error])
        .mount("/1/", routes![day1::xorcube])
        .mount("/4/", routes![day4::strength, day4::contest])
        .mount("/6/", routes![day6::elf])
        .mount("/7/", routes![day7::decode, day7::bake])
        .mount("/8/", routes![day8::weight, day8::drop])
        .mount("/11/", routes![day11::serve, day11::red_pixels]);

    Ok(rocket.into())
}
