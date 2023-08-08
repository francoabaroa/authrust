use rocket::http::CookieJar;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

use crate::api::handlers::{login, logout, register};

pub fn configure() -> Vec<rocket::Route> {
    rocket::routes![
        login::login_page,
        login::login,
        logout::logout,
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

#[catch(404)]
pub fn not_found() -> Template {
    let mut context = HashMap::new();
    context.insert("error_message", "Page not found");
    Template::render("error", &context)
}
