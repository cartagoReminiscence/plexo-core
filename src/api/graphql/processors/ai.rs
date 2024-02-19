use async_graphql::{Context, InputObject, Object, Result};

use plexo_sdk::resources::tasks::task::Task;
use uuid::Uuid;

use crate::api::graphql::commons::extract_context;

#[derive(Default)]
pub struct AIProcessorGraphQLQuery;

#[derive(Default)]
pub struct AIProcessorGraphQLMutation;

#[derive(InputObject)]
struct SuggestNextTaskInput {
    project_id: Uuid,
    query: Option<String>,
}

#[derive(InputObject)]
struct SubdivideTaskInput {
    task_id: Uuid,
    #[graphql(default = 3)]
    total_subtasks: u32,
    query: Option<String>,
}

#[Object]
impl AIProcessorGraphQLQuery {
    async fn suggest_next_task(&self, ctx: &Context<'_>, _input: SuggestNextTaskInput) -> Result<Task> {
        let (_core, _member_id) = extract_context(ctx)?;
        todo!()
    }

    async fn subdivide_task(&self, ctx: &Context<'_>, _input: SubdivideTaskInput) -> Result<Vec<Task>> {
        let (_core, _member_id) = extract_context(ctx)?;
        todo!()
    }
}
