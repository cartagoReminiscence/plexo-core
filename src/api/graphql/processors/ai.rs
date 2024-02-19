use async_graphql::{Context, Object, Result};

use plexo_sdk::cognition::operations::{CognitionOperations, SubdivideTaskInput, TaskSuggestion, TaskSuggestionInput};

use crate::api::graphql::commons::extract_context;

#[derive(Default)]
pub struct AIProcessorGraphQLQuery;

#[derive(Default)]
pub struct AIProcessorGraphQLMutation;

#[Object]
impl AIProcessorGraphQLQuery {
    async fn suggest_next_task(&self, ctx: &Context<'_>, input: TaskSuggestionInput) -> Result<TaskSuggestion> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_suggestions(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn subdivide_task(&self, ctx: &Context<'_>, input: SubdivideTaskInput) -> Result<Vec<TaskSuggestion>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .subdivide_task(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}
