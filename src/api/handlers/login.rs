use crate::db::{AuthenticationError, authenticate_user};
use rocket::form::Form;
use rocket::State;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use time::OffsetDateTime;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::DbPool;

#[derive(Serialize, Deserialize, FromForm)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[get("/login")]
pub fn login_page(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    // Check for a "user" cookie
    if cookies.get("user").is_some() {
        // Redirect the user to the main page if the cookie exists
        return Err(Redirect::to("/"));
    }

    // Render the login template
    let mut context = HashMap::new();
    context.insert("name", "World"); // temporary
    Ok(Template::render("login", &context))
}

#[post("/login", data = "<form>")]
pub fn login(
    form: Form<LoginForm>,
    conn: &State<DbPool>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, AuthenticationError> {
    // Check for a "user" cookie
    if cookies.get("user").is_some() {
        // Redirect the user to the main page if the cookie exists
        return Ok(Redirect::to("/"));
    }

    let mut conn = conn.inner().get().map_err(|_| {
        AuthenticationError::DieselError(diesel::result::Error::RollbackTransaction)
    })?;
    let user = authenticate_user(&mut conn, &form.username, &form.password);

    match user {
        Ok(user) => {
            // Create the cookie
            let cookie = Cookie::build("user", user.username.clone())
                .path("/")
                .expires(OffsetDateTime::now_utc() + time::Duration::days(1)) // 24 hours
                .finish();

            cookies.add(cookie);

            // Redirect
            Ok(Redirect::to("/"))
        },
        Err(error) => Err(error),
    }
}
