/* This file is generated and managed by dsync */

use crate::diesel::*;
use serde::{Deserialize, Serialize};
use crate::data::schema::*;

/// Connection Type as set in dsync
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

impl TableA {

    /// Insert a new row on tableA with a given [`CreateTableA`]
    pub fn create(db: &mut Connection, item: &CreateTableA) -> QueryResult<Self> {
        use crate::data::schema::tableA::dsl::*;

        insert_into(tableA).values(item).get_result::<Self>(db)
    }

    /// Get a specific row with the primary key
    pub fn read(db: &mut Connection, param__id: i32) -> QueryResult<Self> {
        use crate::data::schema::tableA::dsl::*;

        tableA.filter(_id.eq(param__id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::data::schema::tableA::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = tableA.count().get_result(db)?;
        let items = tableA.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Delete a row with the given primary key
    pub fn delete(db: &mut Connection, param__id: i32) -> QueryResult<usize> {
        use crate::data::schema::tableA::dsl::*;

        diesel::delete(tableA.filter(_id.eq(param__id))).execute(db)
    }

}
