use super::*;

// #[derive(Serialize, Default, ToSchema)]
// #[schema(example = "OK")]
// pub struct Meta {
//     pub file_name: Option<String>,
//     pub timestamp: Option<u32>,
//     pub file_size: u64,
// }
#[utoipa::path(
    get,
    // path = "/api/lib.wasm",
    path = "/lib.wasm",
    tag = TODO_TAG,
    // responses(
    //     (status = 200, description = "Get file description", body = Option<Meta>)
    // )
)]
pub async fn handler(
    state: Extension<SharedState>,
    // Path(file_id): Path<String>,
) -> Response {
    let body = include_bytes!("../../../../../zig/sekistov/lib.wasm");
    (
        [(axum::http::header::CONTENT_TYPE, "application/wasm")],
        body,
    )
        .into_response()
    // info!("file_id: {file_id}");
    // let mut orig_file_path = {
    //     let mut ret = std::path::PathBuf::from("video");
    //     ret.push(file_id);
    //     ret.push("orig");
    //     ret
    // };
    // if let Ok(metadata) = orig_file_path.metadata() {
    //     let mut ret = Meta::default();
    //     ret.file_size = metadata.len();
    //     let meta_file_path = {
    //         let mut ret = orig_file_path;
    //         ret.set_file_name("meta.yaml");
    //         ret
    //     };
    //
    //     if let Some((file_name, timestamp)) = tokio::fs::read_to_string(meta_file_path)
    //         .await
    //         .ok()
    //         .and_then(|s| {
    //             #[derive(Deserialize)]
    //             struct Meta {
    //                 file_name: String,
    //                 timestamp: u32,
    //             }
    //             serde_yml::from_str::<Meta>(&s)
    //                 .map(
    //                     |Meta {
    //                          file_name,
    //                          timestamp,
    //                      }| (file_name, timestamp),
    //                 )
    //                 .ok()
    //         })
    //     {
    //         ret.file_name = Some(file_name);
    //         ret.timestamp = Some(timestamp);
    //     }
    //     Json(Some(ret))
    // } else {
    //     Json(None)
    // }
}
