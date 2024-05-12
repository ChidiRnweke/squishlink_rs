use crate::config::DBConfig;

use super::name_generator::GeneratedName;
use crate::schema::links::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use url::Url;

use crate::schema::{self, links};
use std::time::SystemTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub id: i32,
    pub original_link: String,
    pub short_link: String,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = links)]
pub struct NewLink<'a> {
    pub original_link: &'a str,
    pub short_link: &'a str,
}

pub trait NamesRepository {
    fn store_name(&mut self, original: &Url, generated: &GeneratedName) -> Result<(), String>;
    fn name_exists(&mut self, name: &GeneratedName) -> Result<bool, String>;
    fn retrieve_original_name(&mut self, name: &GeneratedName) -> Result<Option<String>, String>;
}

pub struct PostgresRepository(PgConnection);

impl NamesRepository for PostgresRepository {
    fn name_exists(&mut self, name: &GeneratedName) -> Result<bool, String> {
        let result: Option<Link> = links
            .filter(short_link.eq(&name.0))
            .first::<Link>(&mut self.0)
            .optional()
            .map_err(|e| e.to_string())?;
        Ok(result.is_some())
    }

    fn retrieve_original_name(&mut self, name: &GeneratedName) -> Result<Option<String>, String> {
        let result: Option<Link> = links
            .filter(short_link.eq(&name.0))
            .first::<Link>(&mut self.0)
            .optional()
            .map_err(|e| e.to_string())?;
        match result {
            Some(link) => Ok(Some(link.original_link)),
            None => Ok(None),
        }
    }

    fn store_name(&mut self, original: &Url, generated: &GeneratedName) -> Result<(), String> {
        let new_link = NewLink {
            original_link: original.as_str(),
            short_link: &generated.0,
        };
        diesel::insert_into(links)
            .values(new_link)
            .execute(&mut self.0)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl PostgresRepository {
    fn from_connection(conn: PgConnection) -> Self {
        Self(conn)
    }

    pub fn from_config(db_config: &DBConfig) -> Self {
        let connection = establish_connection(db_config);
        Self::from_connection(connection)
    }
}

pub fn establish_connection(db_config: &DBConfig) -> PgConnection {
    let database_url = db_config.to_connection_string();
    let err_msg = format!("Error connecting to {}", database_url);
    PgConnection::establish(&database_url).expect(&err_msg)
}
