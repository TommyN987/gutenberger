use model::book::Book;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub book: Book,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let def = "https://www.gutenberg.org/cache/epub/71148/pg71148.cover.medium.jpg".to_string();
    html!(
        <div class="w-full lg:w-[45%] bg-secondary dark:bg-primary border-2 border-primary dark:border-secondary p-8">
            <img class="mx-auto" src={props.book.cover_image_url_medium.clone().unwrap_or(def)} alt="pic" />
            <div class="pt-4">
                <h3 class="text-primary dark:text-secondary medieval text-3xl text-center">{props.book.title.clone()}</h3>
            </div>
        </div>
    )
}
