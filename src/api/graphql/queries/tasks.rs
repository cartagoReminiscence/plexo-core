use crate::api::graphql::commons::extract_context;
use async_graphql::{Context, Object, Result};

use plexo_sdk::tasks::{
    operations::{GetTasksInput, GetTasksInputBuilder, TaskCrudOperations},
    task::Task,
};

#[derive(Default)]
pub struct TasksGraphQLQuery;

#[Object]
impl TasksGraphQLQuery {
    async fn tasks(&self, ctx: &Context<'_>, input: Option<GetTasksInput>) -> Result<Vec<Task>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_tasks(input.unwrap_or(GetTasksInputBuilder::default().build()?))
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}
