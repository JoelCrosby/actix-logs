use super::schema::log_entries;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct LogEntryEntity {
    pub id: i32,
    pub title: String,
    pub serialised: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "log_entries"]
pub struct CreateLogEntry<'a> {
    pub title: &'a str,
    pub serialised: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: i32,
    pub title: String,
    pub serialised: String,
}
