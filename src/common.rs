use yew::{html, function_component};

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