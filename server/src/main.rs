use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use services::{
    get_book, get_books_from_bookshelf, get_top_bookshelves, get_top_subjects, get_top_ten_books,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod services;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get_top_ten_books)
            .service(get_book)
            .service(get_top_subjects)
            .service(get_top_bookshelves)
            .service(get_books_from_bookshelf)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
