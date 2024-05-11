use crate::config::DBConfig;

use super::name_generator::GeneratedName;
use crate::models::*;
use crate::schema::links::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use url::Url;

pub trait NamesRepository {
    fn store_name(&mut self, original: &Url, generated: &GeneratedName) -> Result<(), String>;
    fn name_exists(&mut self, name: &GeneratedName) -> Result<bool, String>;
    fn retrieve_original_name(&mut self, name: &GeneratedName) -> Result<String, String>;
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

    fn retrieve_original_name(&mut self, name: &GeneratedName) -> Result<String, String> {
        let result: Option<Link> = links
            .filter(short_link.eq(&name.0))
            .first::<Link>(&mut self.0)
            .optional()
            .map_err(|e| e.to_string())?;
        match result {
            Some(link) => Ok(link.original_link),
            None => Err("Link not found".to_string()),
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
    PgConnection::establish("postgres://zo08vB4JXY:CNJDHeIPE5@localhost:5432/squishlink_rs")
        .expect(&format!("Error connecting to {}", database_url))
}
