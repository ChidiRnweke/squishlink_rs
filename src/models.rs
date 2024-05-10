use crate::schema::links;
use std::time::SystemTime;

use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::links)]
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
