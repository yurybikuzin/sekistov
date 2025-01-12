use super::*;

pub async fn upload(_state: Extension<SharedState>, data: Bytes) -> impl IntoResponse {
    info!("len: {}", data.len());
    let mut i = data.len();
    loop {
        i -= 1;
        if data[i] == 0 {
            break;
        }
    }
    info!("i: {i}, data.len: {}", data.len());
    info!("{}", std::str::from_utf8(&data[i + 1..data.len()]).unwrap());
    let timestamp = u32::from_le_bytes(data[i - 4..i].try_into().unwrap());
    info!("timestamp: {timestamp}");

    use blake2::{digest::consts::U16, Blake2s, Digest};
    // use hex_literal::hex;

    type Blake2s128 = Blake2s<U16>;

    let mut hasher = Blake2s128::new();
    hasher.update(&data[0..i - 4]);
    let res = hasher.finalize();
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
    let s = URL_SAFE_NO_PAD.encode(res);
    info!("file_id: {s}");
    // info!(res);
    // assert_eq!(res[..], hex!("2cc55c84e416924e6400")[..]);

    // info!(
    //     "{}",
    //     std::str::from_utf8(&data[data.len() - 14..data.len()]).unwrap()
    // );
    // info!(
    //     "{}",
    //     std::str::from_utf8(&data[data.len() - 14..data.len()]).unwrap()
    // );

    use std::fs;
    use std::io::Write; // bring trait into scope

    // ... later in code
    // let file_path = "video/some";
    // let mut file = fs::OpenOptions::new()
    //     .create(true) // To create a new file
    //     .write(true)
    //     // either use the ? operator or unwrap since it returns a Result
    //     .open(file_path)
    //     .unwrap();
    //
    // let _ = file.write_all(&data);
    // file_path
    "OK"
}
