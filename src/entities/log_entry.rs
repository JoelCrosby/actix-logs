use crate::entities::schema::log_entries::{self, dsl::*};
use crate::Pool;

use actix_web::web;
use anyhow::Error;
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{dsl::insert_into, RunQueryDsl};
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

#[derive(Debug, Deserialize)]
pub struct CreateLogEntryRequest {
    pub title: String,
    pub serialised: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: i32,
    pub title: String,
    pub serialised: String,
    pub created_at: String,
}

impl LogEntryEntity {
    pub fn create(
        pool: web::Data<Pool>,
        create_req: web::Json<CreateLogEntryRequest>,
    ) -> Result<LogEntry, Error> {
        let db_connection = pool.get()?;

        let new_entity = CreateLogEntry {
            title: &create_req.title,
            serialised: &create_req.serialised,
        };

        let data = insert_into(log_entries)
            .values(&new_entity)
            .get_result::<LogEntryEntity>(&db_connection)?;

        let result = LogEntry {
            id: data.id,
            title: data.title,
            serialised: data.serialised,
            created_at: data.created_at.to_string(),
        };

        Ok(result)
    }

    pub fn get(pool: web::Data<Pool>) -> Result<Vec<LogEntry>, Error> {
        let db_connection = pool.get()?;

        let data = log_entries
            .order_by(created_at.desc())
            .load::<LogEntryEntity>(&db_connection)?;

        let result = data
            .into_iter()
            .map(|row| LogEntry {
                id: row.id,
                title: row.title,
                serialised: row.serialised,
                created_at: row.created_at.to_string(),
            })
            .collect();

        Ok(result)
    }

    pub fn get_by_id(pool: web::Data<Pool>, entity_id: web::Path<i32>) -> Result<LogEntry, Error> {
        let db_connection = pool.get()?;

        let data: LogEntryEntity = log_entries
            .find(&entity_id.into_inner())
            .first(&db_connection)?;

        let result = LogEntry {
            id: data.id,
            title: data.title,
            serialised: data.serialised,
            created_at: data.created_at.to_string(),
        };

        Ok(result)
    }
}
