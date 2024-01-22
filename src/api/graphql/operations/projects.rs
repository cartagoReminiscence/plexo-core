use crate::api::graphql::commons::extract_context;
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::projects::{
    operations::{CreateProjectInput, GetProjectsInput, ProjectCrudOperations, UpdateProjectInput},
    project::Project,
};
use tokio_stream::Stream;
use uuid::Uuid;

#[derive(Default)]
pub struct ProjectsGraphQLQuery;

#[Object]
impl ProjectsGraphQLQuery {
    async fn projects(&self, ctx: &Context<'_>, input: GetProjectsInput) -> Result<Vec<Project>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_projects(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn project(&self, ctx: &Context<'_>, id: Uuid) -> Result<Project> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_project(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

#[derive(Default)]
pub struct ProjectsGraphQLMutation;

#[Object]
impl ProjectsGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_project(&self, ctx: &Context<'_>, mut input: CreateProjectInput) -> Result<Project> {
        let (core, member_id) = extract_context(ctx)?;

        input.owner_id = member_id;

        core.engine
            .create_project(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn update_project(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: UpdateProjectInput,
    ) -> Result<Project> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .update_project(id, input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn delete_project(&self, ctx: &Context<'_>, id: Uuid) -> Result<Project> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .delete_project(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

#[derive(Default)]
pub struct ProjectsGraphQLSubscription;

#[Subscription]
impl ProjectsGraphQLSubscription {
    async fn events_project(&self) -> impl Stream<Item = i32> {
        futures_util::stream::iter(0..10)
    }
}

