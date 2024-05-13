use crate::config::DBConfig;

use super::name_generator::GeneratedName;
use crate::errors::AppError;
use crate::schema::links::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use url::Url;

use crate::schema::{self, links};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::time::{Duration, SystemTime};
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

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
    fn store_name(&mut self, original: &Url, generated: &GeneratedName) -> Result<(), AppError>;
    fn name_exists(&mut self, name: &GeneratedName) -> Result<bool, AppError>;
    fn retrieve_original_name(&mut self, name: &GeneratedName) -> Result<String, AppError>;
}

pub struct PostgresRepository(PgConnection);

impl NamesRepository for PostgresRepository {
    fn name_exists(&mut self, name: &GeneratedName) -> Result<bool, AppError> {
        let result: Option<Link> = links
            .filter(short_link.eq(&name.0))
            .first::<Link>(&mut self.0)
            .optional()
            .map_err(AppError::DatabaseError)?;
        Ok(result.is_some())
    }

    fn retrieve_original_name(&mut self, name: &GeneratedName) -> Result<String, AppError> {
        let result: Option<Link> = links
            .filter(short_link.eq(&name.0))
            .first::<Link>(&mut self.0)
            .optional()
            .map_err(AppError::DatabaseError)?;
        result
            .ok_or(AppError::NotFoundError)
            .map(|link| link.original_link)
    }

    fn store_name(&mut self, original: &Url, generated: &GeneratedName) -> Result<(), AppError> {
        let new_link = NewLink {
            original_link: original.as_str(),
            short_link: &generated.0,
        };
        diesel::insert_into(links)
            .values(new_link)
            .execute(&mut self.0)
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }
}

impl PostgresRepository {
    fn from_connection(conn: PgConnection) -> Self {
        Self(conn)
    }

    pub fn from_config(db_config: &DBConfig) -> Result<Self, AppError> {
        let connection = establish_connection(db_config)?;
        Ok(Self::from_connection(connection))
    }

    pub fn cleanup_old_links(&mut self) -> Result<usize, diesel::result::Error> {
        let week = Duration::from_secs_f64(604800.0);
        let last_week = SystemTime::now() + week;
        diesel::delete(links)
            .filter(created_at.lt(last_week))
            .execute(&mut self.0)
    }
}

fn establish_connection(db_config: &DBConfig) -> Result<PgConnection, AppError> {
    let database_url = db_config.to_connection_string();
    PgConnection::establish(&database_url).map_err(|e| AppError::InfraError(e.to_string()))
}

pub fn run_migration(db_config: &DBConfig) {
    let mut conn = establish_connection(&db_config)
        .expect("An error occurred when trying to obtain a database connection to run migrations. Shutting down app.");
    conn.run_pending_migrations(MIGRATIONS).expect("foo");
    log::info!("Migrations ran successfully.");
}
