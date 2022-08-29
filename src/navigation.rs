use yew::{html, Html, function_component};
use yew_router::prelude::*;

use crate::pages;
use crate::common::Header;
pub struct Navigation{}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/credits")]
    Credits,
    #[at("/gallery")]
    Gallery,
    #[at("/past")]
    PastEvents,
    #[at("/contact")]
    ContactPage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Navigation{
    pub fn switch(routes: &Route) -> Html {
        match routes {
            Route::Home => html!
            {
                <>
                <pages::home_page::HomePage/>
                </>
            } ,
            Route::Credits => html! 
            {
                <>
                <pages::credits_page::CreditsPage/>
                </>
            } ,
            Route::Gallery => html! 
            { 
                <>
                <pages::gallery_page::GalleryPage/>
                </>
            } ,
            Route::PastEvents => html! 
            { 
                <>
                <pages::past_events::PastEvents/>
                </>
            } ,
            Route::ContactPage => html! 
            { 
                <>
                <pages::contact_page::ContactPage/>
                </>
            } ,
            Route::NotFound => html! { 
                <>
                <Header/>
                <NavigationMenu/>
                <h1>{ "404" }</h1>
                </>
            },
        }
    }
}



#[function_component(NavigationMenu)]
pub fn navigation_menu() -> Html{
    

    html!{
        <>
            <hr/>
                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                {" | "}
                <Link<Route> to={Route::Credits}>{ "Credits" }</Link<Route>>
                {" | "}
                <Link<Route> to={Route::Gallery}>{ "Photo Gallery" }</Link<Route>>
                {" | "}
                <Link<Route> to={Route::PastEvents}>{ "Past Events" }</Link<Route>>
                {" | "}
                <Link<Route> to={Route::ContactPage}>{ "Contact" }</Link<Route>>
            <hr/>
        </>
    }
}


