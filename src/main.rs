#![allow(non_snake_case)]
use std::ops::Deref;
//use log::info;
use web_sys::*;
use yew::prelude::*;

mod helper_funcs;
mod default_styles;
use default_styles::defaults::set_if_empty;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    #[prop_or_default]
    pub input_style: String,
    #[prop_or_default]
    pub upload_style: String,
    #[prop_or_default]
    pub selection_box_style: String,
    #[prop_or_default]
    pub image_style: String,
}

#[function_component]
fn App(props: &Props) -> Html {
    let active = use_state(|| false);

    let open = {
        let active = active.clone();
        move |_| {
            active.set(true);
        }
    };

    let close = {
        let active = active.clone();
        move |_| {
            active.set(false);
        }
    };

    let read = || {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let storage_length = local_storage.length().unwrap();
        let mut items = Vec::new();
        for i in 0..storage_length { 
            // getting the stored blob url
            let key = format!("jcm_{}", i);
            let blob_uri = local_storage.get_item(&key).expect("couldn't get local storage");
            if let Some(blob_uri) = blob_uri {
                items.push(blob_uri);
            }
        }
        items
    };

    let file_input_node_ref = use_node_ref();
    let images = use_state(read);

    let store = {
        let images = images.clone();
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let storage_length = local_storage.length().unwrap();
        let file_input_node_ref = file_input_node_ref.clone();

        move |_| {
            if let Some(input) = file_input_node_ref.cast::<HtmlInputElement>() {
                let files = input.files().unwrap();
                for i in 0..files.length() { 
                    let key = format!("jcm_{}", &storage_length + i);
                    helper_funcs::store_data_uri_string(key, &files.item(i).unwrap(), images.clone(), read.clone());
                }
            }
        }
    };

    // Default styles
    let mut input_style = props.input_style.clone();
    let mut selection_box_style = props.selection_box_style.clone();
    let mut image_style = props.image_style.clone();
    let mut upload_style = props.image_style.clone();
    set_if_empty(&mut input_style, &mut selection_box_style, &mut image_style, &mut upload_style);

    html! {
        <div style="width: 200px;">
            <input onclick={open} style={ input_style }/>
            if *active {
                <div style="display: grid; grid-template-columns: 3.8fr 0.2fr; grid-template-rows: 1fr; justify-items: end; align-items: baseline; width: 400%;">
                    <div style={ upload_style }>
                        <svg style="position: absolute; margin: auto; top: 0; left: 0; right: 0; bottom: 0;" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                            <path d="M6 20h12M12 16V4m0 0l3.5 3.5M12 4L8.5 7.5" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                        </svg>
                        <input style="opacity: 0; width: 100%;height:100%;" ref={file_input_node_ref} onchange={store} type="file" accept="image/png, image/jpeg, image/webp" multiple=true/>
                    </div>
                    <svg style="position: relative; right: -33%" onclick={close} width="100%" height="100%" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M3 17V7a2 2 0 012-2h14a2 2 0 012 2v10a2 2 0 01-2 2H5a2 2 0 01-2-2z" stroke="#000000" stroke-width="1.5"></path>
                        <path d="M10 14.243l2.121-2.122m0 0L14.243 10m-2.122 2.121L10 10m2.121 2.121l2.122 2.122M6 8h1" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                </div>
                <div style={ selection_box_style }>
                    { images.deref().iter().map(|i| html_nested!(
                        <img src={ i.clone() } style={ image_style.clone() }/>
                    )).collect::<Html>()}
                </div>
            }
        </div>
    }
}

fn main() {
    //* Debuggig
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}