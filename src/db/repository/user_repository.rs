use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier},
    Argon2,
};
use diesel::pg::PgConnection;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::Response;

use crate::db::models::{NewUser, User};
use crate::db::schema::users;

pub struct UserRepository;

#[derive(Debug)]
pub enum UserCreationError {
    HashingError(argon2::password_hash::Error),
    DieselError(diesel::result::Error),
    DuplicateUser(String),
}

#[derive(Debug)]
pub enum AuthenticationError {
    DieselError(diesel::result::Error),
    UserNotFound,
    PasswordMismatch,
    HashingError(argon2::password_hash::Error),
}

impl From<argon2::password_hash::Error> for AuthenticationError {
    fn from(err: argon2::password_hash::Error) -> AuthenticationError {
        AuthenticationError::HashingError(err)
    }
}

impl<'r> Responder<'r, 'static> for AuthenticationError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            AuthenticationError::HashingError(_) => {
                Response::build().status(Status::InternalServerError).ok()
            }
            AuthenticationError::DieselError(_) => {
                Response::build().status(Status::BadRequest).ok()
            }
            AuthenticationError::UserNotFound => {
                let msg = "User not found";
                Response::build()
                    .header(ContentType::Plain)
                    .sized_body(msg.len(), std::io::Cursor::new(msg))
                    .status(Status::NotFound)
                    .ok()
            }
            AuthenticationError::PasswordMismatch => {
                let msg = "Password mismatch";
                Response::build()
                    .header(ContentType::Plain)
                    .sized_body(msg.len(), std::io::Cursor::new(msg))
                    .status(Status::Forbidden)
                    .ok()
            }
        }
    }
}

impl<'r> Responder<'r, 'static> for UserCreationError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            UserCreationError::HashingError(_) => {
                Response::build().status(Status::InternalServerError).ok()
            }
            UserCreationError::DieselError(_) => Response::build().status(Status::BadRequest).ok(),
            UserCreationError::DuplicateUser(msg) => Response::build()
                .header(ContentType::Plain)
                .sized_body(msg.len(), std::io::Cursor::new(msg))
                .status(Status::BadRequest)
                .ok(),
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

impl UserRepository {
    pub fn authenticate_user(
        conn: &mut PgConnection,
        username: &str,
        password: &str,
    ) -> Result<User, AuthenticationError> {
        let user = users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .optional()
            .map_err(AuthenticationError::DieselError)?;

        match user {
            Some(user) => {
                let password_hash = argon2::PasswordHash::new(&user.password)?;
                let argon2 = Argon2::default();
                if argon2
                    .verify_password(password.as_bytes(), &password_hash)
                    .is_ok()
                {
                    Ok(user)
                } else {
                    Err(AuthenticationError::PasswordMismatch)
                }
            }
            None => Err(AuthenticationError::UserNotFound),
        }
    }

    pub fn create_user(
        conn: &mut PgConnection,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User, UserCreationError> {
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
                "Username or email already exists".into(),
            ));
        }

        let hashed_password = Self::hash_password(password)?;

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
        let salt = std::env::var("SALT").unwrap(); // temporary! not safe at all
        let salt = argon2::password_hash::SaltString::encode_b64(salt.as_bytes())?;

        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), salt.as_salt())?;

        Ok(password_hash.to_string())
    }
}
