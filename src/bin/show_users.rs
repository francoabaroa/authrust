use self::models::*;
use diesel::prelude::*;
use authrust::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("\n");
        println!("{}", user.username);
        println!("-----------");
        println!("{}", user.email);
        println!("-----------\n");
    }
}