use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

pub mod models;
pub mod schema;

use self::models::{NewUser, User};

pub enum UserCreationError {
    HashingError(argon2::password_hash::Error),
    DieselError(diesel::result::Error),
}

impl From<argon2::password_hash::Error> for UserCreationError {
    fn from(err: argon2::password_hash::Error) -> UserCreationError {
        UserCreationError::HashingError(err)
    }
}

impl From<diesel::result::Error> for UserCreationError {
    fn from(err: diesel::result::Error) -> UserCreationError {
        UserCreationError::DieselError(err)
    }
}

pub fn establish_connection(conn: &mut PgConnection) -> Result<(), UserCreationError> { // Connection passed as an argument
    use self::schema::users::dsl::users;

    let results = users.load::<User>(conn).map_err(UserCreationError::DieselError)?; // Sample query to verify connection

    println!("Loaded users: {:?}", results);

    Ok(())
}

pub fn create_user(conn: &mut PgConnection, username: &str, email: &str, password: &str) -> Result<User, UserCreationError> {
    use self::schema::users;

    let hashed_password = hash_password(password)?;

    let new_user = NewUser {
        username,
        email,
        password: &hashed_password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .map_err(UserCreationError::DieselError)
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = env::var("SALT").unwrap(); // temporary
    let salt = SaltString::encode_b64(salt.as_bytes())?;

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), salt.as_salt())?;

    Ok(password_hash.to_string())
}
