use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::Response;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder;
use std::env;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

use crate::models::{User, NewUser};
use crate::schema::users;

#[derive(Debug)]
pub enum UserCreationError {
    HashingError(argon2::password_hash::Error),
    DieselError(diesel::result::Error),
    DuplicateUser(String),
}

impl<'r> Responder<'r, 'static> for UserCreationError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            UserCreationError::HashingError(_) => Response::build().status(Status::InternalServerError).ok(),
            UserCreationError::DieselError(_) => Response::build().status(Status::BadRequest).ok(),
            UserCreationError::DuplicateUser(msg) => {
                Response::build()
                    .header(ContentType::Plain)
                    .sized_body(msg.len(), std::io::Cursor::new(msg))
                    .status(Status::BadRequest)
                    .ok()
            },
        }
    }
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
    use crate::schema::users::dsl::users;

    let results = users.load::<User>(conn).map_err(UserCreationError::DieselError)?; // Sample query to verify connection

    println!("Loaded users: {:?}", results);

    Ok(())
}

pub fn create_user(conn: &mut PgConnection, username: &str, email: &str, password: &str) -> Result<User, UserCreationError> {
    // Check if the username or email already exists
    let existing_user = users::table
        .filter(users::username.eq(username))
        .or_filter(users::email.eq(email))
        .first::<User>(conn)
        .optional()
        .map_err(UserCreationError::DieselError)?;

    // If the user already exists, return an error
    if existing_user.is_some() {
        return Err(UserCreationError::DuplicateUser(
            "Username or email already exists".into()
        ));
    }


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
    let salt = env::var("SALT").unwrap(); // temporary! not safe at all
    let salt = SaltString::encode_b64(salt.as_bytes())?;

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), salt.as_salt())?;

    Ok(password_hash.to_string())
}
