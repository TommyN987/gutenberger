use crate::components::home::card::Card;
use model::book::{Book, Subject};

use gloo::console::log;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html};

#[function_component(Board)]
pub fn board() -> Html {
    let books = use_state(|| None::<Vec<Book>>);

    use_effect_with_deps(
        |books| {
            let books = books.clone();

            let fut = async move {
                let client = reqwest::Client::new();
                let res = client.get("http://localhost:8040").send().await;

                match res {
                    Ok(resp) => {
                        let data = resp.text().await.unwrap_or_default();
                        let data: Vec<Book> = serde_json::from_str(&data).unwrap();
                        books.set(Some(data));
                    }
                    Err(_) => log!("err"),
                }
            };
            spawn_local(fut);

            || {}
        },
        books.clone(),
    );

    let book_cards = match *books {
        Some(ref books) => books
            .iter()
            .map(|book| html! { <Card book={book.clone()} /> })
            .collect::<Html>(),
        None => html! {},
    };

    html!(
        <section class={"col-span-1 md:col-span-7 lg:col-span-9 bg-greenish border-2 border-primary dark:border-secondary"}>
            <h2 class="flex flex-row items-center pl-8 medieval text-3xl bg-pinkish text-primary dark:text-secondary border-b-2 border-primary dark:border-secondary h-20">{"Most Popular Books"}</h2>
            <div class="flex flex-col lg:flex-row flex-wrap gap-8 justify-between p-8">
                { book_cards }
            </div>
        </section>

    )
}
