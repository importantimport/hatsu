use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, Redirect},
};
use hatsu_utils::AppError;

#[debug_handler]
pub async fn redirect(Path(base64_url): Path<String>) -> impl IntoResponse {
    let base64 = base64_simd::URL_SAFE;

    match base64.decode_to_vec(&base64_url) {
        Ok(utf8_url) => match String::from_utf8(utf8_url) {
            Ok(url) if url.starts_with("https://") => {
                Ok(Redirect::permanent(&format!("/o/{}", url)).into_response())
            }
            _ => Err(AppError::not_found("Record", &base64_url)),
        },
        _ => Err(AppError::not_found("Record", &base64_url)),
    }
    // Redirect::permanent(&format!("/o/{}", object)).into_response()
}
