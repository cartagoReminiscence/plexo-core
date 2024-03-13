use crate::api::graphql::{commons::extract_context, resources::tasks::Task};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::{
    changes::{
        change::{ChangeOperation, ChangeResourceType},
        operations::{ChangeCrudOperations, CreateChangeInputBuilder},
    },
    tasks::{
        extensions::{CreateTasksInput, TasksExtensionOperations},
        operations::{CreateTaskInput, GetTasksInput, TaskCrudOperations, UpdateTaskInput},
    },
};
use serde_json::json;
use tokio::task;
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
            .map(|tasks| tasks.into_iter().map(|task| task.into()).collect())
    }

    async fn task(&self, ctx: &Context<'_>, id: Uuid) -> Result<Task> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_task(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|task| task.into())
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

        let saved_input = input.clone();

        let task = core.engine.create_task(input).await?;
        let saved_task = task.clone();

        let input = saved_input.clone();
        // let task = task.clone();
        let engine = core.engine.clone();

        task::spawn(async move {
            let change = engine
                .create_change(
                    CreateChangeInputBuilder::default()
                        .owner_id(member_id)
                        .resource_id(task.id)
                        .operation(ChangeOperation::Create)
                        .resource_type(ChangeResourceType::Task)
                        .diff_json(
                            serde_json::to_string(&json!({
                                "input": input,
                                "result": task,
                            }))
                            .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .await
                .unwrap();

            println!("change registered: {} | {}", change.operation, change.resource_type);
        });

        Ok(saved_task.into())
    }

    async fn create_tasks(&self, ctx: &Context<'_>, input: CreateTasksInput) -> Result<Vec<Task>> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.tasks.iter_mut().for_each(|task| task.owner_id = member_id);

        let saved_input = input.clone();

        let tasks = core.engine.create_tasks(input).await?;
        let saved_tasks = tasks.clone();

        // .map_err(|err| async_graphql::Error::new(err.to_string()))
        // .map(|tasks| tasks.into_iter().map(|task| task.into()).collect())

        tasks.iter().for_each(|task| {
            let engine = core.engine.clone();
            let input = saved_input.clone();
            let task = task.clone();

            task::spawn(async move {
                let change = engine
                    .create_change(
                        CreateChangeInputBuilder::default()
                            .owner_id(member_id)
                            .resource_id(task.id)
                            .operation(ChangeOperation::Create)
                            .resource_type(ChangeResourceType::Task)
                            .diff_json(
                                serde_json::to_string(&json!({
                                    "input": input,
                                    "result": task,
                                }))
                                .unwrap(),
                            )
                            .build()
                            .unwrap(),
                    )
                    .await
                    .unwrap();

                println!("change registered: {} | {}", change.operation, change.resource_type);
            });
        });

        Ok(saved_tasks.into_iter().map(|task| task.into()).collect())
    }

    async fn update_task(&self, ctx: &Context<'_>, id: Uuid, input: UpdateTaskInput) -> Result<Task> {
        let (core, member_id) = extract_context(ctx)?;

        let saved_input = input.clone();

        let task = core.engine.update_task(id, input).await?;

        let task = task.clone();
        let saved_task = task.clone();
        let engine = core.engine.clone();

        // .map_err(|err| async_graphql::Error::new(err.to_string()))
        // .map(|task| task.into())

        tokio::spawn(async move {
            let change = engine
                .create_change(
                    CreateChangeInputBuilder::default()
                        .owner_id(member_id)
                        .resource_id(task.id)
                        .operation(ChangeOperation::Update)
                        .resource_type(ChangeResourceType::Task)
                        .diff_json(
                            serde_json::to_string(&json!({
                                "input": saved_input,
                                "result": task,
                            }))
                            .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .await
                .unwrap();

            println!("change registered: {} | {}", change.operation, change.resource_type);
        });

        Ok(saved_task.into())
    }

    async fn delete_task(&self, ctx: &Context<'_>, id: Uuid) -> Result<Task> {
        let (core, _member_id) = extract_context(ctx)?;

        let task = core.engine.delete_task(id).await?;
        let saved_task = task.clone();

        // .map_err(|err| async_graphql::Error::new(err.to_string()))
        // .map(|task| task.into())

        let engine = core.engine.clone();

        tokio::spawn(async move {
            let change = engine
                .create_change(
                    CreateChangeInputBuilder::default()
                        .owner_id(task.owner_id)
                        .resource_id(task.id)
                        .operation(ChangeOperation::Delete)
                        .resource_type(ChangeResourceType::Task)
                        .diff_json(
                            serde_json::to_string(&json!({
                                "result": task,
                            }))
                            .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .await
                .unwrap();

            println!("change registered: {} | {}", change.operation, change.resource_type);
        });

        Ok(saved_task.into())
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
