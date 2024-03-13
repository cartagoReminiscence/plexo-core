use crate::api::graphql::{commons::extract_context, resources::assets::Asset};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::{
    assets::operations::{AssetCrudOperations, CreateAssetInput, GetAssetsInput, UpdateAssetInput},
    changes::change::{ChangeResourceType, ListenEvent},
};
use tokio_stream::{Stream, StreamExt};
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
            .map(|assets| assets.into_iter().map(|asset| asset.into()).collect())
    }

    async fn asset(&self, ctx: &Context<'_>, id: Uuid) -> Result<Asset> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_asset(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|asset| asset.into())
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
            .map(|asset| asset.into())
    }

    async fn update_asset(&self, ctx: &Context<'_>, id: Uuid, input: UpdateAssetInput) -> Result<Asset> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .update_asset(id, input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|asset| asset.into())
    }

    async fn delete_asset(&self, ctx: &Context<'_>, id: Uuid) -> Result<Asset> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .delete_asset(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|asset| asset.into())
    }
}

#[derive(Default)]
pub struct AssetsGraphQLSubscription;

#[Subscription]
impl AssetsGraphQLSubscription {
    async fn assets(&self, ctx: &Context<'_>) -> impl Stream<Item = ListenEvent> {
        let (core, _member_id) = extract_context(ctx).unwrap();

        core.engine
            .listen(ChangeResourceType::Assets)
            .await
            .unwrap()
            .map(|x| x.unwrap())
    }
}
