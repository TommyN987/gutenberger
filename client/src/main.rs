use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1 class="text-3xl">{"Gutenberger"}</h1>
    }
}
