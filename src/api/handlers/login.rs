use crate::db::repository::user_repository::{AuthenticationError, UserRepository};
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
pub struct LoginForm {
    username: String,
    password: String,
}

#[get("/login")]
pub fn login_page(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    // Check for a "session_id" private cookie
    if cookies.get_private("session_id").is_some() {
        // Redirect the user to the main page if the private cookie exists
        return Err(Redirect::to("/"));
    }
    let mut context = HashMap::new();
    context.insert("name", "World"); // temporary

    // Render the login template
    Ok(Template::render("login", &context))
}

#[post("/login", data = "<form>")]
pub fn login(
    form: Form<LoginForm>,
    conn: &State<DbPool>,
    cookies: &CookieJar<'_>,
    sessions: &State<SessionStore>,
) -> Result<Redirect, Template> {
    let mut conn = conn.inner().get().map_err(|_| {
        let mut context = HashMap::new();
        context.insert("error_message", "Database connection error.");
        return Template::render("error", context);
    })?;
    let user: Result<crate::db::models::User, AuthenticationError> =
        UserRepository::authenticate_user(&mut conn, &form.username, &form.password);

    match user {
        Ok(user) => {
            let user_session = UserSession {
                username: user.username.clone(),
                email: user.email.clone(),
            };
            let session_id = sessions.inner().create_session(user_session);

            // Create the private cookie
            let private_cookie = Cookie::build("session_id", session_id)
                .path("/")
                .expires(OffsetDateTime::now_utc() + time::Duration::days(1)) // 24 hours
                .finish();

            cookies.add_private(private_cookie);

            Ok(Redirect::to("/"))
        }
        Err(error) => {
            let mut context = HashMap::new();
            context.insert(
                "error_message",
                format!("Authentication error: {}", error.to_string()),
            );
            Err(Template::render("error", context))
        }
    }
}
