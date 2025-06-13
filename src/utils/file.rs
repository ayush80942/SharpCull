use axum::extract::Multipart;
use image::{DynamicImage, ImageFormat};
use tokio::io::AsyncReadExt;

pub async fn multipart_to_image_buffer(multipart: &mut Multipart) -> Option<DynamicImage> {
    while let Some(field) = multipart.next_field().await.ok()? {
        let content_type = field.content_type()?.to_string();
        if content_type.starts_with("image/") {
            let mut data = vec![];
            let mut field_data = field;
            field_data.read_to_end(&mut data).await.ok()?;
            let img = image::load_from_memory(&data).ok()?;
            return Some(img);
        }
    }
    None
}
