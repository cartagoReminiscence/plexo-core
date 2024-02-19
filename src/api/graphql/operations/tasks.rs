use crate::api::graphql::commons::extract_context;
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::tasks::{
    operations::{CreateTaskInput, GetTasksInput, TaskCrudOperations, UpdateTaskInput},
    task::Task,
};
use tokio_stream::Stream;
use uuid::Uuid;

#[derive(Default)]
pub struct TasksGraphQLQuery;

#[Object]
impl TasksGraphQLQuery {
    async fn tasks(&self, ctx: &Context<'_>, input: Option<GetTasksInput>) -> Result<Vec<Task>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_tasks(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn task(&self, ctx: &Context<'_>, id: Uuid) -> Result<Task> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_task(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

#[derive(Default)]
pub struct TasksGraphQLMutation;

#[Object]
impl TasksGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_task(&self, ctx: &Context<'_>, input: CreateTaskInput) -> Result<Task> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.owner_id = member_id;

        core.engine
            .create_task(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn update_task(&self, ctx: &Context<'_>, id: Uuid, input: UpdateTaskInput) -> Result<Task> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .update_task(id, input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn delete_task(&self, ctx: &Context<'_>, id: Uuid) -> Result<Task> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .delete_task(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

#[derive(Default)]
pub struct TasksGraphQLSubscription;

#[Subscription]
impl TasksGraphQLSubscription {
    async fn events1(&self) -> impl Stream<Item = i32> {
        futures_util::stream::iter(0..10)
    }
}
