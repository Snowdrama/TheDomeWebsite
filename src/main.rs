use yew::{Component, Context, html, Html};

use yew_router::prelude::*;

mod pages;
mod navigation;
mod common;
struct App {}

pub enum Msg {
    UpdateTime,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <BrowserRouter>
                <Switch<navigation::Route> render={Switch::render( navigation::Navigation::switch)} />
            </BrowserRouter>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}