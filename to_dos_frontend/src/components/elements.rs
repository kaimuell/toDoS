

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

pub const BACKEND_SERVER_GET_ALL : &str = "http://localhost:8080/todo/all";
pub const BACKEND_SERVER_POST : &str = "http://localhost:8080/todo/new?item=";
pub const BACKEND_SERVER_DELETE : &str = "http://localhost:8080/todo/delete?id=";

#[derive(Clone, PartialEq, Deserialize)]
pub struct TodoItem{
    id : usize,
    name : String,
}


#[derive(Properties, PartialEq)]
pub struct ToDoItemListProbs{
    pub items : Vec<TodoItem>,
}

#[function_component(ItemsList)]
pub fn item_list(listprobs : &ToDoItemListProbs) -> Html{
   listprobs.items.iter().map(|item|html!{ 
        <tr key={item.id}>
            <td>{format!("{}", item.name)}</td>
            <td> <button type="button" onclick={  
                let id = item.id;
                Callback::from(move |_|{
                ToDoItemListProbs::delete(id);
            }) }>{"DONE"}</button> </td>
        </tr>
    }).collect()
}

impl ToDoItemListProbs{

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



#[function_component(TextInput)]
pub fn create_input() -> Html{
    html!{
    <div>
        <input type="text" id="textinput" name="textinput" maxlength="80" size="80" />
    </div>
    }
}

impl TextInput {
    pub fn on_changed_post_item () -> Callback<Event> {
        Callback::from(move |e : Event|{
        let target : EventTarget = e
                        .target()
                        .expect("should have target");
        let value =  target.unchecked_into::<HtmlInputElement>().value();
        post_item(&value);
    })
    }
}

