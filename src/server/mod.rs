use crate::api;

pub fn start() -> Vec<rocket::Route> {
    // Initialize routes
    api::routes::configure()
}
