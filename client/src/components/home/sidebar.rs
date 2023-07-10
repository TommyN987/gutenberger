use model::book::{Bookshelf, Subject};

use gloo::console::log;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html};

use crate::components::home::side_card::SideCard;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let subjects = use_state(|| None::<Vec<Subject>>);
    let bookshelves = use_state(|| None::<Vec<Bookshelf>>);

    use_effect_with_deps(
        |subjects| {
            let subjects = subjects.clone();

            let fut = async move {
                let client = reqwest::Client::new();
                let res = client.get("http://localhost:8040/subjects").send().await;

                match res {
                    Ok(resp) => {
                        let data = resp.text().await.unwrap_or_default();
                        let data: Vec<Subject> = serde_json::from_str(&data).unwrap();
                        subjects.set(Some(data));
                    }
                    Err(_) => log!("err"),
                }
            };
            spawn_local(fut);

            || {}
        },
        subjects.clone(),
    );

    use_effect_with_deps(
        |bookshelves| {
            let bookshelves = bookshelves.clone();

            let fut = async move {
                let client = reqwest::Client::new();
                let res = client.get("http://localhost:8040/bookshelves").send().await;

                match res {
                    Ok(resp) => {
                        let data = resp.text().await.unwrap_or_default();
                        let data: Vec<Bookshelf> = serde_json::from_str(&data).unwrap();
                        bookshelves.set(Some(data));
                    }
                    Err(_) => log!("err"),
                }
            };
            spawn_local(fut);

            || {}
        },
        bookshelves.clone(),
    );

    let subject_props = match *subjects {
        Some(ref subjects) => subjects
            .iter()
            .map(|subject| (subject.subject_name.clone(), subject.subject_id))
            .collect::<Vec<(String, i32)>>(),
        None => vec![],
    };

    let bookshelf_props = match *bookshelves {
        Some(ref bookshelves) => bookshelves
            .iter()
            .map(|bookshelf| (bookshelf.shelf_name.clone(), bookshelf.shelf_id))
            .collect::<Vec<(String, i32)>>(),
        None => vec![],
    };

    html!(
        <section class="col-span-1 md:col-span-5 lg:col-span-3 flex flex-col gap-8">
            <SideCard title={"Popular Subjects".to_string()} items={subject_props} />
            <SideCard title={"Popular Bookshelves".to_string()} items={bookshelf_props} />
        </section>
    )
}
