use web_sys::{FileReader, window};
use wasm_bindgen::{JsCast, prelude::*};
use yew::UseStateHandle;

fn error_setting_local_storage(_e: JsValue) {
    let error = 
"Looks like your browsers local storage for this website is full.
Having many big images on a website can slow down page load speed drastically. 
However if you insist on having a lot of images here's what you can do about it:
                - remove some content you might not need
                - scale down your images aspect ratio
                - convert some of your images to a smaller image format
                - host your images externally";
    window().unwrap().alert_with_message(error).expect("Error: Could not send an alert message");
}

pub fn store_data_uri_string<F>(key: String, file: &web_sys::File, images: UseStateHandle<Vec<String>>, read: F) 
    where F: Fn() -> Vec<String> + 'static 
{
    let reader = FileReader::new().expect("Error: Could not create FileReader");
    reader.read_as_data_url(file).expect("Error: Could not read file as data URI");
    let reader_c = reader.clone();
    let closure = Closure::wrap(Box::new(move || {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let data_uri_string = reader.result().expect("Error: Could not get result from FileReader");
        local_storage.set_item(key.as_str(), &data_uri_string.as_string().unwrap()).unwrap_or_else(error_setting_local_storage);
        images.set(read());
    }) as Box<dyn FnMut() >);
    reader_c.set_onloadend(closure.as_ref().dyn_ref());
    closure.forget();
}