use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1 class={classes!("has-text-centered")}>
                { "Home :3" }
            </h1>
        </div>
    }
}
