use gloo_net::http::Request;
use wasm_bindgen::JsValue;
use yew::{Properties, Component, Context, use_effect_with, Html, html, function_component};

use crate::{state::{AppState, TodoItem}, server_interactiorns::{BACKEND_SERVER_GET_ALL, delete}};



enum Msg{
    Delete{id : usize},
    Post{item : String}
}


#[derive(Properties, Clone, PartialEq)]
struct App{
    state : AppState,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let l = Self {
            state: AppState { items: Vec::new() }
        };
       
        web_sys::console::log_1(&JsValue::from_str("cloned items",));
        use_effect_with( () , move|_|{
        web_sys::console::log_1(&JsValue::from_str("started move",));
        wasm_bindgen_futures::spawn_local(async move {
            let result  = Request::get(BACKEND_SERVER_GET_ALL)
                .send()
                .await;
            match result {
                Ok(respose) => {
                    let fetched_items : Vec<TodoItem> = respose
                    .json()
                    .await
                    .unwrap();
                let b  = fetched_items.len();
                web_sys::console::log_1(&JsValue::from_str(&format!("{}", b)),);
                // web_sys::console::log_1(&JsValue::from_str(&format!("{}", b).unchecked_into::<HtmlInputElement>().value().clone()));
                l.state.items = fetched_items;},
                Err(_) => web_sys::console::log_1(&JsValue::from_str("error fetching items",))
                }
        });
        || ()
    });
    l
    }

    fn changed(&mut self, _ctx: &Context<Self>){
        true
    }
    

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        self.state.items.iter().map(|item|html!{ 
            <tr key={item.id}>
                <td>{format!("{}", item.name)}</td>
                <td> <button type="button" onclick={  
                    link.callback(|_| delete(item.id))
                    }>{"DONE"}</button> 
                </td>
            </tr>
        }).collect()
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Delete(x) => {delete(x);
                                self.state.items.into_iter().filter(|it| it.id != x)} ,
            Msg::Post(it) => {},
            _=> {}
        }
    }
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



