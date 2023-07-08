/* This file is generated and managed by dsync */

use crate::diesel::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;

type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row for table `users`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name=users, primary_key(name,address))]
pub struct Users {
    pub name: String,
    pub address: String,
    pub secret: String,
}

/// Create struct for [`Users`] on table `users`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=users)]
pub struct CreateUsers {
    pub name: String,
    pub address: String,
    pub secret: String,
}

/// Update struct for [`Users`] on table `users`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=users)]
pub struct UpdateUsers {
    pub secret: Option<String>,
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

impl Users {

    /// Insert a new row on users with a given [`CreateUsers`]
    pub fn create(db: &mut Connection, item: &CreateUsers) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        insert_into(users).values(item).get_result::<Self>(db)
    }

    /// Get a specific row with the primary key
    pub fn read(db: &mut Connection, param_name: String, param_address: String) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        users.filter(name.eq(param_name)).filter(address.eq(param_address)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::users::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = users.count().get_result(db)?;
        let items = users.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Update a row given the primary key with updates from [`UpdateUsers`]
    pub fn update(db: &mut Connection, param_name: String, param_address: String, item: &UpdateUsers) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        diesel::update(users.filter(name.eq(param_name)).filter(address.eq(param_address))).set(item).get_result(db)
    }

    /// Delete a row with the given primary key
    pub fn delete(db: &mut Connection, param_name: String, param_address: String) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(name.eq(param_name)).filter(address.eq(param_address))).execute(db)
    }

}
