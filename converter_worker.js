importScripts("./pkg/toolbelt.js");
let wasm_converter = wasm_bindgen("./pkg/toolbelt_bg.wasm");

let zip = undefined;
let zip_buffer = new Uint8Array();

Uint8Array.prototype.appendBytes = function(name, file_bytes) {
    let text_bytes = new TextEncoder();
    let file_base64_bytes = new TextEncoder();
    text_bytes = text_bytes.encode(name);
    file_bytes = file_base64_bytes.encode(file_bytes);

    let tmp_data = new Uint8Array(this.length + 20 + text_bytes.length + file_bytes.length);
    //since Uint8Array is a zero'ed slice of memory we need to offet the data ourself
    let offset = 0;
    tmp_data.set(this, offset);
    offset += this.length;
    tmp_data.set([0xFF, 0x54, 0x42, 0x42, 0x4E], offset); // FF T B B N (Toolbelt begin name)
    offset += 5;
    tmp_data.set(text_bytes, offset);
    offset += text_bytes.length;
    tmp_data.set([0xFF, 0x54, 0x42, 0x42, 0x46], offset); // FF T B B F (Toolbelt begin file)
    offset += 5;
    tmp_data.set(file_bytes, offset);
    offset += file_bytes.length;
    tmp_data.set([0xFF, 0x54, 0x42, 0x45, 0x46], offset); // FF T B E F (Toolbelt end file)
    return tmp_data;
};

onmessage = function(e) {
    if(e.data.status == "convert_images") {
        zip = new Object();
        zip_buffer = new Uint8Array();
        let fileList = e.data.file_elements;
        for (let i = 0; i < fileList.length; i++) {
            fileList[i].arrayBuffer()
                .then((buffer) => {
                    return wasm_converter.then(() => {
                        let into_format = wasm_bindgen.FileFormat[e.data.into_format];
                        return wasm_bindgen.convert_image(new Uint8Array(buffer), into_format);
                    });
                })
                .then((converted_image_bytes) => {
                    if(converted_image_bytes.length > 1) {
                        let new_name = function() {
                            let x = fileList[i].name.split(".");
                            x[x.length - 1 ] = e.data.into_format.toLowerCase();
                            return x.join(".");
                        }();
                        //Using an array buffer to 'move' the result and not clone it
                        let new_type = 'image/'+e.data.into_format.toLowerCase();
                        //create a copy of the byte array for zipping
                        //zip[new_name] = converted_image_bytes.slice(0);
                        zip_buffer = zip_buffer.appendBytes(new_name, btoa(converted_image_bytes.slice(0)));
                        postMessage({status: "image_done", converted_image_bytes, new_type, new_name}, [converted_image_bytes.buffer]);
                    } else {
                        let error_message = "";
                        let status = converted_image_bytes[0];
                        if(status == 0) {
                            error_message = "Couldn't load image."
                        } else if(status == 1) {
                            error_message = "Couldn't decode image."
                        } else {
                            error_message = "Couldn't encode image."
                        }
                        postMessage({status: "image_error", error_message, name: fileList[i].name});
                    }
                });
        }
    } else if(e.data.status == "generate_zip") {
        wasm_converter.then(() => {
            let zip_data = wasm_bindgen.create_zip(zip_buffer);
            postMessage({status: "finished_zip", zip_bytes: zip_data}, [zip_data.buffer]);
            zip = new Object();
            zip_buffer = new Uint8Array();
        });
    }
};

