use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use time::OffsetDateTime;

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    if cookies.get("user").is_some() {
        let cookie = Cookie::build("user", "")
            .path("/")
            .expires(OffsetDateTime::now_utc() - time::Duration::days(1)) // Past time
            .finish();
        cookies.remove(cookie);
    }

    // Redirect the user to the login page
    Redirect::to("/")
}
