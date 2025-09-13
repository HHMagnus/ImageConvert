use std::io::Cursor;

use image::{ImageError, ImageFormat};
use wasm_bindgen::prelude::wasm_bindgen;

fn map_image_err(err: ImageError) -> String {
    format!("Image processing error: {}", err)
}

fn convert(image_data: Vec<u8>, output: ImageFormat) -> Result<Vec<u8>, String> {
    let img = image::load_from_memory(&image_data).map_err(map_image_err)?;
    let mut output_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut output_data), output).map_err(map_image_err)?;
    Ok(output_data)
}

#[wasm_bindgen]
pub fn convert_exposed(image_data: Vec<u8>, output: String) -> Result<Vec<u8>, String> {
    let output = match output.to_lowercase().as_str() {
        "png" => ImageFormat::Png,
        "jpeg" | "jpg" => ImageFormat::Jpeg,
        "gif" => ImageFormat::Gif,
        "bmp" => ImageFormat::Bmp,
        "ico" => ImageFormat::Ico,
        "tiff" => ImageFormat::Tiff,
        "webp" => ImageFormat::WebP,
        _ => return Err("Unsupported output format".to_string()),
    } ;
    convert(image_data, output)
}