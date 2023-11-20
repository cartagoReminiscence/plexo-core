use async_graphql::{dataloader::DataLoader, MergedObject, Schema};
// use plexo_sdk::backend::engine::Engine;

use super::tasks::TasksGraphQLQuery;

#[derive(MergedObject, Default)]
pub struct QueryRoot(TasksGraphQLQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot;

#[derive(MergedObject, Default)]
pub struct SubscriptionRoot;

pub trait GraphQLSchema {
    fn graphql_api_schema(&self) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
}

// impl GraphQLSchema for Engine<Postre> {
//     fn graphql_api_schema(&self) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> {
//         Schema::build(
//             QueryRoot::default(),
//             MutationRoot::default(),
//             SubscriptionRoot,
//         )
//         .data(self.clone()) // TODO: Optimize this
//         .data(DataLoader::new(TaskLoader::new(self.clone()), tokio::spawn))
//         .data(DataLoader::new(
//             ProjectLoader::new(self.clone()),
//             tokio::spawn,
//         ))
//         .data(DataLoader::new(
//             LabelLoader::new(self.clone()),
//             tokio::spawn,
//         ))
//         .data(DataLoader::new(
//             MemberLoader::new(self.clone()),
//             tokio::spawn,
//         ))
//         .data(DataLoader::new(TeamLoader::new(self.clone()), tokio::spawn))
//         .finish()
//     }
// }
