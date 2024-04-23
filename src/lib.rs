mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::fs::File;
use std::io::prelude::*;
use zstd::stream::read::Decoder;
use web_sys::HtmlInputElement;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Create a file input element
    let input: HtmlInputElement = document.create_element("input")?.dyn_into()?;
    input.set_type("file");

    // Listen for the change event
    let on_change = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let files = target.dyn_into::<HtmlInputElement>().unwrap().files().unwrap();
        let file = files.get(0).unwrap();
        alert(&file.name());
    }) as Box<dyn FnMut(_)>);

    input.add_event_listener_with_callback("change", on_change.as_ref().unchecked_ref())?;
    on_change.forget();

    // Append the file input element to the body
    body.append_child(&input)?;

    Ok(())
}

/*
pub fn init() {
    // Open the ZSTD file
    let f = File::open("archive.zst")?;

    // Create a ZSTD decoder
    let mut decoder = Decoder::new(f)?;
    
    // Read the decompressed bytes into a Vec<u8>
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    
    // At this point, `buffer` contains the decompressed bytes.
    // If the ZSTD file was an archive (e.g., tar), you can use a crate like `tar` to read the files:
    let mut archive = tar::Archive::new(&*buffer);
    
    // Iterate over the files in the archive
    for file in archive.entries()? {
        let mut file = file?;
        println!("{}", file.path()?.display());
    }
    
    Ok(())
}
*/
