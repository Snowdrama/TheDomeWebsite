use yew::{Component, Context, html, Html};

use yew_router::prelude::*;

mod pages;
struct App {

}


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/credits")]
    Credits,
    #[not_found]
    #[at("/404")]
    NotFound,
}
impl App{
    fn switch(routes: &Route) -> Html {
        match routes {
            Route::Home => pages::home_page::page(),
            Route::Credits => html! {
                <h1>
                    {"Credits!!!"}
                </h1>
            },
            Route::NotFound => html! { <h1>{ "404" }</h1> },
        }
    }
}

impl Component for App {

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <BrowserRouter>
                <Switch<Route> render={Switch::render(App::switch)} />
            </BrowserRouter>)
    }
}



fn main() {
    yew::start_app::<App>();
}