use yew::prelude::*;
use gloo_net::http::Request;

use shared::{HabitWithCompletions};

#[derive(Properties, PartialEq)]
struct HabitListProps {
    habits: Vec<HabitWithCompletions>,
}

#[function_component(HabitList)]
fn habit_list(HabitListProps { habits }: &HabitListProps) -> Html {
    habits.iter().map(|habit| {
        html! {
        <div>
            <h3>{format!("{} {}/{} times {}", habit.habit.name, habit.completed, habit.habit.reps, habit.habit.cadance)}</h3>
        </div>
    }}).collect()
}

#[function_component(App)]
fn app() -> Html {
    let habits = use_state(|| vec![]);
    {
        let habits = habits.clone();
        use_effect_with((), move |_| {
            let habits = habits.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_habits: Vec<HabitWithCompletions> = Request::get("/habits")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                habits.set(fetched_habits);
            });
            || ()
        });
    }
    html! {
        <div>
            <h1>{ "Habits" }</h1>
            <p> { "A rusty habit tracker" } </p>
            <HabitList habits={(*habits).clone()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
