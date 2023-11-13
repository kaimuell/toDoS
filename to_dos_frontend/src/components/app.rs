use gloo_net::http::Request;
use yew::prelude::*;
use wasm_bindgen::JsValue;
use stylist::global_style;

use crate::components::elements::*;

#[function_component(App)]
pub fn app_component() -> Html {



    // Fetch Items from Server
    let items = use_state(Vec::new);
    {
    web_sys::console::log_1(&JsValue::from_str("Fetching items",));
    let items = items.clone();
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
                items.set(fetched_items);},
                Err(_) => web_sys::console::log_1(&JsValue::from_str("error fetching items",))
                }
        });
        || ()
    });
    }

    global_style!(r#"
    h1{
    color: whitesmoke;
    background-color: darkblue;
    }
    div{
    margin-left: 30px;
    margin-right: 30px;
    margin-bottom: 5px;
    align-items: center;
    text-align: center;
    } 
    "#).unwrap();

    html!(
        <>  
        
            <div>
            <h1>{ "TODOS : "}</h1>
            <input type="text" name="txt_input" value="next todo" onchange={ TextInput::on_changed_post_item() }/>
            </div>
            <div>  
                <table>
                <ItemsList items={(*items).clone()} />
                </table>
            </div>
        </>
    )
}

