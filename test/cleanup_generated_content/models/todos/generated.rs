/* This file is generated and managed by dsync */

use crate::diesel::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;

type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row for table `todos`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todos {
    /// Field Representing column `id`
    pub id: i32,
    /// Field Representing column `text`
    pub text: String,
    /// Field Representing column `completed`
    pub completed: bool,
    /// Field Representing column `created_at`
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Field Representing column `updated_at`
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Create struct for [`Todos`] on table `todos`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=todos)]
pub struct CreateTodos {
    /// Field Representing column `id`
    pub id: i32,
    /// Field Representing column `text`
    pub text: String,
    /// Field Representing column `completed`
    pub completed: bool,
}

/// Update struct for [`Todos`] on table `todos`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=todos)]
pub struct UpdateTodos {
    /// Field Representing column `text`
    pub text: Option<String>,
    /// Field Representing column `completed`
    pub completed: Option<bool>,
    /// Field Representing column `created_at`
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Field Representing column `updated_at`
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
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

impl Todos {

    /// Insert a new row on todos with a given [`CreateTodos`]
    pub fn create(db: &mut Connection, item: &CreateTodos) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        insert_into(todos).values(item).get_result::<Self>(db)
    }

    /// Get a specific row with the primary key
    pub fn read(db: &mut Connection, param_id: i32) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        todos.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::todos::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = todos.count().get_result(db)?;
        let items = todos.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Update a row given the primary key with updates from [`UpdateTodos`]
    pub fn update(db: &mut Connection, param_id: i32, item: &UpdateTodos) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        diesel::update(todos.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row with the given primary key
    pub fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::todos::dsl::*;

        diesel::delete(todos.filter(id.eq(param_id))).execute(db)
    }

}
