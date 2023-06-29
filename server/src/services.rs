use model::book::Book;

use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/books/{id}")]
pub async fn get_top_ten_books(pool: web::Data<PgPool>, id: web::Path<i64>) -> impl Responder {
    let id = id.into_inner();
    let books = sqlx::query!(
        r#"
        SELECT *
        FROM books
        WHERE books.book_id = $1;
        "#,
        id
    )
    .fetch_all(&**pool)
    .await;

    match books {
        Ok(books) => {
            if let Some(first) = books.first() {
                let book = Book {
                    book_id: id.clone(),
                    title: first.title.clone(),
                    language: "English".to_string(),
                    authors: None,
                    subjects: None,
                    bookshelves: None,
                    content_url: first.content_url.clone(),
                    downloads: first.downloads.clone(),
                    category: first.category.clone(),
                    cover_image_url_medium: first.cover_image_url_medium.clone(),
                    cover_image_url_small: first.cover_image_url_small.clone(),
                    release_date: first.release_date.clone(),
                };

                HttpResponse::Ok().json(book)
            } else {
                HttpResponse::NotFound().body("Book not found")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}
