use std::io::Write;

use actix_multipart::Multipart;
use actix_web::web;
use futures_util::TryStreamExt as _;
use uuid::Uuid;

pub async fn save_file(mut payload: Multipart, parent: String) -> anyhow::Result<Vec<String>> {
    let mut files: Vec<String> = vec![];

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        std::fs::create_dir_all(&format!("./tmp/{}", &parent));

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("./tmp/{}/{}", &parent, filename);
        files.push(filepath.clone());
        let mut f = web::block(move || std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(files)
}
