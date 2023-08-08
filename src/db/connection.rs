use crate::db::models::User;
use crate::db::repository::user_repository::UserCreationError;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection(conn: &mut PgConnection) -> Result<(), UserCreationError> {
    // Connection passed as an argument
    use crate::db::schema::users::dsl::users;

    let results = users
        .load::<User>(conn)
        .map_err(UserCreationError::DieselError)?; // Sample query to verify connection

    println!("Loaded users: {:?}", results);

    Ok(())
}
