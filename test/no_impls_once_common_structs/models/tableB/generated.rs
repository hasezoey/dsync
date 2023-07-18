/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::models::table_a::TableA;
use serde::{Deserialize, Serialize};
use crate::schema::*;

type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row for table `tableB`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable, Identifiable, Associations, AsChangeset)]
#[diesel(table_name=tableB, primary_key(_id), belongs_to(TableA, foreign_key=link))]
pub struct TableB {
    /// Field Representing column `_id`
    pub _id: i32,
    /// Field Representing column `link`
    pub link: i32,
}

/// Create struct for [`TableB`] on table `tableB`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=tableB)]
pub struct CreateTableB {
    /// Field Representing column `_id`
    pub _id: i32,
    /// Field Representing column `link`
    pub link: i32,
}

/// Update struct for [`TableB`] on table `tableB`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=tableB)]
pub struct UpdateTableB {
    /// Field Representing column `link`
    pub link: Option<i32>,
}


