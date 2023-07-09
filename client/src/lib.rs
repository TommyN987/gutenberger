use std::ops::Deref;

use yew::{function_component, html, use_state, Callback, Html};

#[derive(PartialEq, Clone)]
pub enum Theme {
    Light,
    Dark,
}

#[function_component(App)]
pub fn app() -> Html {
    let mode = use_state(|| Theme::Dark);
    let theme = match mode.deref().clone() {
        Theme::Light => "",
        Theme::Dark => "dark",
    };

    let handle_theme_click = {
        let mode = mode.clone();
        Callback::from(move |_| {
            mode.set(match mode.deref().clone() {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            })
        })
    };

    html! {
        <div class={theme}>
            <div class="bg-secondary dark:bg-primary">
                <header class="relative flex flex-col py-8">
                    <h1 class="text-8xl medieval text-center text-primary dark:text-secondary">{"Gutenberger"}</h1>
                    <button class="absolute top-8 right-8 bg-primary dark:bg-secondary text-secondary dark:text-primary px-4 py-2" onclick={handle_theme_click}>{"Theme"}</button>
                </header>
            </div>
        </div>
    }
}
