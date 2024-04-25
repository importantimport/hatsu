use axum::{debug_handler, extract::Path, response::Redirect};
use hatsu_utils::AppError;

#[debug_handler]
pub async fn notice(Path(base64_url): Path<String>) -> Result<Redirect, AppError> {
    let base64 = base64_simd::URL_SAFE;

    base64.decode_to_vec(&base64_url).map_or_else(
        |_| Err(AppError::not_found("Record", &base64_url)),
        |utf8_url| match String::from_utf8(utf8_url) {
            Ok(url) if url.starts_with("https://") =>
                Ok(Redirect::permanent(&format!("/posts/{url}"))),
            _ => Err(AppError::not_found("Record", &base64_url)),
        },
    )
}
