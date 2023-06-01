use gloo_net::http::Request;
use yew::prelude::*;

use shared::HabitWithCompletions;

#[derive(Properties, PartialEq)]
struct HabitProps {
    habit: HabitWithCompletions,
}
#[function_component(Habit)]
fn habit_list(HabitProps { habit }: &HabitProps) -> Html {
    let clicks = use_state(|| 0);
    let other_clicks = clicks.clone();
    html! {
        <div onclick={move |_| clicks.set(*clicks + 1)}>
            <h3>{format!("{} {}/{} times {}", habit.habit.name, habit.completed + *other_clicks, habit.habit.reps, habit.habit.cadance)}</h3>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct HabitListProps {
    habits: Vec<HabitWithCompletions>,
}

#[function_component(HabitList)]
fn habit_list(HabitListProps { habits }: &HabitListProps) -> Html {
    habits
        .iter()
        .map(|habit| {
            html! {<Habit habit={habit.clone()} />}
        })
        .collect()
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
