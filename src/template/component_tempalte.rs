use yew::{Component, Context, html, Html};
use yew_router::prelude::*;

pub struct CompoenentName{

}

impl CompoenentName{

}

impl Component for CompoenentName {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <>
            
            </>
        }
    }
}