use async_graphql::{Context, InputObject, Object, Result};
use chrono::{DateTime, Utc};
use plexo_sdk::tasks::task::{Task, TaskPriority, TaskStatus};
use uuid::Uuid;

#[derive(Default)]
pub struct TasksGraphQLQuery;

#[derive(InputObject)]
pub struct TaskFilter {
    pub project_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub due_date_from: Option<DateTime<Utc>>,
    pub due_date_to: Option<DateTime<Utc>>,
}

#[Object]
impl TasksGraphQLQuery {
    async fn tasks(&self, ctx: &Context<'_>, _filter: Option<TaskFilter>) -> Result<Vec<Task>> {
        todo!()
    }
}
