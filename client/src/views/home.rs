use yew::{function_component, html, Html};

use crate::components::home::{board::Board, sidebar::Sidebar};

#[function_component(Home)]
pub fn home() -> Html {
    html!(
        <>
            <Board />
            <Sidebar />
        </>
    )
}
