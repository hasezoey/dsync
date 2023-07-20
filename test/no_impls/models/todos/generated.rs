/* This file is generated and managed by dsync */

use crate::diesel::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;

/// Connection Type as set in dsync
type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row for table `todos`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todos {
    /// Field Representing column `id`
    pub id: i32,
    /// Field Representing column `unsigned`
    pub unsigned: u32,
    /// Field Representing column `text`
    pub text: String,
    /// Field Representing column `completed`
    pub completed: bool,
    /// Field Representing column `type`
    pub type_: String,
    /// Field Representing column `created_at`
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Field Representing column `updated_at`
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Create struct for [`Todos`] on table `todos`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=todos)]
pub struct CreateTodos {
    /// Field Representing column `unsigned`
    pub unsigned: u32,
    /// Field Representing column `text`
    pub text: String,
    /// Field Representing column `completed`
    pub completed: bool,
    /// Field Representing column `type`
    pub type_: String,
}

/// Update struct for [`Todos`] on table `todos`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=todos)]
pub struct UpdateTodos {
    /// Field Representing column `unsigned`
    pub unsigned: Option<u32>,
    /// Field Representing column `text`
    pub text: Option<String>,
    /// Field Representing column `completed`
    pub completed: Option<bool>,
    /// Field Representing column `type`
    pub type_: Option<String>,
    /// Field Representing column `created_at`
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Field Representing column `updated_at`
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}


