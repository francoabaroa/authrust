#[macro_use]
extern crate rocket;
extern crate diesel_migrations;

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenvy::dotenv;
use rocket_dyn_templates::Template;
use std::env;

pub mod db;
pub mod models;
pub mod schema;
pub mod server;
pub mod api;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let mut conn = pool.get().expect("Failed to get connection from pool");
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let routes = server::start();

    rocket::build()
        .attach(Template::fairing())
        .manage(pool)
        .mount("/", routes)
}