use yew::{function_component, html, use_state, Callback, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub items: Vec<(String, i32)>,
}

#[function_component(SideCard)]
pub fn side_card(props: &Props) -> Html {
    let amount_of_rendered_items = use_state(|| 5);

    let handle_show_more_click = {
        let amount_of_rendered_items = amount_of_rendered_items.clone();
        Callback::from(move |_| amount_of_rendered_items.set(*amount_of_rendered_items + 5))
    };

    let handle_collapse_click = {
        let amount_of_rendered_items = amount_of_rendered_items.clone();
        Callback::from(move |_| amount_of_rendered_items.set(5))
    };

    let items_to_render = props
        .items
        .clone()
        .iter()
        .take(*amount_of_rendered_items)
        .map(|item| html! { <li class="text-xl text-primary dark:text-secondary">{&item.0}</li>})
        .collect::<Html>();

    html!(
        <div class="border-2 border-primary dark:border-secondary">
            <h4 class="flex flex-row items-center pl-8 medieval bg-beige text-3xl text-primary border-b-2 border-primary dark:border-secondary h-20">{props.title.clone()}</h4>
            <ul class="flex flex-col gap-2 px-8 pt-8">
                { items_to_render }
            </ul>
            <div class="flex flex-row justify-between px-8">
                <button class="py-4 text-xl medieval text-primary dark:text-secondary hover:underline underline-offset-8" onclick={handle_show_more_click}>{"Show more"}</button>
                { if *amount_of_rendered_items > 5 {
                        html! { <button class="py-4 text-xl medieval text-primary dark:text-secondary hover:underline underline-offset-8" onclick={handle_collapse_click}>{"Collapse"}</button>}
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    )
}
