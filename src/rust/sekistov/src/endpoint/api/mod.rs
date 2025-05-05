use super::*;

pub mod check;
pub use check::*;

pub mod upload;
pub use upload::*;

pub mod wasm;
pub use wasm::*;

use crate::AppState;

use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Path, Query},
    response::IntoResponse,
    Extension, Json,
};

pub fn router(shared_state: Arc<RwLock<AppState>>) -> OpenApiRouter {
    let router = OpenApiRouter::new()
        .routes(routes!(check::handler))
        .routes(routes!(upload::handler))
        .routes(routes!(wasm::handler))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .with_state(shared_state);

    // let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
    //     .nest("/", router)
    //     .split_for_parts();
    //
    // let router = router
    //     ..merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
    //         .merge(Redoc::with_url("/redoc", api.clone()))
    //         // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
    //         // via SwaggerUi instead we only make rapidoc to point to the existing doc.
    //         .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    //         // Alternative to above
    //         // .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", api).path("/rapidoc"))
    //         .merge(Scalar::with_url("/scalar", api));

    router
}
