use std::collections::HashMap;

use model::book::{Author, Book, Bookshelf, Record, Subject};
use model::utils;

use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/")]
pub async fn get_top_ten_books(pool: web::Data<PgPool>) -> impl Responder {
    let res = sqlx::query_as!(
        Record,
        r#"
        SELECT
            books.book_id,
            books.title,
            books.content_url,
            books.downloads,
            books.category,
            books.cover_image_url_medium,
            books.cover_image_url_small,
            languages.language_name,
            authors.author_name,
            authors.year_of_birth,
            authors.year_of_death,
            subjects.subject_name,
            bookshelves.shelf_name
        FROM 
            books
        INNER JOIN
            books_authors ON books.book_id = books_authors.book_id
        INNER JOIN
            authors ON books_authors.author_id = authors.author_id
        INNER JOIN
			languages ON books.language_id = languages.language_id
        LEFT JOIN
			books_bookshelves as bookshelves_bridge ON books.book_id = bookshelves_bridge.book_id
		LEFT JOIN
			bookshelves ON bookshelves_bridge.shelf_id = bookshelves.shelf_id
        LEFT JOIN
            books_subjects ON books.book_id = books_subjects.book_id
        LEFT JOIN
            subjects ON books_subjects.subject_id = subjects.subject_id
        ORDER BY 
            books.downloads DESC
        LIMIT 160;
        "#,
    )
    .fetch_all(&**pool)
    .await;

    match res {
        Ok(res) => {
            let books = parse_book_records_response(res);

            let mut books_vec: Vec<Book> = books.into_iter().map(|(_, v)| v).collect();
            books_vec.sort_by(|a, b| b.downloads.cmp(&a.downloads));
            HttpResponse::Ok().json(books_vec)
        }
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[get("/books/{id}")]
pub async fn get_book(pool: web::Data<PgPool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let res = sqlx::query_as!(
        Record,
        r#"
        SELECT
            books.book_id,
            books.title,
            books.content_url,
            books.downloads,
            books.category,
            books.cover_image_url_medium,
            books.cover_image_url_small,
            languages.language_name,
            authors.author_name,
            authors.year_of_birth,
            authors.year_of_death,
            subjects.subject_name,
            bookshelves.shelf_name
        FROM 
            books
        INNER JOIN
            books_authors ON books.book_id = books_authors.book_id
        INNER JOIN
            authors ON books_authors.author_id = authors.author_id
        INNER JOIN
			languages ON books.language_id = languages.language_id
        LEFT JOIN
			books_bookshelves as bookshelves_bridge ON books.book_id = bookshelves_bridge.book_id
		LEFT JOIN
			bookshelves ON bookshelves_bridge.shelf_id = bookshelves.shelf_id
        LEFT JOIN
            books_subjects ON books.book_id = books_subjects.book_id
        LEFT JOIN
            subjects ON books_subjects.subject_id = subjects.subject_id
        WHERE books.book_id = $1;
        "#,
        id
    )
    .fetch_all(&**pool)
    .await;

    match res {
        Ok(res) => {
            let books = parse_book_records_response(res);

            let books_vec: Vec<Book> = books.into_iter().map(|(_, v)| v).collect();
            match books_vec.get(0) {
                Some(first_book) => HttpResponse::Ok().json(first_book),
                None => HttpResponse::NotFound().body("No book found"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[get("/subjects")]
pub async fn get_top_subjects(pool: web::Data<PgPool>) -> impl Responder {
    let res = sqlx::query_as!(
        Subject,
        r#"
        SELECT
            subject_name
        FROM
            subjects
        WHERE
            LENGTH(subject_name) > 2
        ORDER BY 
            count_of_books DESC
        LIMIT 100;
        "#
    )
    .fetch_all(&**pool)
    .await;

    match res {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().body("Internal service error"),
    }
}

#[get("/bookshelves")]
pub async fn get_top_bookshelves(pool: web::Data<PgPool>) -> impl Responder {
    let res = sqlx::query_as!(
        Bookshelf,
        r#"
        SELECT
            shelf_name
        FROM
            bookshelves
        WHERE
            LENGTH(shelf_name) > 2
        ORDER BY 
            count_of_books DESC
        LIMIT 100;
        "#
    )
    .fetch_all(&**pool)
    .await;

    match res {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().body("Internal service error"),
    }
}

#[get("/bookshelf/{shelf_name}")]
pub async fn get_books_from_bookshelf(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> impl Responder {
    let shelf_name = path.into_inner();
    let res = sqlx::query_as!(
        Record,
        r#"
        SELECT
            books.book_id,
            books.title,
            books.content_url,
            books.downloads,
            books.category,
            books.cover_image_url_medium,
            books.cover_image_url_small,
            languages.language_name,
            authors.author_name,
            authors.year_of_birth,
            authors.year_of_death,
            subjects.subject_name,
            bookshelves.shelf_name
        FROM 
            books
        INNER JOIN
            books_authors ON books.book_id = books_authors.book_id
        INNER JOIN
            authors ON books_authors.author_id = authors.author_id
        INNER JOIN
			languages ON books.language_id = languages.language_id
        LEFT JOIN
			books_bookshelves as bookshelves_bridge ON books.book_id = bookshelves_bridge.book_id
		LEFT JOIN
			bookshelves ON bookshelves_bridge.shelf_id = bookshelves.shelf_id
        LEFT JOIN
            books_subjects ON books.book_id = books_subjects.book_id
        LEFT JOIN
            subjects ON books_subjects.subject_id = subjects.subject_id
        WHERE bookshelves.shelf_name = $1; 
        "#,
        shelf_name
    )
    .fetch_all(&**pool)
    .await;

    match res {
        Ok(res) => {
            let books = parse_book_records_response(res);

            let books_vec: Vec<Book> = books.into_iter().map(|(_, v)| v).collect();
            match books_vec.get(0) {
                Some(first_book) => HttpResponse::Ok().json(first_book),
                None => HttpResponse::NotFound().body("No book found"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

fn parse_book_records_response(res: Vec<Record>) -> HashMap<i64, Book> {
    let mut books: HashMap<i64, Book> = HashMap::new();
    for record in res {
        books
            .entry(record.book_id)
            .and_modify(|book| {
                add_author(&record, book);
                add_subject(&record, book);
                add_bookshelf(&record, book);
            })
            .or_insert_with(|| Book::new(&record));
    }
    books
}

fn add_author(record: &Record, book: &mut Book) {
    let author = Author {
        author_name: record.author_name.clone(),
        year_of_birth: record.year_of_birth,
        year_of_death: record.year_of_death,
    };
    utils::add_to_vec(&mut book.authors, author);
}

fn add_subject(record: &Record, book: &mut Book) {
    if let Some(subject_name) = &record.subject_name {
        let subject = Subject {
            subject_name: Some(subject_name.clone()),
        };
        utils::add_to_vec(&mut book.subjects, subject);
    }
}

fn add_bookshelf(record: &Record, book: &mut Book) {
    if let Some(shelf_name) = &record.shelf_name {
        let shelf = Bookshelf {
            shelf_name: Some(shelf_name.clone()),
        };
        utils::add_to_vec(&mut book.bookshelves, shelf);
    }
}
