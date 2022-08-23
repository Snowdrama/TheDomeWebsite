use yew::prelude::*;
extern crate chrono;
extern crate chrono_tz;
use chrono::{TimeZone, DateTime};
use chrono_tz::US::Eastern;
use chrono_tz::Tz;

struct Social{
    label: String,
    url: String,
}
struct Person {
    name: String,
    time: DateTime<Tz>,
    socials: Vec<Social>,
}

#[function_component(App)]
fn app() -> Html {
    let videos = vec![
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
                <div>{format!("{} - {}", video.name, video.time.naive_local().format("%a %b %e %H:%M"))}</div>
                <div>{ socials }</div>
            </div>
        }
    }).collect::<Html>();

    let logo = html!(
        <>
            <img src="images/poster.jpg" />
        </>);

    html! {
        <>
            <h1 class="title-grid dome-title">
                <div>{ "The" }</div>
                <img class="dome-logo" src="images/thedomelogo.png" />
                <div>{ "Dome" }</div>
            </h1>
            <hr/>
            <div class="teaser-grid">
                <img width="24%" src="images/lp_logo.png" />
                <video width="50%" controls=true autoplay=false>
                    <source src="videos/thedometeaser.mov" type="video/mp4" />
                </video>
                <img width="24%" src="images/in_logo.png" />
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
                        { videos }
                    </div>
                </div>
            </div>
            <hr/>
            <div class="poster-container">
            <img class="poster" src="images/Poster.jpg" />
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}