# Authentication Service in Rust (IN PROGRESS)

This is a simple, experimental authentication service built in Rust. It includes functionalities such as user registration and viewing all users. More functionality to follow.

## Requirements

- Rust (latest stable version)
- PostgreSQL

## Environment Variables

You need to set the following environment variables for database configuration and security:

- `DATABASE_URL`: The connection string of your PostgreSQL database in the format `postgres://username:password@localhost/database_name`.
- `SALT`: The salt used for hashing the passwords.

Please note `SALT` is temporary; this is not safe at all and each user should have an individual salt.

You can set these variables in your environment or in a `.env` file at the root of your project.

## Running the Project

1. Make sure you have created your database instance locally and set the DATABASE_URL environment variable correctly. Then run your Diesel migrations to set up the database schema.

    ```
    diesel migration run
    ```

2. Use Cargo to build and run the project:

    ```bash
    cargo run --bin authrust
    ```

This will start the server. The port and host would depend on how you have configured your Rocket application.

**Note:** As this project is experimental, it may have issues or lack certain features. It is not recommended for use in production environments without further development and testing.

## Project Structure

The project has been structured in a modular fashion with separate files for database operations, schema definitions, and business logic. If you're new to Rust, it may take some time to familiarize yourself with the ownership, borrowing and lifetime concepts which are heavily used in this project.

Feel free to explore the project and open issues or pull requests if you have any questions, comments, or improvements.

Happy coding!
