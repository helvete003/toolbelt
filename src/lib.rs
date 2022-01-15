use wasm_bindgen::prelude::*;
use image::io::Reader as ImageReader;
use std::io::Cursor;
mod utils;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(str: &str);
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