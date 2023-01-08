use js_sys::Array;
use web_sys::{Event, IdbOpenDbRequest, IdbDatabase, IdbTransactionMode, IdbRequest, Blob, Url, File};
use wasm_bindgen::{JsCast, JsValue, prelude::*};
use yew::UseStateHandle;

use crate::Image;

fn get_file_blob<F>(file: web_sys::File, then: F)
where F: Fn(Blob) + 'static
{
    then(file.into());
}

// for getting data from indexedDB
pub fn set_img_sources(images: UseStateHandle<Vec<Image>>, as_blob: bool) {
    let connection = init_idb();

    // on db connection success
    let success_closure = Closure::wrap(Box::new(move |event: Event| {
        let idb_open_request: IdbOpenDbRequest = event.target().unwrap().unchecked_into();
        let idb: IdbDatabase = idb_open_request.result().unwrap().into();

        // create read transaction
        let tr = idb.transaction_with_str_and_mode("images", IdbTransactionMode::Readonly);
        let tr = tr.unwrap().object_store("images").unwrap().get_all();

        // on successfull read transaction
        let images = images.clone();
        let tr_success_closure = Closure::wrap(Box::new(move |event: Event| {
            let tr_req: IdbRequest = event.target().unwrap().unchecked_into();
            let tr_res = tr_req.result().unwrap();
            let res_arr: Vec<JsValue> = tr_res.dyn_into::<Array>().unwrap().to_vec();
            if !as_blob {
                let res_vec = res_arr.iter().map(|jsval| {
                    let url = jsval.as_string().unwrap();
                    Image{ name: "no_blob".to_string(), url: url }
                }).collect::<Vec<Image>>();
                images.set(res_vec);
            } else {
                let res_vec = res_arr.iter().map(|jsval| {
                    let blob = Blob::from(jsval.clone());
                    let name = File::from(jsval.clone()).name();
                    let url = Url::create_object_url_with_blob(&blob).unwrap();
                    Image{ name: name, url: url }
                }).collect::<Vec<Image>>();
                images.set(res_vec);
            }

        }) as Box<dyn FnMut(Event)>);
        tr.unwrap().set_onsuccess(tr_success_closure.as_ref().dyn_ref());
        tr_success_closure.forget();

    }) as Box<dyn FnMut(Event)>);
    connection.set_onsuccess(success_closure.as_ref().dyn_ref());
    success_closure.forget();
}

fn init_idb() -> IdbOpenDbRequest {
    let indexed_db_factory = web_sys::window().unwrap().indexed_db().unwrap().unwrap();
    let connection = indexed_db_factory.open_with_u32("jcm", 1).expect("Error: coudn't open db");
    
    // create stores upon creation
    let upgrade_closure = Closure::wrap(Box::new(move |event: Event| {
        let idb_open_request: IdbOpenDbRequest = event.target().unwrap().unchecked_into();
        let idb: IdbDatabase = idb_open_request.result().unwrap().into();
        idb.create_object_store("images").expect("Error: Coutn't create object store 'images'");
    }) as Box<dyn FnMut(Event)>);
    connection.set_onupgradeneeded(upgrade_closure.as_ref().dyn_ref());
    upgrade_closure.forget();

    connection
}

// for setting data in indexedDB
pub fn store_file_blob<F>(name: String, file: web_sys::File, read: F) 
    where F: Fn() + 'static + Clone
{
    get_file_blob(file, move |blob| {
        let connection = init_idb();

        // on db connection success
        let name = name.clone();
        let read = read.clone();
        let success_closure = Closure::wrap(Box::new(move |event: Event| {
            let idb_open_request: IdbOpenDbRequest = event.target().unwrap().unchecked_into();
            let idb: IdbDatabase = idb_open_request.result().unwrap().into();

            // create transaction
            let tr = idb.transaction_with_str_and_mode("images", IdbTransactionMode::Readwrite);
            let tr = tr.unwrap().object_store("images").unwrap().put_with_key(&blob, &JsValue::from_str(&name));
            
            let read = read.clone();
            let tr_success_closure = Closure::wrap(Box::new(move |_: Event| {
                read();
            }) as Box<dyn FnMut(Event)>);
            tr.unwrap().set_onsuccess(tr_success_closure.as_ref().dyn_ref());
            tr_success_closure.forget();
        
        }) as Box<dyn FnMut(Event)>);
        connection.set_onsuccess(success_closure.as_ref().dyn_ref());
        success_closure.forget();
    });
}

pub fn delete_file_blob<F>(name: &str, read: F)
    where F: Fn() + 'static + Clone
{
    let connection = init_idb();
    let read = read.clone();
    let name = name.to_string();
    let success_closure = Closure::wrap(Box::new(move |event: Event| {
        let idb_open_request: IdbOpenDbRequest = event.target().unwrap().unchecked_into();
        let idb: IdbDatabase = idb_open_request.result().unwrap().into();

        // create transaction
        let tr = idb.transaction_with_str_and_mode("images", IdbTransactionMode::Readwrite);
        let tr = tr.unwrap().object_store("images").unwrap().delete( &JsValue::from_str(&name));
        
        let read = read.clone();
        let tr_success_closure = Closure::wrap(Box::new(move |_: Event| {
            read();
        }) as Box<dyn FnMut(Event)>);
        tr.unwrap().set_onsuccess(tr_success_closure.as_ref().dyn_ref());
        tr_success_closure.forget();
    
    }) as Box<dyn FnMut(Event)>);
    connection.set_onsuccess(success_closure.as_ref().dyn_ref());
    success_closure.forget();
}