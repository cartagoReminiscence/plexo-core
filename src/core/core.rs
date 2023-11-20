use plexo_sdk::backend::engine::SDKEngine;

use crate::auth::engine::AuthEngine;

#[derive(Clone)]
pub struct Core {
    pub engine: SDKEngine,
    pub auth: AuthEngine,
}
