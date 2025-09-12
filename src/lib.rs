use std::io::Cursor;

use image::{ImageError, ImageFormat};

fn map_image_err(err: ImageError) -> String {
    format!("Image processing error: {}", err)
}

fn convert(image_data: Vec<u8>, output: ImageFormat) -> Result<Vec<u8>, String> {
    let img = image::load_from_memory(&image_data).map_err(map_image_err)?;
    let mut output_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut output_data), output).map_err(map_image_err)?;
    Ok(output_data)
}