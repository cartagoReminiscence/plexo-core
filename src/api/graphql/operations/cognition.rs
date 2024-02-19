use crate::api::graphql::commons::extract_context;
use async_graphql::{Context, Object, Result};

use plexo_sdk::cognition::operations::{CognitionOperations, SubdivideTaskInputBuilder, TaskSuggestion, TaskSuggestionInput};

use uuid::Uuid;

#[derive(Default)]
pub struct CognitionGraphQLQuery;

#[Object]
impl CognitionGraphQLQuery {
    async fn get_suggestions(&self, ctx: &Context<'_>, input: Option<TaskSuggestionInput>) -> Result<TaskSuggestion> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_suggestions(input.unwrap_or_default())
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn subdivide_task(&self, ctx: &Context<'_>, task_id: Uuid) -> Result<Vec<TaskSuggestion>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .subdivide_task(SubdivideTaskInputBuilder::default().task_id(task_id).build()?)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}
