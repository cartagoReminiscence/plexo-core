use crate::api::graphql::{commons::extract_context, resources::teams::Team};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::{
    changes::change::{ChangeResourceType, ListenEvent},
    teams::operations::{CreateTeamInput, GetTeamsInput, TeamCrudOperations, UpdateTeamInput},
};

use tokio_stream::{Stream, StreamExt};
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
            .map(|teams| teams.into_iter().map(|team| team.into()).collect())
    }

    async fn team(&self, ctx: &Context<'_>, id: Uuid) -> Result<Team> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_team(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|team| team.into())
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
            .map(|team| team.into())
    }

    async fn update_team(&self, ctx: &Context<'_>, id: Uuid, input: UpdateTeamInput) -> Result<Team> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .update_team(id, input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|team| team.into())
    }

    async fn delete_team(&self, ctx: &Context<'_>, id: Uuid) -> Result<Team> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .delete_team(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|team| team.into())
    }
}

#[derive(Default)]
pub struct TeamsGraphQLSubscription;

#[Subscription]
impl TeamsGraphQLSubscription {
    async fn teams(&self, ctx: &Context<'_>) -> impl Stream<Item = ListenEvent> {
        let (core, _member_id) = extract_context(ctx).unwrap();

        core.engine
            .listen(ChangeResourceType::Teams)
            .await
            .unwrap()
            .map(|x| x.unwrap())
    }
}
