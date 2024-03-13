use crate::api::graphql::{commons::extract_context, resources::changes::Change};
use async_graphql::{Context, Object, Result};

use plexo_sdk::resources::changes::operations::{ChangeCrudOperations, CreateChangeInput, GetChangesInput, UpdateChangeInput};
// use tokio_stream::Stream;
use uuid::Uuid;

#[derive(Default)]
pub struct ChangesGraphQLQuery;

#[Object]
impl ChangesGraphQLQuery {
    async fn changes(&self, ctx: &Context<'_>, input: Option<GetChangesInput>) -> Result<Vec<Change>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_changes(input.unwrap_or_default())
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|changes| changes.into_iter().map(|change| change.into()).collect())
    }

    async fn change(&self, ctx: &Context<'_>, id: Uuid) -> Result<Change> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_change(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|change| change.into())
    }
}

#[derive(Default)]
pub struct ChangesGraphQLMutation;

#[Object]
impl ChangesGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_change(&self, ctx: &Context<'_>, input: CreateChangeInput) -> Result<Change> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.owner_id = member_id;

        core.engine
            .create_change(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|change| change.into())
    }

    async fn update_change(&self, ctx: &Context<'_>, id: Uuid, input: UpdateChangeInput) -> Result<Change> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .update_change(id, input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|change| change.into())
    }

    async fn delete_change(&self, ctx: &Context<'_>, id: Uuid) -> Result<Change> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .delete_change(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|change| change.into())
    }
}

#[derive(Default)]
pub struct ChangesGraphQLSubscription;

// #[Subscription]
// impl ChangesGraphQLSubscription {
//     async fn events_change(&self) -> impl Stream<Item = i32> {
//         todo!()
//     }
// }
