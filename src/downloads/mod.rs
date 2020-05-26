use crate::utils::errors;
use actix_files::{Files, NamedFile};
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{
    web::{get, scope, Path as ReqPath},
    Error, Scope,
};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct Info {
    file_name: String,
}

async fn download_file(info: ReqPath<Info>) -> Result<NamedFile, Error> {
    let path = Path::new(&info.file_name);
    if path.exists() {
        let file = NamedFile::open(&info.file_name)?;
        Ok(file.set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
    } else {
        Err(errors::CustomError::NoFileFound(format!("unable to find {}", &info.file_name)).into())
    }
}

pub fn register_download_routes() -> Scope {
    scope("/download")
        .route("/file/{file_name}", get().to(download_file))
        .service(Files::new("/directory", "http").show_files_listing())
}
