use async_graphql::{Context, Result};
use uuid::Uuid;

use crate::{auth::auth::PlexoAuthToken, core::core::Core, errors::app::PlexoAppError};

pub fn extract_context(ctx: &Context<'_>) -> Result<(Core, Uuid)> {
    let Ok(auth_token) = &ctx.data::<PlexoAuthToken>() else {
        return Err(PlexoAppError::MissingAuthorizationToken.into());
    };

    let plexo_engine = ctx.data::<Core>()?.to_owned();

    let Ok(claims) = plexo_engine.auth.extract_claims(auth_token) else {
        return Err(PlexoAppError::InvalidAuthorizationToken.into());
    };

    let member_id = claims.member_id();

    Ok((plexo_engine, member_id))
}
