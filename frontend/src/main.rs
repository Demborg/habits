use gloo_console::log;
use gloo_net::http::Request;
use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use web_sys::HtmlDialogElement;

use shared::HabitWithCompletions;

#[derive(Properties, PartialEq)]
struct HabitProps {
    habit: HabitWithCompletions,
    callback: Callback<()>,
}

fn color_from_urgency(urgency: f64) -> String {
    if urgency <= 0.25 {
        "#05c46b".to_string()
    } else if urgency <= 0.7 {
        "#ffa801".to_string()
    } else {
        "#ff3f34".to_string()
    }
}

#[styled_component]
fn Habit(HabitProps { habit, callback}: &HabitProps) -> Html {
    let name = habit.habit.name.clone();
    let callback = callback.clone();
    let onclick = move |_| {
        let url = format!("/complete/{}", name);
        wasm_bindgen_futures::spawn_local(async move {
            Request::get(&url).send().await.expect("Couldn't complete");
            log!("Completed the task!");
        });
        callback.emit(());
    };
    html! {
    <div onclick={onclick} class={css!("
            background: ${bg};
            color: #d2dae2;
            border-radius: 20px;
            padding: 20px;
            margin-bottom: 0.5rem;
            box-shadow: 0px 5px 15px rgba(0,0,0,0.2);
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            overflow: hidden;
        ", bg = color_from_urgency(habit.urgency()))}>
        <div class={css!("display: flex; flex-direction: column;")}>
            <h2 class={css!("font-size: 2em; margin: 0px;")}>{&habit.habit.name}</h2>
            <p class={css!("font-size: 1em; opacity: 0.8; margin: 0.5em 0 0.5em 0;")}>{&habit.habit.desciription}</p>
        </div>
        <div class={css!("display: flex; flex-direction: row; font-size: 1.2em; justify-content: space-between;")}>
            <p class={css!("margin: 0;")}>{format!("{}/{}", habit.completed, habit.habit.reps)}</p>
            <p class={css!("margin: 0;")}>{&habit.habit.cadance}</p>
        </div>
    </div>
    }
}

#[derive(Properties, PartialEq)]
struct HabitListProps {
    habits: Vec<HabitWithCompletions>,
    callback: Callback<()>,
}

#[function_component]
fn HabitList(HabitListProps { habits, callback}: &HabitListProps) -> Html {
    habits
        .iter()
        .map(|habit| {
            html! {<Habit habit={habit.clone()} callback={callback} />}
        })
        .collect()
}

#[styled_component]
fn Modal() -> Html {
    let onsubmit = move |_| {
        let habit = shared::Habit {
            name: "testing again".to_string(),
            desciription: "".to_string(),
            cadance: shared::Cadance::Daily,
            reps: 10,
        };
        wasm_bindgen_futures::spawn_local(async move {
            Request::post("/habit").json(&habit).unwrap().send().await.expect("Couldn't complete");
            log!("Created a task");
        });
    };
    html!(
        <>
            <h2>{"New habit"}</h2>
            <form method="dialog" onsubmit={onsubmit}>
                <input placeholder={"name"} required={true}/>
                <input placeholder={"description"} />
                <input type={"number"} required={true} placeholder={"reps"} />
                <select>
                    <option value="Daily">{"Daily"}</option>
                    <option value="Weekly">{"Weekly"}</option>
                    <option value="Monthly">{"Monthly"}</option>
                </select>
                <button>{"Add"}</button>
            </form>
        </>
    )

}

#[styled_component]
fn App() -> Html {
    let habits = use_state(|| vec![]);
    let flag = use_state(|| 0);
    {
        let habits = habits.clone();
        use_effect_with_deps(
            move |_| {
                let habits = habits.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut fetched_habits: Vec<HabitWithCompletions> = Request::get("/habits")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    fetched_habits.sort_unstable_by(|a, b| b.urgency().total_cmp(&a.urgency()));
                    habits.set(fetched_habits);
                });
            },
            flag.clone(),
        );
    }
    let callback = {
        let flag = flag.clone();
        Callback::from(move |_| flag.set(*flag + 1))
    };
    let modal_ref = use_node_ref();
    let open_modal = Callback::from({
        let modal_ref = modal_ref.clone();
        move |_| {
            modal_ref.cast::<HtmlDialogElement>().unwrap().show_modal().unwrap();
        }
    });
    html! {
        <>
            <Global css={css!("background: #1e272e;")} />
            <dialog ref={modal_ref}>
                <Modal/>
            </dialog>
            <div class={css!("display: flex; align-items: center; justify-content: center; flex-direction: column;")}>
                <h1 class={css!("color: #d2dae2;")} onclick={open_modal}>{ "Habits" }</h1>
                <div class={css!("width: 100%; max-width: 300px;")}>
                    <HabitList habits={(*habits).clone()} callback={callback}/>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
