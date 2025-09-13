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
		"avif" => Some(ImageFormat::Avif),
		"bmp" => Some(ImageFormat::Bmp),
		"dds" => Some(ImageFormat::Dds),
		"exr" => Some(ImageFormat::OpenExr),
		"ff" => Some(ImageFormat::Farbfeld),
		"gif" => Some(ImageFormat::Gif),
		"hdr" => Some(ImageFormat::Hdr),
		"ico" => Some(ImageFormat::Ico),
		"jpeg" => Some(ImageFormat::Jpeg),
		"png" => Some(ImageFormat::Png),
		"pnm" => Some(ImageFormat::Pnm),
		"qoi" => Some(ImageFormat::Qoi),
		"tga" => Some(ImageFormat::Tga),
		"tiff" => Some(ImageFormat::Tiff),
		"webp" => Some(ImageFormat::WebP),
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