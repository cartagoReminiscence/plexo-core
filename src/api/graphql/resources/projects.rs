use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use plexo_sdk::resources::{
    assets::asset::Asset,
    changes::change::Change,
    members::member::Member,
    projects::{project::Project as SDKProject, relations::ProjectRelations},
    teams::team::Team,
};

use crate::api::graphql::commons::extract_context;

use super::tasks::Task;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Project {
    #[graphql(flatten)]
    project: SDKProject,
}

impl From<SDKProject> for Project {
    fn from(val: SDKProject) -> Self {
        Project { project: val }
    }
}

#[ComplexObject]
impl Project {
    async fn owner(&self, ctx: &Context<'_>) -> Result<Member> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.project.owner(&plexo_engine.loaders).await.map_err(|e| e.into())
    }

    async fn lead(&self, ctx: &Context<'_>) -> Result<Member> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.project.lead(&plexo_engine.loaders).await.map_err(|e| e.into())
    }

    async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<Task>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.project
            .tasks(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|tasks| tasks.into_iter().map(|task| task.into()).collect())
    }

    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<Member>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.project.members(&plexo_engine.loaders).await.map_err(|e| e.into())
    }

    async fn assets(&self, ctx: &Context<'_>) -> Result<Vec<Asset>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.project.assets(&plexo_engine.loaders).await.map_err(|e| e.into())
    }

    async fn teams(&self, ctx: &Context<'_>) -> Result<Vec<Team>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.project.teams(&plexo_engine.loaders).await.map_err(|e| e.into())
    }

    async fn changes(&self, ctx: &Context<'_>) -> Result<Vec<Change>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.project.changes(&plexo_engine.loaders).await.map_err(|e| e.into())
    }
}
