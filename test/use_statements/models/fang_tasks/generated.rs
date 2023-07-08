/* This file is generated and managed by dsync */

use crate::diesel::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;

type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row for table `fang_tasks`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name=fang_tasks, primary_key(id))]
pub struct FangTasks {
    /// Field Representing column `id`
    pub id: uuid::Uuid,
    /// Field Representing column `metadata`
    pub metadata: serde_json::Value,
    /// Field Representing column `error_message`
    pub error_message: Option<String>,
    /// Field Representing column `state`
    pub state: crate::schema::sql_types::FangTaskState,
    /// Field Representing column `task_type`
    pub task_type: String,
    /// Field Representing column `uniq_hash`
    pub uniq_hash: Option<String>,
    /// Field Representing column `retries`
    pub retries: i32,
    /// Field Representing column `scheduled_at`
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    /// Field Representing column `created_at`
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Field Representing column `updated_at`
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Create struct for [`FangTasks`] on table `fang_tasks`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=fang_tasks)]
pub struct CreateFangTasks {
    /// Field Representing column `id`
    pub id: uuid::Uuid,
    /// Field Representing column `metadata`
    pub metadata: serde_json::Value,
    /// Field Representing column `error_message`
    pub error_message: Option<String>,
    /// Field Representing column `state`
    pub state: crate::schema::sql_types::FangTaskState,
    /// Field Representing column `task_type`
    pub task_type: String,
    /// Field Representing column `uniq_hash`
    pub uniq_hash: Option<String>,
    /// Field Representing column `retries`
    pub retries: i32,
    /// Field Representing column `scheduled_at`
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    /// Field Representing column `created_at`
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Field Representing column `updated_at`
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Update struct for [`FangTasks`] on table `fang_tasks`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=fang_tasks)]
pub struct UpdateFangTasks {
    /// Field Representing column `metadata`
    pub metadata: Option<serde_json::Value>,
    /// Field Representing column `error_message`
    pub error_message: Option<Option<String>>,
    /// Field Representing column `state`
    pub state: Option<crate::schema::sql_types::FangTaskState>,
    /// Field Representing column `task_type`
    pub task_type: Option<String>,
    /// Field Representing column `uniq_hash`
    pub uniq_hash: Option<Option<String>>,
    /// Field Representing column `retries`
    pub retries: Option<i32>,
    /// Field Representing column `scheduled_at`
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
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

impl FangTasks {

    /// Insert a new row on fang_tasks with a given [`CreateFangTasks`]
    pub fn create(db: &mut Connection, item: &CreateFangTasks) -> QueryResult<Self> {
        use crate::schema::fang_tasks::dsl::*;

        insert_into(fang_tasks).values(item).get_result::<Self>(db)
    }

    /// Get a specific row with the primary key
    pub fn read(db: &mut Connection, param_id: uuid::Uuid) -> QueryResult<Self> {
        use crate::schema::fang_tasks::dsl::*;

        fang_tasks.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::fang_tasks::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = fang_tasks.count().get_result(db)?;
        let items = fang_tasks.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Update a row given the primary key with updates from [`UpdateFangTasks`]
    pub fn update(db: &mut Connection, param_id: uuid::Uuid, item: &UpdateFangTasks) -> QueryResult<Self> {
        use crate::schema::fang_tasks::dsl::*;

        diesel::update(fang_tasks.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row with the given primary key
    pub fn delete(db: &mut Connection, param_id: uuid::Uuid) -> QueryResult<usize> {
        use crate::schema::fang_tasks::dsl::*;

        diesel::delete(fang_tasks.filter(id.eq(param_id))).execute(db)
    }

}
