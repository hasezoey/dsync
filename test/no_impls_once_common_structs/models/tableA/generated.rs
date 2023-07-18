/* This file is generated and managed by dsync */

use crate::diesel::*;
use serde::{Deserialize, Serialize};
use crate::models::common::*;
use crate::schema::*;

type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row for table `tableA`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name=tableA, primary_key(_id))]
pub struct TableA {
    /// Field Representing column `_id`
    pub _id: i32,
}

/// Create struct for [`TableA`] on table `tableA`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name=tableA)]
pub struct CreateTableA {
    /// Field Representing column `_id`
    pub _id: i32,
}


