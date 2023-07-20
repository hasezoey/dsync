/* This file is generated and managed by dsync */

use crate::diesel::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;

/// Connection Type as set in dsync
type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row for table `test`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name=test, primary_key(id))]
pub struct Test {
    /// Field Representing column `id`
    pub id: i32,
    /// Field Representing column `completed`
    pub completed: bool,
}

/// Create struct for [`Test`] on table `test`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=test)]
pub struct CreateTest {
    /// Field Representing column `completed`
    pub completed: bool,
}

/// Update struct for [`Test`] on table `test`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=test)]
pub struct UpdateTest {
    /// Field Representing column `completed`
    pub completed: Option<bool>,
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

impl Test {

    /// Insert a new row on test with a given [`CreateTest`]
    pub fn create(db: &mut Connection, item: &CreateTest) -> QueryResult<Self> {
        use crate::schema::test::dsl::*;

        insert_into(test).values(item).get_result::<Self>(db)
    }

    /// Get a specific row with the primary key
    pub fn read(db: &mut Connection, param_id: i32) -> QueryResult<Self> {
        use crate::schema::test::dsl::*;

        test.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::test::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = test.count().get_result(db)?;
        let items = test.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Update a row given the primary key with updates from [`UpdateTest`]
    pub fn update(db: &mut Connection, param_id: i32, item: &UpdateTest) -> QueryResult<Self> {
        use crate::schema::test::dsl::*;

        diesel::update(test.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row with the given primary key
    pub fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::test::dsl::*;

        diesel::delete(test.filter(id.eq(param_id))).execute(db)
    }

}
