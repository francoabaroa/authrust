#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;
use serde::{Serialize, Deserialize};
use rocket::fairing::AdHoc;

pub mod db;
pub mod models;
pub mod schema;

use db::{create_user, establish_connection, UserCreationError};
use crate::models::User;

#[derive(Serialize, Deserialize)]
pub struct RegistrationForm {
    username: String,
    email: String,
    password: String,
}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "World");
    Template::render("index", &context)
}

#[post("/register", format = "json", data = "<form>")]
fn register(form: Json<RegistrationForm>, conn: &State<DbPool>) -> Result<Json<User>, UserCreationError> {
    let mut conn = conn.inner().get().map_err(|_| UserCreationError::DieselError(diesel::result::Error::RollbackTransaction))?;
    let user = create_user(&mut conn, &form.username, &form.email, &form.password);

    match user {
        Ok(user) => Ok(Json(user)),
        Err(error) => Err(error),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    rocket::build()
        .attach(AdHoc::on_ignite("Database Migrations", |rocket| async {
            let mut conn = rocket
                .state::<DbPool>()
                .unwrap()
                .get()
                .expect("get connection");
            establish_connection(&mut conn).unwrap_or_else(|_| {
                panic!("Failed to establish a database connection.");
            });
            rocket
        }))
        .attach(Template::fairing())
        .manage(pool)
        .mount("/", routes![index, register])
}


