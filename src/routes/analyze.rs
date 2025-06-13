use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::Serialize;
use crate::services::image_analysis::{analyze_blur, analyze_brightness};
use crate::utils::file::multipart_to_image_buffer;

#[derive(Serialize)]
struct AnalysisResult {
    blur_score: f32,
    brightness: f32,
}

pub async fn analyze_image(mut multipart: Multipart) -> impl IntoResponse {
    let Some(image_buffer) = multipart_to_image_buffer(&mut multipart).await else {
        return (StatusCode::BAD_REQUEST, "Invalid image upload").into_response();
    };

    let blur_score = analyze_blur(&image_buffer);
    let brightness = analyze_brightness(&image_buffer);

    let result = AnalysisResult {
        blur_score,
        brightness,
    };

    (StatusCode::OK, Json(result)).into_response()
}
