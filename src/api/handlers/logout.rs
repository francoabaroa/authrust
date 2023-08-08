use crate::session::SessionStore;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::State;
use time::OffsetDateTime;

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>, sessions: &State<SessionStore>) -> Redirect {
    if let Some(session_cookie) = cookies.get("session_id") {
        let session_id = session_cookie.value().to_string();
        sessions.inner().remove_session(&session_id);

        let cookie = Cookie::build("session_id", "")
            .path("/")
            .expires(OffsetDateTime::now_utc() - time::Duration::days(1)) // Past time
            .finish();
        cookies.remove(cookie);
    }

    // Redirect the user to the login page
    Redirect::to("/")
}
