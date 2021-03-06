use wasm_bindgen::prelude::*;
use zip::write::FileOptions;
use image::io::Reader as ImageReader;
use std::io::{self, Write, Cursor};
mod utils;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(str: &str);
}

/*macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}*/

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
pub fn convert_image(image_bytes: Vec<u8>, into_file: FileFormat) -> Result<Vec<u8>, JsValue> {
    utils::set_panic_hook();

    let mut converted_bytes: Vec<u8> = Vec::new();

    let img = match ImageReader::new(Cursor::new(image_bytes)).with_guessed_format() {
        Ok(i) => i,
        Err(e) => {
            return Err(JsValue::from(
                format!("Couldn't load image. {}", e)
            ));
        }
    };
    let img = match img.decode() {
        Ok(i) => i,
        Err(e) => {
            return Err(JsValue::from(
                format!("Couldn't decode image. {}", e)
            ));
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
        Ok(_i) => return Ok(converted_bytes),
        Err(e) => {
            return Err(JsValue::from(
                format!("Couldn't encode image. {}", e)
            ));
        }
    }
}

#[wasm_bindgen]
pub fn create_zip(files: Vec<u8>) -> Vec<u8> {
    utils::set_panic_hook();
    //console_log!("Starting Zipping");
    //console_log!("Parsed data");
    let mut buffer: Vec<u8> = Vec::new();
    //let mut file_cursor = Cursor::new(&files);

    let mut zip = zip::ZipWriter::new(Cursor::new(&mut buffer));
    //console_log!("Created zip");
    for (key, file) in files.iter().enumerate() {
        if *file == 0xFF {
            if files[key+1] == 0x54 && files[key+2] == 0x42 && files[key+3] == 0x42 && files[key+4] == 0x4E {
                read_data(&files[key+5..], &mut zip);
            }
        }
    }
    
    let final_zip = zip.finish().expect("Failed to finish zip");
    final_zip.into_inner().to_vec()
}

fn read_data<W: Write + io::Seek>(file: &[u8], zip: &mut zip::ZipWriter<W>) {
    let mut name_buffer: Vec<u8> = Vec::new();
    let mut file_buffer: Vec<u8> = Vec::new();
    'outer: for (key_a, n_a) in file.iter().enumerate() {
        if *n_a == 0xFF {
            if file[key_a+1] == 0x54 && file[key_a+2] == 0x42 && file[key_a+3] == 0x42 && file[key_a+4] == 0x46 {
                let file_slice = &file[key_a+5..];
                for (key_b, n_b) in file_slice.iter().enumerate() {
                    if *n_b == 0xFF {
                        if file_slice[key_b+1] == 0x54 && file_slice[key_b+2] == 0x42 && file_slice[key_b+3] == 0x45 && file_slice[key_b+4] == 0x46 {
                            break 'outer;
                        }
                    }
                    file_buffer.push(*n_b);
                }
                break;
            }
        } else {
            name_buffer.push(*n_a);
        }
    }
    let name = String::from_utf8(name_buffer).unwrap();
    let file_base64 = String::from_utf8(file_buffer).unwrap();
    let bytes = base64_decode(&file_base64);

    zip.start_file(name, FileOptions::default()).expect("Failed at start_file");
    zip.write_all(&bytes).expect("Failed to write File to zip");
}

fn base64_decode(file_base64: &String) -> Vec<u8> {
    let data = base64::decode(&file_base64).unwrap();
    let bytes_str = std::str::from_utf8(&data).unwrap();
    let iter = bytes_str.split(',');
    let mut result: Vec<u8> = Vec::new();
    for n in iter {
        result.push(n.parse().unwrap());
    }

    result
}