use model::book::Book;

use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/")]
pub async fn get_top_ten_books(pool: web::Data<PgPool>) -> impl Responder {
    let res = sqlx::query!(
        r#"
        SELECT *
        FROM books
        ORDER BY books.downloads DESC
        LIMIT 10;
        "#,
    )
    .fetch_all(&**pool)
    .await;

    match res {
        Ok(res) => {
            let mut books: Vec<Book> = vec![];
            res.iter().for_each(|record| {
                let book = Book {
                    book_id: record.book_id,
                    title: record.title.clone(),
                    language: "English".to_string(),
                    authors: None,
                    subjects: None,
                    bookshelves: None,
                    content_url: record.content_url.clone(),
                    downloads: record.downloads,
                    category: record.category.clone(),
                    cover_image_url_medium: record.cover_image_url_medium.clone(),
                    cover_image_url_small: record.cover_image_url_small.clone(),
                    release_date: record.release_date.clone(),
                };
                books.push(book)
            });
            HttpResponse::Ok().json(books)
        }
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}
