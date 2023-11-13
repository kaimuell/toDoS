use gloo_net::http::Request;

pub const BACKEND_SERVER_GET_ALL : &str = "http://localhost:8080/todo/all";
pub const BACKEND_SERVER_POST : &str = "http://localhost:8080/todo/new?item=";
pub const BACKEND_SERVER_DELETE : &str = "http://localhost:8080/todo/delete?id=";


pub fn delete(item_with_id : usize){
    let url: String = format!("{}{}", BACKEND_SERVER_DELETE, &item_with_id);
    web_sys::console::log_1(&url.clone().into());
    wasm_bindgen_futures::spawn_local(async move {
        let response = Request::delete(&url).send().await;
        match response {
            Ok(_) => {
                web_sys::console::log_1(&String::from("Delete : OK").into());
            },
            Err(e) => {web_sys::console::log_1(&format!("Delete : Failed - {}", e).into());},
        }
    });
}

pub fn post_item (name : &str) {
    let url = format!("{}{}", BACKEND_SERVER_POST, name);
    wasm_bindgen_futures::spawn_local(async move {
    let response = Request::post(&url).send().await;
    match response {
        Err(e) => {web_sys::console::log_1(&format!("POST : Failed - {}", e).into());}
        _ => {web_sys::console::log_1(&("POST : OK").into());}
    }
    });
}