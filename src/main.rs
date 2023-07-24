#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    hello: &'static str,
}

#[get("/")]
fn index() -> Json<Message> {
    Json(Message {
        hello: "world!",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}