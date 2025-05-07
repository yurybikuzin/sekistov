use super::*;

pub mod check;
pub mod upload;
pub mod wasm;

use crate::AppState;

use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Path},
    response::IntoResponse,
    Extension, Json,
};

pub fn router(shared_state: Arc<RwLock<AppState>>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(check::handler))
        .routes(routes!(upload::handler))
        .routes(routes!(wasm::handler))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .with_state(shared_state)
}
