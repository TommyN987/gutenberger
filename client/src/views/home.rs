use yew::{function_component, html, use_state, Callback, Html};

use crate::components::home::board::Board;

#[function_component(Home)]
pub fn home() -> Html {
    html!(
        <>
            <Board />
            <section class="col-span-1 md:col-span-5 lg:col-span-3 bg-blue-600">
            </section>
        </>
    )
}
