use actix_multipart::Multipart;
use actix_web::{
    web::{post, scope},
    Error, HttpResponse, Scope,
};
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};


// reference from https://github.com/actix/examples/tree/master/multipart-async-std
async fn upload_file(mut body: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = body.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

pub fn register_upload_handlers() -> Scope {
    scope("/upload").route("", post().to(upload_file))
}
