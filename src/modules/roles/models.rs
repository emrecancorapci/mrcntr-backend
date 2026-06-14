use crate::config::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub id: i32,
    pub title: String,
}
