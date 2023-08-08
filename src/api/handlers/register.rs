use crate::db::repository::user_repository::{UserCreationError, UserRepository};
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
    // Check for a "user" cookie
    if cookies.get("user").is_some() {
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
) -> Result<Redirect, UserCreationError> {
    let mut conn = conn
        .inner()
        .get()
        .map_err(|_| UserCreationError::DieselError(diesel::result::Error::RollbackTransaction))?;
    let user = UserRepository::create_user(&mut conn, &form.username, &form.email, &form.password);

    match user {
        Ok(_) => {
            // Create the cookie
            let cookie = Cookie::build("user", form.username.clone())
                .path("/")
                .expires(OffsetDateTime::now_utc() + time::Duration::days(1)) // 24 hours
                .finish();

            cookies.add(cookie);

            // Redirect
            Ok(Redirect::to(uri!("/")))
        }
        Err(error) => Err(error),
    }
}
