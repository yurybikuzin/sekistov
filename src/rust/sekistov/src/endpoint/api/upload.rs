use super::*;

#[utoipa::path(
    post,
    path = "/upload",
    tag = TODO_TAG,
    request_body(content_type = "application/octet-stream"),
    responses(
        (status = 200, description = "Did upload successfully", body = UploadResponse)
    )
)]
#[axum::debug_handler]
pub async fn handler(_state: Extension<SharedState>, data: Bytes) -> impl IntoResponse {
    #[derive(Serialize)]
    struct Meta<'a> {
        file_name: &'a str,
        timestamp: u32,
    }
    let (file_id, meta) = {
        let mut i = data.len();
        loop {
            i -= 1;
            if data[i] == 0 {
                break;
            }
        }
        let file_name = std::str::from_utf8(&data[i + 1..data.len()]).unwrap();
        let timestamp = u32::from_le_bytes(data[i - 4..i].try_into().unwrap());

        let file_id = {
            use blake2::{digest::consts::U16, Blake2s, Digest};

            type Blake2s128 = Blake2s<U16>;

            let mut hasher = Blake2s128::new();
            hasher.update(&data[0..i - 4]);
            let res = hasher.finalize();
            use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
            URL_SAFE_NO_PAD.encode(res)
        };
        (
            file_id,
            Meta {
                file_name,
                timestamp,
            },
        )
    };

    use tokio::io::AsyncWriteExt;

    let file_path = {
        let mut ret = std::path::PathBuf::from("video");
        ret.push(file_id);
        ret
    };
    let from_file_path = {
        let file_path = {
            let mut ret = file_path.clone();
            ret.push("orig~");
            ret
        };
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|err| anyhow!("create_dir_all({parent:?}): {err}"))
                .unwrap();
        }
        file_path
    };

    will_did!(info => format!("write({from_file_path:?})"),
        tokio::fs::write(&from_file_path, &data)
            .await
            .map_err(|err| anyhow!("write({from_file_path:?}): {err}")).unwrap()
    );

    let to_file_path = {
        let mut ret = file_path;
        ret.push("orig");
        ret
    };

    let meta_file_path = {
        let mut ret = to_file_path.clone();
        ret.set_file_name("meta.yaml");
        ret
    };

    will_did!(info => format!("write({meta_file_path:?})"),
        tokio::fs::write(&meta_file_path, serde_yml::to_string(&meta).unwrap().as_bytes())
            .await
            .map_err(|err| anyhow!("write({from_file_path:?}): {err}")).unwrap()
    );

    will_did!(info => format!("rename({from_file_path:?}, {to_file_path:?})"),
        tokio::fs::rename(&from_file_path, &to_file_path)
            .await
            .map_err(|err| anyhow!("rename({from_file_path:?}, {to_file_path:?})"))
            .unwrap()
    );

    UploadResponse("OK").0.into_response()
}

// #[derive(ToSchema)]
// struct UploadResponse(#[schema(examples = ("OK"))] &'static str);

#[derive(ToSchema)]
#[schema(value_type = String)]
#[schema(example = "OK")]
struct UploadResponse(&'static str);

// use utoipa::{
//     openapi::{KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat, Type},
//     PartialSchema,
// };
//
// impl PartialSchema for UploadResponse {
//     fn schema() -> RefOr<Schema> {
//         // ... impl schema generation here
//         RefOr::T(Schema::Object(
//             ObjectBuilder::new()
//                 .schema_type(Type::String)
//                 .examples(["OK"])
//                 // .format(Some(SchemaFormat::KnownFormat(KnownFormat::String)))
//                 .build(),
//         ))
//     }
// }
