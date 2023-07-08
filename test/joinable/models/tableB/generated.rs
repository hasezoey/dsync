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

/// Result of a `.paginate` function
#[derive(Debug, Serialize)]
pub struct PaginationResult<T> {
    /// Result items from the current page
    pub items: Vec<T>,
    /// Count of how many items there are in total
    pub total_items: i64,
    /// Current page, 0-based index
    pub page: i64,
    /// Size of a page
    pub page_size: i64,
    /// Number of pages in total
    pub num_pages: i64,
}

impl TableB {

    /// Insert a new row on tableB with a given [`CreateTableB`]
    pub fn create(db: &mut Connection, item: &CreateTableB) -> QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        insert_into(tableB).values(item).get_result::<Self>(db)
    }

    /// Get a specific row with the primary key
    pub fn read(db: &mut Connection, param__id: i32) -> QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        tableB.filter(_id.eq(param__id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::tableB::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = tableB.count().get_result(db)?;
        let items = tableB.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Update a row given the primary key with updates from [`UpdateTableB`]
    pub fn update(db: &mut Connection, param__id: i32, item: &UpdateTableB) -> QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        diesel::update(tableB.filter(_id.eq(param__id))).set(item).get_result(db)
    }

    /// Delete a row with the given primary key
    pub fn delete(db: &mut Connection, param__id: i32) -> QueryResult<usize> {
        use crate::schema::tableB::dsl::*;

        diesel::delete(tableB.filter(_id.eq(param__id))).execute(db)
    }

}
