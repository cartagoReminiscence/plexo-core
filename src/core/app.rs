use std::{str::FromStr, sync::Arc};

use plexo_sdk::backend::{
    engine::{SDKConfig, SDKEngine},
    loaders::SDKLoaders,
};

use crate::{auth::engine::AuthEngine, errors::app::PlexoAppError};

use super::config::{GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET, GITHUB_REDIRECT_URL, JWT_ACCESS_TOKEN_SECRET, TRACING_LEVEL};
use tracing::{subscriber::set_global_default, Level};

use tracing_subscriber::FmtSubscriber;
#[derive(Clone)]
pub struct Core {
    pub engine: SDKEngine,
    pub auth: AuthEngine,
    pub loaders: Arc<SDKLoaders>,
}

pub async fn new_core_from_env() -> Result<Core, PlexoAppError> {
    let engine = SDKEngine::new(SDKConfig::from_env()).await?;

    match engine.migrate().await {
        Ok(_) => println!("Database migration successful"),
        Err(err) => println!("Database migration failed: {}", err),
    }

    let arc_engine = Arc::new(engine.clone());

    let loaders = Arc::new(SDKLoaders::new(arc_engine));

    let auth = AuthEngine::new(
        (*JWT_ACCESS_TOKEN_SECRET).to_string(),
        (*JWT_ACCESS_TOKEN_SECRET).to_string(),
        (*GITHUB_CLIENT_ID).to_owned(),
        (*GITHUB_CLIENT_SECRET).to_owned(),
        Some((*GITHUB_REDIRECT_URL).to_owned()),
    );

    let tracing_level = Level::from_str((*TRACING_LEVEL).to_uppercase().as_str()).unwrap_or(Level::INFO);

    set_global_default(FmtSubscriber::builder().with_max_level(tracing_level).finish()).expect("setting default subscriber failed");

    Ok(Core { engine, auth, loaders })
}
