use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use plexo_sdk::resources::{
    changes::change::Change,
    labels::label::Label,
    members::member::Member,
    projects::project::Project,
    tasks::{relations::TaskRelations, task::Task as SDKTask},
};

use crate::api::graphql::commons::extract_context;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Task {
    #[graphql(flatten)]
    task: SDKTask,
}

impl From<SDKTask> for Task {
    fn from(val: SDKTask) -> Self {
        Task { task: val }
    }
}

#[ComplexObject]
impl Task {
    async fn owner(&self, ctx: &Context<'_>) -> Result<Member> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task.owner(&plexo_engine.loaders).await.map_err(|e| e.into())
    }

    async fn project(&self, ctx: &Context<'_>) -> Result<Option<Project>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task.project(&plexo_engine.loaders).await.map_err(|e| e.into())
    }

    async fn lead(&self, ctx: &Context<'_>) -> Result<Option<Member>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task.lead(&plexo_engine.loaders).await.map_err(|e| e.into())
    }

    async fn parent(&self, ctx: &Context<'_>) -> Result<Option<Task>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task
            .parent(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|task| task.map(|t| t.into()))
    }

    async fn assignees(&self, ctx: &Context<'_>) -> Result<Vec<Member>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task.assignees(&plexo_engine.loaders).await.map_err(|e| e.into())
    }

    async fn labels(&self, ctx: &Context<'_>) -> Result<Vec<Label>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task.labels(&plexo_engine.loaders).await.map_err(|e| e.into())
    }

    async fn subtasks(&self, ctx: &Context<'_>) -> Result<Vec<Task>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task
            .subtasks(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|tasks| tasks.into_iter().map(|task| task.into()).collect())
    }

    async fn changes(&self, ctx: &Context<'_>) -> Result<Vec<Change>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task.changes(&plexo_engine.loaders).await.map_err(|e| e.into())
    }
}
