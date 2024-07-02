use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

use crate::impl_actix_responder;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl_actix_responder!(User);

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}
