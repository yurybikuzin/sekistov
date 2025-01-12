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
