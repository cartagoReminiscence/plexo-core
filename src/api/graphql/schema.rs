use async_graphql::{
    extensions::{Analyzer, Tracing}, // extensions::OpenTelemetry,
    MergedObject,
    MergedSubscription,
    Schema,
};

use crate::core::app::Core;

use super::{
    operations::{
        assets::{AssetsGraphQLMutation, AssetsGraphQLQuery},
        auth::AuthMutation,
        changes::ChangesGraphQLQuery,
        labels::{LabelsGraphQLMutation, LabelsGraphQLQuery},
        members::{MembersGraphQLMutation, MembersGraphQLQuery},
        profile::{ProfileGraphQLMutation, ProfileGraphQLQuery},
        projects::{ProjectsGraphQLMutation, ProjectsGraphQLQuery},
        tasks::{TasksGraphQLMutation, TasksGraphQLQuery, TasksGraphQLSubscription},
        teams::{TeamsGraphQLMutation, TeamsGraphQLQuery},
    },
    processors::ai::AIProcessorGraphQLQuery,
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    TasksGraphQLQuery,
    AssetsGraphQLQuery,
    LabelsGraphQLQuery,
    ProjectsGraphQLQuery,
    TeamsGraphQLQuery,
    MembersGraphQLQuery,
    ChangesGraphQLQuery,
    AIProcessorGraphQLQuery,
    ProfileGraphQLQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    TasksGraphQLMutation,
    AuthMutation,
    AssetsGraphQLMutation,
    LabelsGraphQLMutation,
    ProjectsGraphQLMutation,
    TeamsGraphQLMutation,
    MembersGraphQLMutation,
    ProfileGraphQLMutation,
    // ChangesGraphQLMutation,
);

#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(TasksGraphQLSubscription);

pub trait GraphQLSchema {
    fn graphql_api_schema(&self) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
}

impl GraphQLSchema for Core {
    fn graphql_api_schema(&self) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> {
        Schema::build(QueryRoot::default(), MutationRoot::default(), SubscriptionRoot::default())
            .data(self.clone()) // TODO: Optimize this
            .extension(Tracing)
            .extension(Analyzer)
            // .extension(open_telemetry)
            .finish()
    }
}
