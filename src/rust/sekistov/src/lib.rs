#[allow(unused_imports)]
use anyhow::{anyhow, bail, Error, Result};
#[allow(unused_imports)]
use tracing::{debug, error, info, span, trace, warn, Level};

use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Extension,
};
use common_macros::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use tokio::sync::RwLock;

mod endpoint;

declare_settings! {
    // keep_alive_secs: u64,
}

use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

const TODO_TAG: &str = "todo";

pub async fn server(
    port: u16,
    op_mode: op_mode::OpMode,
    pkg_name: &'static str,
    pkg_version: &'static str,
) -> Result<()> {
    let shared_state = Arc::new(tokio::sync::RwLock::new(AppState {
        pkg_name,
        pkg_version,
        op_mode,
    }));

    // OpenAPI example stolen from https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs
    #[derive(OpenApi)]
    #[openapi(
        // modifiers(&SecurityAddon),
        tags(
            (name = TODO_TAG, description = "Todo items management API")
        )
    )]
    struct ApiDoc;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", endpoint::api::router(shared_state.clone()))
        .split_for_parts();

    let app = router
        .route("/about", get(endpoint::about::handler))
        .layer(Extension(shared_state.clone()))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()))
        // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
        // via SwaggerUi instead we only make rapidoc to point to the existing doc.
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        // Alternative to above
        // .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", api).path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api))
        // .with_state(shared_state)
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("will start web server at PORT={port}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|err| anyhow!(err))
}

pub type SharedState = Arc<tokio::sync::RwLock<AppState>>;

pub struct AppState {
    pkg_name: &'static str,
    pkg_version: &'static str,
    op_mode: op_mode::OpMode,
}
