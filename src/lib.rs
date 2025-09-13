use std::io::Cursor;

use image::{ImageError, ImageFormat};
use js_sys::Reflect;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

fn map_image_err(err: ImageError) -> String {
    format!("Image processing error: {}", err)
}

fn convert(image_data: Vec<u8>, output: ImageFormat) -> Result<Vec<u8>, String> {
	report_progress("Loading image...");
    let img = image::load_from_memory(&image_data).map_err(map_image_err)?;
    let mut output_data: Vec<u8> = Vec::new();
	report_progress("Converting to new format...");
    img.write_to(&mut Cursor::new(&mut output_data), output).map_err(map_image_err)?;
	report_progress("Completed conversion.");
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