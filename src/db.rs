pub const DB_PATH: &str = "./data/sqlite.db";

use diesel::prelude::*;

use crate::models::User;
use thiserror::Error;

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(&DB_PATH)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &DB_PATH))
}

pub fn create_user(
    conn: &mut SqliteConnection,
    username: &str,
    password_hash: &str,
    email: &str,
) -> Result<User, Error> {
    use crate::schema::users;
    let new_user = NewUser {
        username: username.to_owned(),
        password_hash: password_hash.to_owned(),
        email: email.to_owned(),
    };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)?;

    Ok(result)
}

pub fn get_users(conn: &mut SqliteConnection) -> Result<Vec<User>, Error> {
    use crate::schema::users::dsl::*;

    let result = users.select(User::as_select()).load(conn)?;
    Ok(result)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error reading the DB file: {0}")]
    ReadDBError(#[from] diesel::result::Error),
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub email: String,
}
