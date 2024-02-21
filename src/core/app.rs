use std::sync::Arc;

use plexo_sdk::backend::{
    engine::{SDKConfig, SDKEngine},
    loaders::SDKLoaders,
};

use crate::{auth::engine::AuthEngine, errors::app::PlexoAppError};

use super::config::{GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET, GITHUB_REDIRECT_URL, JWT_ACCESS_TOKEN_SECRET};

#[derive(Clone)]
pub struct Core {
    pub engine: SDKEngine,
    pub auth: AuthEngine,
    pub loaders: Arc<SDKLoaders>,
}

pub async fn new_core_from_env() -> Result<Core, PlexoAppError> {
    let engine = SDKEngine::new(SDKConfig::from_env()).await?;

    if let Err(err) = engine.migrate().await {
        println!("Database migration failed: {}", err);
    } else {
        println!("Database migration successful");
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

    Ok(Core { engine, auth, loaders })
}
