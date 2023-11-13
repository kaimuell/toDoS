use serde::Deserialize;
use yew::Properties;



#[derive(Clone, PartialEq, Deserialize)]
pub struct TodoItem{
    pub id : usize,
    pub name : String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct AppState{
    pub items : Vec<TodoItem>

}