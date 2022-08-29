use yew::{Component, Context, html, Html};

use chrono::{DateTime, Utc, TimeZone};
use chrono_tz::{US::Eastern, Tz};
use gloo_timers::callback::{Interval};

use crate::navigation::NavigationMenu;
use crate::common::Header;

pub struct HomePage {
    time: String,
    now: DateTime<Utc>,
    target: DateTime<Utc>,
    _interval: Interval,
}
pub enum Msg {
    UpdateTime,
}
pub struct Social{
    label: String,
    url: String,
}
pub struct Person {
    name: String,
    time: DateTime<Tz>,
    socials: Vec<Social>,
}

impl HomePage {
    fn update_countdown(&mut self){
        self.now = Utc::now();
        let diff = self.target.signed_duration_since(self.now);
        if diff.num_seconds() > 0 {
            let diff_days = diff.num_seconds() / (3600*24);
            let diff_hours = (diff.num_seconds() % (3600*24)) / 3600;
            let diff_minutes = (diff.num_seconds() % 3600) / 60;
            let diff_seconds = diff.num_seconds() % 60;
            self.time = format!("{} days {} hours {} minutes {} seconds", diff_days.to_string(), diff_hours.to_string(), diff_minutes.to_string(), diff_seconds.to_string());
        
        }
        else if diff.num_seconds() < -86400 {
            self.time = "Party's Over!".to_string();
        }
        else{
            self.time = "Party Time!".to_string();
        }
    }

}

impl Component for HomePage{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        
        let clock_handle = {
            let link = _ctx.link().clone();
            Interval::new(1000, move || link.send_message(Msg::UpdateTime))
        };

        Self {
            time: format!("{} days {} hours {} minutes {} seconds", 0, 0, 0, 0),
            now: Utc::now(),
            target: Utc.ymd(2022, 8, 27).and_hms(4, 0, 0),
            _interval: clock_handle
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTime => {
                self.update_countdown();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let djtimes = vec![
            Person {
                name: "DJ Drekt".to_string(),
                time: Eastern.ymd(2022, 8, 27).and_hms(00, 00, 0),
                socials: vec! []
            },
            Person {
                name: "Xyzzyx".to_string(),
                time: Eastern.ymd(2022, 8, 27).and_hms(00, 45, 0),
                socials: vec! []
            },
            Person {
                name: "Rubiks".to_string(),
                time: Eastern.ymd(2022, 8, 27).and_hms(01, 30, 0),
                socials: vec! []
            },
            Person {
                name: "Ayabunnie".to_string(),
                time: Eastern.ymd(2022, 8, 27).and_hms(02, 15, 0),
                socials: vec! []
            },
            Person {
                name: "Devixel".to_string(),
                time: Eastern.ymd(2022, 8, 27).and_hms(03, 00, 0),
                socials: vec! []
            },
            Person {
                name: "Dreamzzz".to_string(),
                time: Eastern.ymd(2022, 8, 27).and_hms(03, 45, 0),
                socials: vec! []
            },
            Person {
                name: "Aaldrik".to_string(),
                time: Eastern.ymd(2022, 8, 27).and_hms(4, 30, 0),
                socials: vec! []
            },
            Person {
                name: "Divinity".to_string(),
                time: Eastern.ymd(2022, 8, 27).and_hms(5, 15, 0),
                socials: vec! []
            },
        ];
    
        let djtimes = djtimes.iter().map(|video| {
            let socials = video.socials.iter().map(|social| {
                html! {
                    <div>
                        <a href={format!("{}", social.url)}>{format!("{}", social.label)}</a>
                    </div>
                }
            }).collect::<Html>();
            html! {
                <div>
                    <div>{format!("{} - {}", video.name, video.time.naive_local().format("%a %b %e %H:%M"))}</div>
                    <div>{ socials }</div>
                </div>
            }
        }).collect::<Html>();
    
        html!(
            <>
                <Header/>
                <NavigationMenu/>
                <div class="teaser-grid">
                    <img class="teaser-left" width="24%" src="images/lp_logo.png" />
                    <video class="teaser-center" width="50%" controls=true autoplay=false>
                        <source src="videos/thedometeaser.mov" type="video/mp4" />
                    </video>
                    <img  class="teaser-right" width="24%" src="images/in_logo.png" />
                </div>
                <hr/>
                <div class="set-times-container">
                    <div class="set-times">
                        <h1>
                            { "08/26 - LP x IN" }
                        </h1>
                        <div>
                            { "[ Times ]" }
                        </div>
                        <div>
                            { djtimes }
                        </div>
                    </div>
                </div>
                <hr/>
                <div class="poster-container">
                <img class="poster" src="images/Poster.jpg" />
                </div>
            </>)
    }
}