use yew::{Component, Context, html, Html};

use chrono::{DateTime, Utc, TimeZone};
use chrono_tz::{US::Eastern, Tz};
use gloo_timers::callback::{Interval};

use crate::navigation::NavigationMenu;
use crate::common::{Header, Countdown};

pub struct HomePage {
    time: String,
    now: DateTime<Utc>,
    target: DateTime<Utc>,
    _interval: Interval,
}
pub enum Msg {
    UpdateTime,
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
        html!(
            <>
                <Header/>
                <NavigationMenu/>
                <div style="text-align:center;">
                    <h1>{"More Info Coming Soon!"}</h1>
                    <div>{"We want to have many more events at the Dome, right now nothing is planned so go check out the Gallery or the Past Events in the mean time!"}</div>
                </div>
            </>)
    }
}