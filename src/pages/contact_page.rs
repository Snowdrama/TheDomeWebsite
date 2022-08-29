use yew::{Component, Context, html, Html};
use crate::navigation::NavigationMenu;
use crate::common::Header;


pub struct ContactPage{

}

impl ContactPage{

}

impl Component for ContactPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <>
                <Header/>
                <NavigationMenu/>
                <div class="contact-grid">
                    <div class="contact-cell">
                        <img src="/images/contact_assets/snowdrama_HalfSize.png"/>
                        <h1>{"Snowdrama"}</h1>
                        <a href="https://twitter.com/_snowdrama">{"Twitter"}</a>
                    </div>
                    <div class="contact-cell">
                        <img src="/images/contact_assets/rubiks_HalfSize.png"/>
                        <h1>{"Rubiks"}</h1>
                        <p><a href="https://twitter.com/rubiksdood">{"Twitter"}</a></p>
                        <p><a href="https://rubiksartworks.carrd.co/">{"Carrd"}</a></p>
                    </div>
                </div>
            </>
        }
    }
}