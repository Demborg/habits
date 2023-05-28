use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Habits" }</h1>
            <p> { "A rusty habit tracker" } </p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
