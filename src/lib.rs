use wasm_bindgen::prelude::*;
use zip::write::FileOptions;
use image::io::Reader as ImageReader;
use std::io::Cursor;
use std::collections::HashMap;
use std::io::Write;
mod utils;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(str: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub enum FileFormat {
    Gif,
    Jpeg,
    Ico,
    Png,
    Tga,
    Bmp,
    Farbfeld
}

#[wasm_bindgen]
pub fn convert_image(image_bytes: Vec<u8>, into_file: FileFormat) -> Vec<u8> {
    utils::set_panic_hook();

    let mut converted_bytes: Vec<u8> = Vec::new();

    let img = match ImageReader::new(Cursor::new(image_bytes)).with_guessed_format() {
        Ok(i) => i,
        Err(_) => {
            converted_bytes.push(0);
            return converted_bytes;
        }
    };
    let img = match img.decode() {
        Ok(i) => i,
        Err(_) => {
            converted_bytes.push(1);
            return converted_bytes;
        }
    };
    
    
    let format = match into_file {
        FileFormat::Gif => image::ImageOutputFormat::Gif,
        FileFormat::Jpeg => image::ImageOutputFormat::Jpeg(100),
        FileFormat::Ico => image::ImageOutputFormat::Ico,
        FileFormat::Png => image::ImageOutputFormat::Png,
        FileFormat::Tga => image::ImageOutputFormat::Tga,
        FileFormat::Bmp => image::ImageOutputFormat::Bmp,
        FileFormat::Farbfeld => image::ImageOutputFormat::Farbfeld
    };
    match img.write_to(&mut converted_bytes, format) {
        Ok(_i) => return converted_bytes,
        Err(_) => {
            converted_bytes.clear();
            converted_bytes.push(2);
            return converted_bytes;
        }
    }
}

#[wasm_bindgen]
pub fn create_zip(files: JsValue) -> Vec<u8> {
    let files: HashMap<String, Vec<u8>> = serde_wasm_bindgen::from_value(files).expect("Couldn't parse values ");
    let mut buffer: Vec<u8> = Vec::new();

    let mut zip = zip::ZipWriter::new(Cursor::new(&mut buffer));
    for (key, file) in files.iter() {
        zip.start_file(key, FileOptions::default()).expect("Failed at start_file");
        zip.write_all(file).expect("Failed to write File to zip");
    }
    let final_zip = zip.finish().expect("Failed to finish zip");
    final_zip.into_inner().to_vec()
}