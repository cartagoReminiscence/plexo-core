use crate::api::graphql::{commons::extract_context, resources::labels::Label};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::labels::operations::{CreateLabelInput, GetLabelsInput, LabelCrudOperations, UpdateLabelInput};
use tokio_stream::Stream;
use uuid::Uuid;

#[derive(Default)]
pub struct LabelsGraphQLQuery;

#[Object]
impl LabelsGraphQLQuery {
    async fn labels(&self, ctx: &Context<'_>, input: Option<GetLabelsInput>) -> Result<Vec<Label>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_labels(input.unwrap_or_default())
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|labels| labels.into_iter().map(|label| label.into()).collect())
    }

    async fn label(&self, ctx: &Context<'_>, id: Uuid) -> Result<Label> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_label(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|label| label.into())
    }
}

#[derive(Default)]
pub struct LabelsGraphQLMutation;

#[Object]
impl LabelsGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_label(&self, ctx: &Context<'_>, input: CreateLabelInput) -> Result<Label> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .create_label(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|label| label.into())
    }

    async fn update_label(&self, ctx: &Context<'_>, id: Uuid, input: UpdateLabelInput) -> Result<Label> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .update_label(id, input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|label| label.into())
    }

    async fn delete_label(&self, ctx: &Context<'_>, id: Uuid) -> Result<Label> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .delete_label(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|label| label.into())
    }
}

#[derive(Default)]
pub struct LabelsGraphQLSubscription;

#[Subscription]
impl LabelsGraphQLSubscription {
    async fn events_label(&self) -> impl Stream<Item = i32> {
        futures_util::stream::iter(0..10)
    }
}
