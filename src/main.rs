#![allow(non_snake_case)]
use std::ops::Deref;
use log::info;
//use log::info;
use web_sys::*;
use yew::prelude::*;

mod helper_funcs;
mod default_styles;
use default_styles::defaults::set_if_empty;

#[derive(Clone)]
pub struct Image {
    name: String,
    url: String,
}

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
    let file_input_node_ref = use_node_ref();
    let active = use_state(|| false);
    let images = use_state(|| vec![Image { name: "".to_string(), url: "".to_string()}]);

    let read = {
        let images = images.clone();
        move || {
            info!("read");
            helper_funcs::set_img_sources(images.clone(), true);
        }
    };

    let store = {
        let read = read.clone();
        let file_input_node_ref = file_input_node_ref.clone();

        move |_| {
            if let Some(input) = file_input_node_ref.cast::<HtmlInputElement>() {
                let files = input.files().unwrap();
                for i in 0..files.length() { 
                    let key = format!("{}", &files.item(i).unwrap().name() );
                    helper_funcs::store_file_blob(key, files.item(i).unwrap(), read.clone());
                }
            }
        }
    };

    let delete = {
        let read = read.clone();

        move |e: MouseEvent| {
            let img = e.target_dyn_into::<Element>().unwrap().next_element_sibling().unwrap();
            let name = img.get_attribute("alt").unwrap();
            helper_funcs::delete_file_blob(&name, read.clone());
        }
    };

    let open = {
        let active = active.clone();
        let read = read.clone();
        move |_| {
            read();
            active.set(true);
        }
    };

    let close = {
        let active = active.clone();
        move |_| {
            active.set(false);
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
            <label for="search_field">
                <span style="visibility:hidden;width:0px;height:0px;position:absolute;">{"file search"}</span>
                <input id="search_field" onclick={open} name="search field" for="images" style={ input_style }/>
            </label>
            if *active {
                <div style="display: grid; grid-template-columns: 3.8fr 0.2fr; grid-template-rows: 1fr; justify-items: end; align-items: normal; width: 400%;">
                    <div style={ upload_style }>
                        <svg alt="upload icon" style="position: absolute; margin: auto; top: 0; left: 0; right: 0; bottom: 0;" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                            <path alt="close icon" d="M6 20h12M12 16V4m0 0l3.5 3.5M12 4L8.5 7.5" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                        </svg>
                        <input tabindex="0" style="opacity: 0; width: 100%;height:100%;" ref={file_input_node_ref} onchange={store} type="file" accept="image/png, image/jpeg, image/webp" multiple=true/>
                    </div>
                    <svg alt="close icon" style="position: relative; right: -33%" onclick={close} width="100%" height="100%" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path alt="close icon" d="M3 17V7a2 2 0 012-2h14a2 2 0 012 2v10a2 2 0 01-2 2H5a2 2 0 01-2-2z" stroke="#000000" stroke-width="1.5"></path>
                        <path alt="close icon" d="M10 14.243l2.121-2.122m0 0L14.243 10m-2.122 2.121L10 10m2.121 2.121l2.122 2.122M6 8h1" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                </div>
                <div id="images" style={ selection_box_style }>
                    { images.deref().iter().map(|i| html_nested!(
                        if !i.url.is_empty() { 
                            <picture style="display: grid; grid-template-columns: 5fr 1fr; align-items: center; justify-items: end; grid-template-areas: \". i\" \"a a\"">
                                <p style="margin: 0px; font-family: Arial, Helvetica, sans-serif; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 100%;">{ i.clone().name }</p>
                                <svg onclick={delete.clone()} alt="delete icon" style="" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                                    <path d="M9.879 14.121L12 12m2.121-2.121L12 12m0 0L9.879 9.879M12 12l2.121 2.121M21 3.6v16.8a.6.6 0 01-.6.6H3.6a.6.6 0 01-.6-.6V3.6a.6.6 0 01.6-.6h16.8a.6.6 0 01.6.6z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                                </svg>
                                <img src={ i.clone().url } alt={ i.clone().name } style={ format!("grid-area: a; {}", image_style.clone()) }/> 
                            </picture>
                        }
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