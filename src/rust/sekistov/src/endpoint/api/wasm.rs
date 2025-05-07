use super::*;

#[utoipa::path(
    get,
    path = "/lib.wasm",
    tag = TODO_TAG,
)]
pub async fn handler(_state: Extension<SharedState>) -> Response {
    let body = include_bytes!("../../../../../zig/sekistov/lib.wasm");
    (
        [(axum::http::header::CONTENT_TYPE, "application/wasm")],
        body,
    )
        .into_response()
}
