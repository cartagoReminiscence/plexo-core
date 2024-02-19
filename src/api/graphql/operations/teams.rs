use crate::api::graphql::commons::extract_context;
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::teams::{
    operations::{CreateTeamInput, GetTeamsInput, TeamCrudOperations, UpdateTeamInput},
    team::Team,
};
use tokio_stream::Stream;
use uuid::Uuid;

#[derive(Default)]
pub struct TeamsGraphQLQuery;

#[Object]
impl TeamsGraphQLQuery {
    async fn teams(&self, ctx: &Context<'_>, input: Option<GetTeamsInput>) -> Result<Vec<Team>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_teams(input.unwrap_or_default())
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn team(&self, ctx: &Context<'_>, id: Uuid) -> Result<Team> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_team(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

#[derive(Default)]
pub struct TeamsGraphQLMutation;

#[Object]
impl TeamsGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_team(&self, ctx: &Context<'_>, input: CreateTeamInput) -> Result<Team> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.owner_id = member_id;

        core.engine
            .create_team(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn update_team(&self, ctx: &Context<'_>, id: Uuid, input: UpdateTeamInput) -> Result<Team> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .update_team(id, input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn delete_team(&self, ctx: &Context<'_>, id: Uuid) -> Result<Team> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .delete_team(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

#[derive(Default)]
pub struct TeamsGraphQLSubscription;

#[Subscription]
impl TeamsGraphQLSubscription {
    async fn events_team(&self) -> impl Stream<Item = i32> {
        futures_util::stream::iter(0..10)
    }
}
