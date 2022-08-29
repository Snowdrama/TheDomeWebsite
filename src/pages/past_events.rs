use yew::{Component, Context, html, Html};
use crate::navigation::NavigationMenu;
use crate::common::Header;

pub struct PastEvents{
}

pub struct Event{
    pub event_name:String,
    pub event_date:String,
    pub event_image_url:Option<String>,
    pub event_description:String,
}

impl PastEvents{
    fn get_events() -> Vec<Event> {
        vec![
            Event {
                event_name: "LP x IN".to_string(),
                event_date: "2022-08-26".to_string(),
                event_image_url: Some("images/event_assets/lpxin/poster.jpg".to_string()),
                event_description: "The Dome launch party, a celebration of the Reunion of Little Pogchamps and the 1 year anniversary of Inebriation Nation!".to_string(),
            },
            Event {
                event_name: "More Events Coming Soon!".to_string(),
                event_date: "".to_string(),
                event_image_url: None,
                event_description: "If you're interested in hosting an event at the dome, you can get contact information on the contact page!".to_string(),
            },
        ]
    }
}

impl Component for PastEvents {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let events = PastEvents::get_events().iter().map(|event| {

            html!{
                <div class="event-cell">
                <h1>
                { format!("{}", event.event_name) }
                </h1>
                <h1>
                { format!("{}", event.event_date) }
                </h1>
                <p>{ format!("{}", event.event_description) }</p>
                {
                    match &event.event_image_url{
                        Some(event_url) => {
                            html!{
                                <img src={ format!("{}", event_url) }/>
                            }
                        },
                        None => {
                            html!{}
                        },
                    }
                }
                
                </div>
            }
        }).collect::<Html>();


        html!{
            <>
            <Header/>
            <NavigationMenu/>
            <div class="event-grid">
            { events }
            </div>
            </>
        }
    }
}