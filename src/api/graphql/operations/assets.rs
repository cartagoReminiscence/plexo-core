use crate::api::graphql::commons::extract_context;
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::assets::{
    asset::Asset,
    operations::{AssetCrudOperations, CreateAssetInput, GetAssetsInput, UpdateAssetInput},
};
use tokio_stream::Stream;
use uuid::Uuid;

#[derive(Default)]
pub struct AssetsGraphQLQuery;

#[Object]
impl AssetsGraphQLQuery {
    async fn assets(&self, ctx: &Context<'_>, input: Option<GetAssetsInput>) -> Result<Vec<Asset>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_assets(input.unwrap_or_default())
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn asset(&self, ctx: &Context<'_>, id: Uuid) -> Result<Asset> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_asset(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

#[derive(Default)]
pub struct AssetsGraphQLMutation;

#[Object]
impl AssetsGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_asset(&self, ctx: &Context<'_>, input: CreateAssetInput) -> Result<Asset> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.owner_id = member_id;

        core.engine
            .create_asset(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn update_asset(&self, ctx: &Context<'_>, id: Uuid, input: UpdateAssetInput) -> Result<Asset> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .update_asset(id, input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn delete_asset(&self, ctx: &Context<'_>, id: Uuid) -> Result<Asset> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .delete_asset(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

#[derive(Default)]
pub struct AssetsGraphQLSubscription;

#[Subscription]
impl AssetsGraphQLSubscription {
    async fn events_asset(&self) -> impl Stream<Item = i32> {
        futures_util::stream::iter(0..10)
    }
}
