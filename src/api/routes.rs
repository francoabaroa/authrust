use crate::session::SessionStore;
use rocket::http::CookieJar;
use rocket::State;
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
pub fn index(cookies: &CookieJar<'_>, sessions: &State<SessionStore>) -> Template {
    let mut context = HashMap::new();
    if let Some(session_cookie) = cookies.get_private("session_id") {
        let session_id = session_cookie.value();
        if let Some(user_session) = sessions.inner().get_user(session_id) {
            context.insert("username", user_session.username);
        }
    }
    Template::render("index", &context)
}

#[catch(404)]
pub fn not_found() -> Template {
    let mut context = HashMap::new();
    context.insert("error_message", "Page not found");
    Template::render("error", &context)
}
