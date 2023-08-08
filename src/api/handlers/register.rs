use crate::db::repository::user_repository::UserRepository;
use crate::session::{SessionStore, UserSession};
use crate::DbPool;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::State;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, FromForm)]
pub struct RegistrationForm {
    username: String,
    email: String,
    password: String,
}

#[get("/register")]
pub fn register_page(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    // Check for a "session_id" cookie
    if cookies.get("session_id").is_some() {
        // Redirect the user to the main page if the cookie exists
        return Err(Redirect::to("/"));
    }

    // Render the registration template
    let mut context = HashMap::new();
    context.insert("name", "World"); // temporary
    Ok(Template::render("register", &context))
}

#[post("/register", data = "<form>")]
pub fn register(
    form: Form<RegistrationForm>,
    conn: &State<DbPool>,
    cookies: &CookieJar<'_>,
    sessions: &State<SessionStore>,
) -> Result<Redirect, Template> {
    let mut conn = conn.inner().get().map_err(|_| {
        let mut context = HashMap::new();
        context.insert("error_message", "Database connection error.");
        return Template::render("error", context);
    })?;
    let user = UserRepository::create_user(&mut conn, &form.username, &form.email, &form.password);

    match user {
        Ok(_) => {
            // Generate a session ID and store the session data
            let user_session = UserSession {
                username: form.username.clone(),
                email: form.email.clone(),
            };
            let session_id = sessions.inner().create_session(user_session);

            // Create the cookie
            let cookie = Cookie::build("session_id", session_id)
                .path("/")
                .expires(OffsetDateTime::now_utc() + time::Duration::days(1)) // 24 hours
                .finish();

            cookies.add(cookie);

            // Redirect
            Ok(Redirect::to(uri!("/")))
        }
        Err(error) => {
            let mut context = HashMap::new();
            context.insert(
                "error_message",
                format!("User creation error: {}", error.to_string()),
            );
            Err(Template::render("error", context))
        }
    }
}
