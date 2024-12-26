#[macro_use]
extern crate diesel;

use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel_logger::LoggingConnection;

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

pub fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // Set the desired log level
        .init();

    let conn = SqliteConnection::establish("example.sqlite").unwrap();
    let mut conn = LoggingConnection::new(conn);

    conn.batch_execute(
        "CREATE TABLE IF NOT EXISTS posts (id INTEGER, title TEXT, body TEXT, published BOOL);",
    )
    .unwrap();
    let new_post = NewPost {
        title: "mytitle",
        body: "mybody",
    };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&mut conn)
        .expect("Error saving new post");
}
