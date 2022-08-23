
use yew::prelude::*;
extern crate chrono;
extern crate chrono_tz;
use chrono::{TimeZone, Utc, DateTime, Local};
use chrono_tz::US::Eastern;
use chrono_tz::Asia::Kolkata;
use chrono_tz::Tz;

struct Social{
    label: String,
    url: String,
}
struct Person {
    id: usize,
    speaker: String,
    time: DateTime<Tz>,
    socials: Vec<Social>,
}

#[function_component(App)]
fn app() -> Html {
    let videos = vec![
        Person {
            id: 0,
            speaker: "DJ Drekt".to_string(),
            time: Eastern.ymd(2022, 8, 27).and_hms(00, 00, 0),
            socials: vec! []
        },
        Person {
            id: 0,
            speaker: "Xyzzyx".to_string(),
            time: Eastern.ymd(2022, 8, 27).and_hms(00, 45, 0),
            socials: vec! []
        },
        Person {
            id: 0,
            speaker: "Rubiks".to_string(),
            time: Eastern.ymd(2022, 8, 27).and_hms(01, 30, 0),
            socials: vec! []
        },
        Person {
            id: 0,
            speaker: "Ayabunnie".to_string(),
            time: Eastern.ymd(2022, 8, 27).and_hms(02, 15, 0),
            socials: vec! []
        },
        Person {
            id: 0,
            speaker: "Devixel".to_string(),
            time: Eastern.ymd(2022, 8, 27).and_hms(03, 00, 0),
            socials: vec! []
        },
        Person {
            id: 0,
            speaker: "Dreamzzz".to_string(),
            time: Eastern.ymd(2022, 8, 27).and_hms(03, 45, 0),
            socials: vec! []
        },
        Person {
            id: 0,
            speaker: "Aaldrik".to_string(),
            time: Eastern.ymd(2022, 8, 27).and_hms(4, 30, 0),
            socials: vec! []
        },
        Person {
            id: 0,
            speaker: "Divinity".to_string(),
            time: Eastern.ymd(2022, 8, 27).and_hms(5, 15, 0),
            socials: vec! []
        },
    ];

    let videos = videos.iter().map(|video| {
        let socials = video.socials.iter().map(|social| {
            html! {
                <div>
                    <a href={format!("{}", social.url)}>{format!("{}", social.label)}</a>
                </div>
            }
        }).collect::<Html>();
        html! {
            <div>
                <div>{format!("{} - {}", video.speaker, video.time.naive_local().format("%a %b %e %H:%M"))}</div>
                <div>{ socials }</div>
            </div>
        }
    }).collect::<Html>();

    html! {
        <>
            <h1>{ "The Dome" }</h1>
            <div>
                { videos }
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}