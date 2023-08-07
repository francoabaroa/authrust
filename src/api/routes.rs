use rocket::http::CookieJar;
use std::collections::HashMap;
use rocket_dyn_templates::Template;

use crate::api::handlers::{login, register};

pub fn configure() -> Vec<rocket::Route> {
    rocket::routes![
        login::login_page,
        login::login,
        register::register_page,
        register::register,
        index
    ]
}

#[get("/")]
pub fn index(cookies: &CookieJar<'_>) -> Template {
    let mut context = HashMap::new();
    if let Some(user_cookie) = cookies.get("user") {
        context.insert("username", user_cookie.value().to_string());
    }
    Template::render("index", &context)
}