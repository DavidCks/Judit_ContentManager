use log::info;
use web_sys::*;
use rusty_css::Style;
use yew::prelude::*;

mod default_styles;
use default_styles::input::InputStyle;
use default_styles::selection_box::SelectionBoxStyle;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    #[prop_or(InputStyle::create().inline().into())]
    pub input_style: AttrValue,
    #[prop_or(SelectionBoxStyle::create().inline().into())]
    pub selection_box_style: AttrValue,
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

    //let content = use_state(|| LocalStorage::get_all::<String>());

    let store = {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let storage_length = local_storage.length();
        move |e: InputEvent| {
            // getting the files
            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
            let files = target.files().unwrap();
            for i in 0..files.length() { 
                // getting the blob url
                let file_blob = files.item(i).unwrap().slice().unwrap();
                let data_url = Url::create_object_url_with_blob(&file_blob);
                // setting local storage
                //local_storage.set_item(key, value)?;
            }
        }
    };

    html! {
        <div>
            <input onclick={open} style={ props.input_style.clone() }/>
            if *active {
                <div onclick={close}>{"x"}</div>
                <div style={ props.selection_box_style.clone() }>
                    <input oninput={store} type="file" accept="image/png, image/jpeg, image/webp" multiple=true/>
                    {"content"}
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