use std::env::var;

use dotenv::dotenv;
use plexo_sdk::{
    backend::engine::new_postgres_engine,
    tasks::operations::{GetTasksInputBuilder, TaskCrudOperations},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = var("DATABASE_URL").unwrap();

    let engine = new_postgres_engine(database_url.as_str()).await.unwrap();

    let tasks = engine
        .get_tasks(GetTasksInputBuilder::default().build().unwrap())
        .await
        .unwrap();

    println!("total tasks: {}", tasks.len());
}
