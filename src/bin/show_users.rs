use authrust::db::connection::establish_connection;
use authrust::db::models::User;
use authrust::db::schema::users::dsl::*;
use diesel::pg::PgConnection;
use diesel::Connection;
use diesel::RunQueryDsl;
use std::env;

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection =
        PgConnection::establish(&database_url).expect("Error connecting to the database");
    let _ = establish_connection(&mut connection).unwrap();

    let results = users
        .load::<User>(&mut connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.username);
        println!("-----------\n");
        println!("{}", user.email);
    }
}
