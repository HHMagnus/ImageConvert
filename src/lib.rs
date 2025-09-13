use std::io::Cursor;

use image::{ImageError, ImageFormat};
use js_sys::Reflect;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

fn map_image_err(err: ImageError) -> String {
    format!("Image processing error: {}", err)
}

fn convert(image_data: Vec<u8>, input: ImageFormat, output: ImageFormat) -> Result<Vec<u8>, String> {
	report_progress("Loading image...");
    let img = image::load_from_memory_with_format(&image_data, input).map_err(map_image_err)?;
    let mut output_data: Vec<u8> = Vec::new();
	report_progress("Converting to new format...");
    img.write_to(&mut Cursor::new(&mut output_data), output).map_err(map_image_err)?;
	report_progress("Completed conversion.");
    Ok(output_data)
}

fn str_to_type(s: &str) -> Option<ImageFormat> {
	match s.to_lowercase().as_str() {
		"image/avif" => Some(ImageFormat::Avif),
		"image/bmp" | "image/x-bmp" => Some(ImageFormat::Bmp),
		"image/vnd-ms.dds" | "image/x-direct-draw-surface" => Some(ImageFormat::Dds),
		"image/x-exr" => Some(ImageFormat::OpenExr),
		"image/ff" => Some(ImageFormat::Farbfeld),
		"image/gif" => Some(ImageFormat::Gif),
		"image/vnd.radiance" => Some(ImageFormat::Hdr),
		"image/x-icon" => Some(ImageFormat::Ico),
		"image/jpeg" => Some(ImageFormat::Jpeg),
		"image/png" => Some(ImageFormat::Png),
		"image/x-portable-bitmap" | "image/x-portable-graymap" | "image/x-portable-pixmap" | "image/x-portable-anymap" => Some(ImageFormat::Pnm),
		"image/qoi" => Some(ImageFormat::Qoi),
		"image/x-tga" | "image/x-targa" => Some(ImageFormat::Tga),
		"image/tiff" | "image/tiff-fx" => Some(ImageFormat::Tiff),
		"image/webp" => Some(ImageFormat::WebP),
		_ => None,
	}
}

#[wasm_bindgen]
pub fn convert_exposed(image_data: Vec<u8>, input: String, output: String) -> Result<Vec<u8>, String> {
    let input = str_to_type(&input).ok_or_else(|| format!("Unsupported input format: {}", input))?;
    let output = str_to_type(&output).ok_or_else(|| format!("Unsupported output format: {}", output))?;
    convert(image_data, input, output)
}

fn report_progress(message: &str) {
    let global = js_sys::global();

    let func = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
        .unwrap()
        .dyn_into::<js_sys::Function>()
        .unwrap();

    let obj = js_sys::Object::new();
    Reflect::set(&obj, &JsValue::from_str("type"), &JsValue::from_str("progress")).unwrap();
    Reflect::set(&obj, &JsValue::from_str("message"), &JsValue::from_str(message)).unwrap();

    func.call1(&global, &obj).unwrap();
}