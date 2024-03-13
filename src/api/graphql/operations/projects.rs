use crate::api::graphql::{commons::extract_context, resources::projects::Project};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::{
    changes::change::{ChangeResourceType, ListenEvent},
    projects::operations::{CreateProjectInput, GetProjectsInput, ProjectCrudOperations, UpdateProjectInput},
};
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

#[derive(Default)]
pub struct ProjectsGraphQLQuery;

#[Object]
impl ProjectsGraphQLQuery {
    async fn projects(&self, ctx: &Context<'_>, input: Option<GetProjectsInput>) -> Result<Vec<Project>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_projects(input.unwrap_or_default())
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|projects| projects.into_iter().map(|project| project.into()).collect())
    }

    async fn project(&self, ctx: &Context<'_>, id: Uuid) -> Result<Project> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_project(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|project| project.into())
    }
}

#[derive(Default)]
pub struct ProjectsGraphQLMutation;

#[Object]
impl ProjectsGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_project(&self, ctx: &Context<'_>, input: CreateProjectInput) -> Result<Project> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.owner_id = member_id;

        core.engine
            .create_project(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|project| project.into())
    }

    async fn update_project(&self, ctx: &Context<'_>, id: Uuid, input: UpdateProjectInput) -> Result<Project> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .update_project(id, input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|project| project.into())
    }

    async fn delete_project(&self, ctx: &Context<'_>, id: Uuid) -> Result<Project> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .delete_project(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|project| project.into())
    }
}

#[derive(Default)]
pub struct ProjectsGraphQLSubscription;

#[Subscription]
impl ProjectsGraphQLSubscription {
    async fn projects(&self, ctx: &Context<'_>) -> impl Stream<Item = ListenEvent> {
        let (core, _member_id) = extract_context(ctx).unwrap();

        core.engine
            .listen(ChangeResourceType::Projects)
            .await
            .unwrap()
            .map(|x| x.unwrap())
    }
}
