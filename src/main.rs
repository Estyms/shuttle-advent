mod day1;

use rocket::{catch, catchers, get, routes};
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
        .mount("/", routes![day1::task1]);

    Ok(rocket.into())
}
