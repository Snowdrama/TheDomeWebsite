use yew::{html, function_component, Component, Context, Html, Properties};

use chrono::{DateTime, Utc, TimeZone};
use chrono_tz::{US::Eastern, Tz};
use gloo_timers::callback::{Interval};


#[function_component(Header)]
pub fn header() -> Html{
    html!{
        <h1 class="title-grid dome-title">
            <div>{ "The" }</div>
            <img class="dome-logo" src="images/thedomelogo.png" />
            <div>{ "Dome" }</div>
        </h1>
    }
}

pub struct Countdown{
    time: String,
    now: DateTime<Utc>,
    target: DateTime<Utc>,
    _interval: Interval,
}

impl Countdown{
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

pub enum CountdownMessages{
    UpdateTime
}

#[derive(Properties, PartialEq)]
pub struct CountdownProperties{
    pub target_string: String,
}

impl Component for Countdown {
    type Message = CountdownMessages;
    type Properties = CountdownProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        let clock_handle = {
            let link = _ctx.link().clone();
            Interval::new(1000, move || link.send_message(CountdownMessages::UpdateTime))
        };
        let mut target_time = Utc::now();

        match Utc.datetime_from_str(&_ctx.props().target_string, "%Y-%m-%d %H:%M:%S") {
            Ok(date_time) => {
                target_time = date_time;
            },
            Err(_) => { },
        }

        Self{
            time: format!("{} days {} hours {} minutes {} seconds", 0, 0, 0, 0),
            now: Utc::now(),
            target: target_time,
            _interval: clock_handle
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CountdownMessages::UpdateTime => {
                self.update_countdown();
                true
            }
        }
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <>
                { format!("{}", self.time) }
            </>
        }
    }
}