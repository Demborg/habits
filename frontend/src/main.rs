use gloo_console::log;
use gloo_net::http::Request;
use stylist::yew::{styled_component, Global};
use web_sys::{HtmlDialogElement, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

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
fn Habit(HabitProps { habit, callback }: &HabitProps) -> Html {
    let name = habit.habit.name.clone();
    let name2 = habit.habit.name.clone();
    let callback = callback.clone();
    let callback2 = callback.clone();
    let onclick = move |_| {
        let url = format!("/complete/{}", name);
        wasm_bindgen_futures::spawn_local(async move {
            Request::get(&url).send().await.expect("Couldn't complete");
            log!("Completed the task!");
        });
        callback.emit(());
    };
    let delete = move |e: MouseEvent| {
        e.stop_propagation();
        let url = format!("/habit/{}", name2);
        wasm_bindgen_futures::spawn_local(async move {
            Request::delete(&url).send().await.expect("Couldn't complete");
            log!("Deleted the task!");
        });
        callback2.emit(());
    };
    let modal_ref = use_node_ref();
    let open_modal = Callback::from({
        let modal_ref = modal_ref.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            modal_ref
                .cast::<HtmlDialogElement>()
                .unwrap()
                .show_modal()
                .unwrap();
        }
    });
    html! {
    <>
        <dialog ref={modal_ref} class={css!("
            background: #808e9b;
            color: #d2dae2;
            border-radius: 20px;
            padding: 20px;
            width: 100%;
            max-width: 300px;
            box-shadow: 0px 5px 15px rgba(0,0,0,0.2);
            flex-direction: column;
            justify-content: space-between;
            overflow: hidden;
            border: 0px;
        ")}>
            <h2>{"Delete?"}</h2>
            <form method="dialog" class={css!("display: flex; flex-direction: row; font-size: 1.2em; justify-content: space-around; margin-top: 1rem;")}>
                <button onclick={delete}>{"Delete"}</button>
                <button>{"Cancel"}</button>
            </form>
        </dialog>
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
                <div class={css!("display: flex; flex-direction: row; justify-content: space-between;")}>
                    <h2 class={css!("font-size: 2em; margin: 0px;")}>{&habit.habit.name}</h2>
                    <h2 class={css!("font-size: 1.5em; margin: 0px;")} onclick={open_modal}>{"🗑"}</h2>
                </div>
                <p class={css!("font-size: 1em; opacity: 0.8; margin: 0.5em 0 0.5em 0;")}>{&habit.habit.desciription}</p>
            </div>
            <div class={css!("display: flex; flex-direction: row; font-size: 1.2em; justify-content: space-between;")}>
                <p class={css!("margin: 0;")}>{format!("{}/{}", habit.completed, habit.habit.reps)}</p>
                <p class={css!("margin: 0;")}>{&habit.habit.cadance}</p>
            </div>
        </div>
    </>
    }
}

#[derive(Properties, PartialEq)]
struct HabitListProps {
    habits: Vec<HabitWithCompletions>,
    callback: Callback<()>,
}

#[function_component]
fn HabitList(HabitListProps { habits, callback }: &HabitListProps) -> Html {
    habits
        .iter()
        .map(|habit| {
            html! {<Habit habit={habit.clone()} callback={callback} />}
        })
        .collect()
}

#[derive(Properties, PartialEq)]
struct ModalProps {
    callback: Callback<()>,
    close: Callback<()>,
}
#[styled_component]
fn Modal(ModalProps { callback, close }: &ModalProps) -> Html {
    let callback = callback.clone();
    let close = close.clone();
    let name = use_node_ref();
    let name_clone = name.clone();
    let description = use_node_ref();
    let description_clone = description.clone();
    let reps = use_node_ref();
    let reps_clone = reps.clone();
    let cadence = use_node_ref();
    let cadence_clone = cadence.clone();
    let onsubmit = move |_| {
        let name = name_clone.cast::<HtmlInputElement>().unwrap().value();
        let description = description_clone
            .cast::<HtmlInputElement>()
            .unwrap()
            .value();
        let reps = reps_clone
            .cast::<HtmlInputElement>()
            .unwrap()
            .value_as_number();
        let cadence = cadence_clone.cast::<HtmlSelectElement>().unwrap().value();
        let habit = shared::Habit {
            id: None,
            name: name,
            desciription: description,
            cadance: shared::Cadance::from(&cadence).unwrap(),
            reps: reps as i32,
        };
        wasm_bindgen_futures::spawn_local(async move {
            Request::post("/habit")
                .json(&habit)
                .unwrap()
                .send()
                .await
                .expect("Couldn't complete");
            log!("Created a task");
        });
        callback.emit(());
    };
    html!(
        <>
            <h2>{"New habit"}</h2>
            <form method="dialog" onsubmit={onsubmit} class={css!("overflow: hidden;")}>
                <input placeholder={"name"} required={true} ref={name} class={css!("width: 100%;")}/>
                <input placeholder={"description"} ref={description} class={css!("width: 100%;")}/>
                <div class={css!("display: flex; flex-direction: row; font-size: 1.2em; justify-content: space-between;")}>
                    <input type={"number"} required={true} placeholder={"reps"} ref={reps}/>
                    <select ref={cadence}>
                        <option value="daily">{"Daily"}</option>
                        <option value="weekly">{"Weekly"}</option>
                        <option value="monthly">{"Monthly"}</option>
                    </select>
                </div>
                <div class={css!("display: flex; flex-direction: row; font-size: 1.2em; justify-content: space-around; margin-top: 1rem;")}>
                    <button>{"Add"}</button>
                    <button type="reset" onclick={move |_| close.emit(())}>{"Cancel"}</button>
                </div>
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
            modal_ref
                .cast::<HtmlDialogElement>()
                .unwrap()
                .show_modal()
                .unwrap();
        }
    });
    let close_modal = Callback::from({
        let modal_ref = modal_ref.clone();
        move |_| {
            modal_ref.cast::<HtmlDialogElement>().unwrap().close();
        }
    });
    html! {
        <>
            <Global css={css!("background: #1e272e;")} />
            <dialog ref={modal_ref} class={css!("
                background: #808e9b;
                color: #d2dae2;
                border-radius: 20px;
                padding: 20px;
                width: 100%;
                max-width: 300px;
                box-shadow: 0px 5px 15px rgba(0,0,0,0.2);
                flex-direction: column;
                justify-content: space-between;
                overflow: hidden;
                border: 0px;
            ")}>
                <Modal callback={callback.clone()} close={close_modal}/>
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
