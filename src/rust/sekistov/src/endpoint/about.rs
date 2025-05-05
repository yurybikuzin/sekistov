use super::*;

pub async fn handler(state: Extension<SharedState>) -> Response {
    let AppState {
        op_mode,
        pkg_name,
        pkg_version,
        ..
    } = &*state.read().await;
    format!("{op_mode} {pkg_name} {pkg_version}\n").into_response()
}
