importScripts("./pkg/toolbelt.js");
let wasm_converter = wasm_bindgen("./pkg/toolbelt_bg.wasm");

onmessage = function(e) {
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
                    //let image_blob = new Blob([converted_image_bytes.buffer], { type: 'image/'+e.data.into_format.toLowerCase() });
                    //Using an array buffer to 'move' the result and not clone it
                    let new_type = 'image/'+e.data.into_format.toLowerCase();
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
};