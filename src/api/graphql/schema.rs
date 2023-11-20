use async_graphql::{MergedObject, MergedSubscription, Schema};
// use plexo_sdk::backend::engine::Engine;

use crate::core::app::Core;

use super::{
    auth::AuthMutation,
    operations::tasks::{TasksGraphQLMutation, TasksGraphQLQuery, TasksGraphQLSubscription},
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(TasksGraphQLQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(TasksGraphQLMutation, AuthMutation);

#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(TasksGraphQLSubscription);

pub trait GraphQLSchema {
    fn graphql_api_schema(&self) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
}

impl GraphQLSchema for Core {
    fn graphql_api_schema(&self) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> {
        Schema::build(
            QueryRoot::default(),
            MutationRoot::default(),
            SubscriptionRoot::default(),
        )
        .data(self.clone()) // TODO: Optimize this
        // .data(DataLoader::new(TaskLoader::new(self.clone()), tokio::spawn))
        // .data(DataLoader::new(
        //     ProjectLoader::new(self.clone()),
        //     tokio::spawn,
        // ))
        // .data(DataLoader::new(
        //     LabelLoader::new(self.clone()),
        //     tokio::spawn,
        // ))
        // .data(DataLoader::new(
        //     MemberLoader::new(self.clone()),
        //     tokio::spawn,
        // ))
        // .data(DataLoader::new(TeamLoader::new(self.clone()), tokio::spawn))
        .finish()
    }
}
