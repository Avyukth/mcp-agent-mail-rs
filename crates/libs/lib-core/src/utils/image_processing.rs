use crate::Result;
use base64::{Engine as _, engine::general_purpose};
use image::{GenericImageView, ImageFormat};

#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    #[error("Invalid image data: {0}")]
    InvalidData(String),
    #[error("Unsupported image format")]
    UnsupportedFormat,
    #[error("Image too small: {width}x{height}")]
    TooSmall { width: u32, height: u32 },
    #[error("Image too large: {width}x{height}")]
    TooLarge { width: u32, height: u32 },
    #[error("Invalid data URI")]
    InvalidDataUri,
}

/// Process and validate image data.
/// Returns the image format and dimensions if valid.
pub fn validate_image(data: &[u8]) -> Result<(ImageFormat, u32, u32)> {
    // 1. Guess format
    let format = image::guess_format(data).map_err(|e| ImageError::InvalidData(e.to_string()))?;

    // 2. Load image to verify integrity and get dimensions
    // limit size to prevent bombs? image crate has some limits but we should be careful.
    let img = image::load_from_memory_with_format(data, format)
        .map_err(|e| ImageError::InvalidData(e.to_string()))?;

    let (width, height) = img.dimensions();

    // 3. Check dimensions (Edge case: 1x1 might be tracking pixel, but let's allow it for now
    // unless strictly forbidden. The test will verify behavior.
    // If input is malformed, load_from_memory would have failed.

    // Check max dimensions (e.g. 8k)
    if width > 7680 || height > 4320 {
        return Err(ImageError::TooLarge { width, height }.into());
    }

    Ok((format, width, height))
}

/// Decode data URI to bytes
pub fn decode_data_uri(uri: &str) -> Result<(Vec<u8>, String)> {
    if !uri.starts_with("data:") {
        return Err(ImageError::InvalidDataUri.into());
    }

    let parts: Vec<&str> = uri.splitn(2, ',').collect();
    if parts.len() != 2 {
        return Err(ImageError::InvalidDataUri.into());
    }

    let metadata = parts[0];
    let data_part = parts[1];

    // Extract media type e.g. "image/png"
    // metadata is like "data:image/png;base64"
    let media_type = metadata
        .strip_prefix("data:")
        .and_then(|s| s.split(';').next())
        .unwrap_or("");

    if !metadata.contains(";base64") {
        // We only support base64 for now
        return Err(ImageError::InvalidDataUri.into());
    }

    let data = general_purpose::STANDARD
        .decode(data_part)
        .map_err(|_| ImageError::InvalidDataUri)?;

    Ok((data, media_type.to_string()))
}

/// Check if image is a "malformed" or special edge case we want to detect?
/// For now relying on image crate's load validation.
pub fn is_valid_image(data: &[u8]) -> bool {
    validate_image(data).is_ok()
}
