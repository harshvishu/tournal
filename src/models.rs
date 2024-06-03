use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub user_id: Option<i32>,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::journals)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Journal {
    pub journal_id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
    pub entry_id: i32,
    pub journal_id: i32,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub tag_id: i32,
    pub user_id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::entry_tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EntryTag {
    pub entry_id: i32,
    pub tag_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::attachments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Attachment {
    pub attachment_id: i32,
    pub entry_id: i32,
    pub file_path: String,
    pub uploaded_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Comment {
    pub comment_id: i32,
    pub entry_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::reminders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Reminder {
    pub reminder_id: i32,
    pub entry_id: i32,
    pub reminder_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
