use super::*;

#[derive(Serialize, Default)]
struct Meta {
    file_name: Option<String>,
    timestamp: Option<u32>,
    file_size: u64,
}
pub async fn check(
    state: Extension<SharedState>,
    Path(file_id): Path<String>,
) -> impl IntoResponse {
    info!("file_id: {file_id}");
    let mut orig_file_path = {
        let mut ret = std::path::PathBuf::from("video");
        ret.push(file_id);
        ret.push("orig");
        ret
    };
    if let Ok(metadata) = orig_file_path.metadata() {
        let mut ret = Meta::default();
        ret.file_size = metadata.len();
        let meta_file_path = {
            let mut ret = orig_file_path;
            ret.set_file_name("meta.yaml");
            ret
        };

        if let Some((file_name, timestamp)) = tokio::fs::read_to_string(meta_file_path)
            .await
            .ok()
            .and_then(|s| {
                #[derive(Deserialize)]
                struct Meta {
                    file_name: String,
                    timestamp: u32,
                }
                serde_yml::from_str::<Meta>(&s)
                    .map(
                        |Meta {
                             file_name,
                             timestamp,
                         }| (file_name, timestamp),
                    )
                    .ok()
            })
        {
            ret.file_name = Some(file_name);
            ret.timestamp = Some(timestamp);
        }
        Json(Some(ret))
    } else {
        Json(None)
    }
}
