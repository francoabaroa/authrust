#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenvy::dotenv;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::State;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod db;
pub mod models;
pub mod schema;

use db::{create_user, AuthenticationError, UserCreationError};
use models::User;

#[derive(Serialize, Deserialize, FromForm)]
pub struct RegistrationForm {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct LoginForm {
    username: String,
    password: String,
}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "World"); // temporary
    Template::render("index", &context)
}

#[get("/login")]
fn login_page() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "World"); // temporary
    Template::render("login", &context)
}

#[post("/login", data = "<form>")]
fn login(form: Form<LoginForm>, conn: &State<DbPool>) -> Result<Json<User>, AuthenticationError> {
    let mut conn = conn.inner().get().map_err(|_| {
        AuthenticationError::DieselError(diesel::result::Error::RollbackTransaction)
    })?;
    let user = db::authenticate_user(&mut conn, &form.username, &form.password);

    match user {
        Ok(user) => Ok(Json(user)),
        Err(error) => Err(error),
    }
}

#[get("/register")]
fn register_page() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "World"); // temporary
    Template::render("register", &context)
}

#[post("/register", data = "<form>")]
fn register(
    form: Form<RegistrationForm>,
    conn: &State<DbPool>,
) -> Result<Json<User>, UserCreationError> {
    let mut conn = conn
        .inner()
        .get()
        .map_err(|_| UserCreationError::DieselError(diesel::result::Error::RollbackTransaction))?;
    let user = create_user(&mut conn, &form.username, &form.email, &form.password);

    match user {
        Ok(user) => Ok(Json(user)),
        Err(error) => Err(error),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    use std::env;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // Run embedded migrations
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    rocket::build()
        .attach(Template::fairing())
        .manage(pool)
        .mount(
            "/",
            routes![index, login, login_page, register, register_page],
        )
}
