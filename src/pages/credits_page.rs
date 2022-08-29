use yew::{Component, Context, html, Html};
use crate::navigation::NavigationMenu;
use crate::common::Header;
pub struct CreditsPage {}

impl CreditsPage {}


impl Component for CreditsPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <>
                <Header/>
                <NavigationMenu/>
                <h1>
                    {"Thanks to everyone that helped make The Dome possible!"}
                </h1>
                <div>
                    <div>{"Big Thanks to the DJs who played at the launch party!"}</div>
                    <div>
                        <ul>
                        <li>{"Rubiks"}</li>
                        <li>{"Xyzzyx"}</li>
                        <li>{"DJ Drekt"}</li>
                        <li>{"Ayabunnie"}</li>
                        <li>{"Devixel"}</li>
                        <li>{"Dreamzzz"}</li>
                        <li>{"Aaldrik"}</li>
                        <li>{"Divinity"}</li>
                        </ul>
                    </div>
                </div>
                <div>
                    <div>{"Thanks to everyone who helped with testing and running the event!"}</div>
                    <ul>
                        <li>{"GrandMasterJ"}</li>
                        <li>{"Mr․Krabss"}</li>
                    </ul>
                </div>
                <div>
                    {"Thanks to RabidCrab for letting us use his bot machine to record the event!"}
                </div>
            </>
        }
    }
}