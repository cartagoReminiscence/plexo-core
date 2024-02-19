use plexo_sdk::backend::engine::{new_postgres_engine, SDKEngine};

use crate::{auth::engine::AuthEngine, errors::app::PlexoAppError};

use super::config::{
    DATABASE_URL, GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET, GITHUB_REDIRECT_URL, JWT_ACCESS_TOKEN_SECRET, LLM_API_KEY, LLM_MODEL_NAME,
};

#[derive(Clone)]
pub struct Core {
    pub engine: SDKEngine,
    pub auth: AuthEngine,
}

pub async fn new_core_from_env() -> Result<Core, PlexoAppError> {
    let engine = new_postgres_engine(
        DATABASE_URL.as_str(),
        false,
        LLM_API_KEY.to_owned(),
        LLM_MODEL_NAME.to_owned(),
    )
    .await?;

    let auth = AuthEngine::new(
        (*JWT_ACCESS_TOKEN_SECRET).to_string(),
        (*JWT_ACCESS_TOKEN_SECRET).to_string(),
        (*GITHUB_CLIENT_ID).to_owned(),
        (*GITHUB_CLIENT_SECRET).to_owned(),
        Some((*GITHUB_REDIRECT_URL).to_owned()),
    );

    Ok(Core { engine, auth })
}
