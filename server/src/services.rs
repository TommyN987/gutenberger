use std::collections::HashMap;

use model::book::{Author, Book, Bookshelf, Subject};
use model::utils;

use actix_web::{get, web, HttpResponse, Responder};
use serde_json::from_str;
use sqlx::PgPool;

#[get("/")]
pub async fn get_top_ten_books(pool: web::Data<PgPool>) -> impl Responder {
    let res = sqlx::query!(
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
            COALESCE(
                (SELECT json_agg(json_build_object('author_id', authors.author_id, 'author_name', authors.author_name, 'year_of_birth', authors.year_of_birth, 'year_of_death', authors.year_of_death)) 
                FROM books_authors 
                INNER JOIN authors ON books_authors.author_id = authors.author_id 
                WHERE books.book_id = books_authors.book_id), '[]') AS authors,
            COALESCE(
                (SELECT json_agg(json_build_object('subject_id', s.subject_id, 'subject_name', s.subject_name)) 
                FROM 
                    (SELECT DISTINCT subjects.subject_id, subjects.subject_name 
                    FROM books_subjects 
                    INNER JOIN subjects ON books_subjects.subject_id = subjects.subject_id 
                    WHERE books.book_id = books_subjects.book_id) AS s), '[]') AS subjects,
            COALESCE(
                (SELECT json_agg(json_build_object('shelf_id', b.shelf_id, 'shelf_name', b.shelf_name)) 
                FROM 
                    (SELECT DISTINCT bookshelves.shelf_id, bookshelves.shelf_name 
                    FROM books_bookshelves 
                    INNER JOIN bookshelves ON books_bookshelves.shelf_id = bookshelves.shelf_id 
                    WHERE books.book_id = books_bookshelves.book_id) AS b), '[]') AS bookshelves
        FROM 
            books
        INNER JOIN
            languages ON books.language_id = languages.language_id
        GROUP BY
            books.book_id,
            languages.language_name
        ORDER BY
            books.downloads DESC
        LIMIT 10;
        "#
    ).fetch_all(&**pool).await;

    match res {
        Ok(rows) => {
            let mut books: Vec<Book> = Vec::new();

            for row in rows {
                let subjects_json = row
                    .subjects
                    .as_ref()
                    .map_or(String::from("[]"), |jv| jv.to_string());
                let subjects: Vec<Subject> = serde_json::from_str(&subjects_json).unwrap();

                let bookshelves_json = row
                    .bookshelves
                    .as_ref()
                    .map_or(String::from("[]"), |jv| jv.to_string());
                let bookshelves: Vec<Bookshelf> = serde_json::from_str(&bookshelves_json).unwrap();

                let authors_json = row
                    .authors
                    .as_ref()
                    .map_or(String::from("[]"), |jv| jv.to_string());
                let authors: Vec<Author> = serde_json::from_str(&authors_json).unwrap();

                let book = Book {
                    book_id: row.book_id,
                    authors,
                    title: row.title.unwrap_or_default(),
                    language: row.language_name.unwrap_or_default(),
                    downloads: row.downloads.unwrap_or_default(),
                    bookshelves: Some(bookshelves),
                    subjects: Some(subjects),
                    category: row.category.unwrap_or_default(),
                    content_url: row.content_url,
                    cover_image_url_small: row.cover_image_url_small,
                    cover_image_url_medium: row.cover_image_url_medium,
                };
                books.push(book);
            }

            HttpResponse::Ok().json(books)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().body("Error occurred")
        }
    }
}

// #[get("/")]
// pub async fn get_top_ten_books(pool: web::Data<PgPool>) -> impl Responder {
//     let res = sqlx::query_as!(
//         Record,
//         r#"
//         SELECT
//             books.book_id,
//             books.title,
//             books.content_url,
//             books.downloads,
//             books.category,
//             books.cover_image_url_medium,
//             books.cover_image_url_small,
//             languages.language_name,
//             authors.author_name,
//             authors.year_of_birth,
//             authors.year_of_death,
//             subjects.subject_name,
//             bookshelves.shelf_name
//         FROM
//             books
//         INNER JOIN
//             books_authors ON books.book_id = books_authors.book_id
//         INNER JOIN
//             authors ON books_authors.author_id = authors.author_id
//         INNER JOIN
// 			languages ON books.language_id = languages.language_id
//         LEFT JOIN
// 			books_bookshelves as bookshelves_bridge ON books.book_id = bookshelves_bridge.book_id
// 		LEFT JOIN
// 			bookshelves ON bookshelves_bridge.shelf_id = bookshelves.shelf_id
//         LEFT JOIN
//             books_subjects ON books.book_id = books_subjects.book_id
//         LEFT JOIN
//             subjects ON books_subjects.subject_id = subjects.subject_id
//         ORDER BY
//             books.downloads DESC
//         LIMIT 160;
//         "#,
//     )
//     .fetch_all(&**pool)
//     .await;

//     match res {
//         Ok(res) => {
//             let books = parse_book_records_response(res);

//             let mut books_vec: Vec<Book> = books.into_iter().map(|(_, v)| v).collect();
//             books_vec.sort_by(|a, b| b.downloads.cmp(&a.downloads));
//             HttpResponse::Ok().json(books_vec)
//         }
//         Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
//     }
// }

// #[get("/books/{id}")]
// pub async fn get_book(pool: web::Data<PgPool>, path: web::Path<i64>) -> impl Responder {
//     let id = path.into_inner();
//     let res = sqlx::query_as!(
//         Record,
//         r#"
//         SELECT
//             books.book_id,
//             books.title,
//             books.content_url,
//             books.downloads,
//             books.category,
//             books.cover_image_url_medium,
//             books.cover_image_url_small,
//             languages.language_name,
//             authors.author_name,
//             authors.year_of_birth,
//             authors.year_of_death,
//             subjects.subject_name,
//             bookshelves.shelf_name
//         FROM
//             books
//         INNER JOIN
//             books_authors ON books.book_id = books_authors.book_id
//         INNER JOIN
//             authors ON books_authors.author_id = authors.author_id
//         INNER JOIN
// 			languages ON books.language_id = languages.language_id
//         LEFT JOIN
// 			books_bookshelves as bookshelves_bridge ON books.book_id = bookshelves_bridge.book_id
// 		LEFT JOIN
// 			bookshelves ON bookshelves_bridge.shelf_id = bookshelves.shelf_id
//         LEFT JOIN
//             books_subjects ON books.book_id = books_subjects.book_id
//         LEFT JOIN
//             subjects ON books_subjects.subject_id = subjects.subject_id
//         WHERE books.book_id = $1;
//         "#,
//         id
//     )
//     .fetch_all(&**pool)
//     .await;

//     match res {
//         Ok(res) => {
//             let books = parse_book_records_response(res);

//             let books_vec: Vec<Book> = books.into_iter().map(|(_, v)| v).collect();
//             match books_vec.get(0) {
//                 Some(first_book) => HttpResponse::Ok().json(first_book),
//                 None => HttpResponse::NotFound().body("No book found"),
//             }
//         }
//         Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
//     }
// }

// #[get("/subjects")]
// pub async fn get_top_subjects(pool: web::Data<PgPool>) -> impl Responder {
//     let res = sqlx::query_as!(
//         Subject,
//         r#"
//         SELECT
//             subject_name
//         FROM
//             subjects
//         WHERE
//             LENGTH(subject_name) > 2
//         ORDER BY
//             count_of_books DESC
//         LIMIT 100;
//         "#
//     )
//     .fetch_all(&**pool)
//     .await;

//     match res {
//         Ok(res) => HttpResponse::Ok().json(res),
//         Err(_) => HttpResponse::InternalServerError().body("Internal service error"),
//     }
// }

// #[get("/bookshelves")]
// pub async fn get_top_bookshelves(pool: web::Data<PgPool>) -> impl Responder {
//     let res = sqlx::query_as!(
//         Bookshelf,
//         r#"
//         SELECT
//             shelf_name
//         FROM
//             bookshelves
//         WHERE
//             LENGTH(shelf_name) > 2
//         ORDER BY
//             count_of_books DESC
//         LIMIT 100;
//         "#
//     )
//     .fetch_all(&**pool)
//     .await;

//     match res {
//         Ok(res) => HttpResponse::Ok().json(res),
//         Err(_) => HttpResponse::InternalServerError().body("Internal service error"),
//     }
// }

// #[get("/bookshelf/{shelf_name}")]
// pub async fn get_books_from_bookshelf(
//     pool: web::Data<PgPool>,
//     path: web::Path<String>,
// ) -> impl Responder {
//     let shelf_name = path.into_inner();
//     let res = sqlx::query_as!(
//         Record,
//         r#"
//         SELECT
//             books.book_id,
//             books.title,
//             books.content_url,
//             books.downloads,
//             books.category,
//             books.cover_image_url_medium,
//             books.cover_image_url_small,
//             languages.language_name,
//             authors.author_name,
//             authors.year_of_birth,
//             authors.year_of_death,
//             subjects.subject_name,
//             bookshelves.shelf_name
//         FROM
//             books
//         INNER JOIN
//             books_authors ON books.book_id = books_authors.book_id
//         INNER JOIN
//             authors ON books_authors.author_id = authors.author_id
//         INNER JOIN
// 			languages ON books.language_id = languages.language_id
//         LEFT JOIN
// 			books_bookshelves as bookshelves_bridge ON books.book_id = bookshelves_bridge.book_id
// 		LEFT JOIN
// 			bookshelves ON bookshelves_bridge.shelf_id = bookshelves.shelf_id
//         LEFT JOIN
//             books_subjects ON books.book_id = books_subjects.book_id
//         LEFT JOIN
//             subjects ON books_subjects.subject_id = subjects.subject_id
//         WHERE bookshelves.shelf_name = $1;
//         "#,
//         shelf_name
//     )
//     .fetch_all(&**pool)
//     .await;

//     match res {
//         Ok(res) => {
//             let books = parse_book_records_response(res);

//             let books_vec: Vec<Book> = books.into_iter().map(|(_, v)| v).collect();
//             match books_vec.get(0) {
//                 Some(first_book) => HttpResponse::Ok().json(first_book),
//                 None => HttpResponse::NotFound().body("No book found"),
//             }
//         }
//         Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
//     }
// }

// fn parse_book_records_response(res: Vec<Record>) -> HashMap<i64, Book> {
//     let mut books: HashMap<i64, Book> = HashMap::new();
//     for record in res {
//         books
//             .entry(record.book_id)
//             .and_modify(|book| {
//                 add_author(&record, book);
//                 add_subject(&record, book);
//                 add_bookshelf(&record, book);
//             })
//             .or_insert_with(|| Book::new(&record));
//     }
//     books
// }

// fn add_author(record: &Record, book: &mut Book) {
//     let author = Author {
//         author_name: record.author_name.clone(),
//         year_of_birth: record.year_of_birth,
//         year_of_death: record.year_of_death,
//     };
//     utils::add_to_vec(&mut book.authors, author);
// }

// fn add_subject(record: &Record, book: &mut Book) {
//     if let Some(subject_name) = &record.subject_name {
//         let subject = Subject {
//             subject_name: Some(subject_name.clone()),
//         };
//         utils::add_to_vec(&mut book.subjects, subject);
//     }
// }

// fn add_bookshelf(record: &Record, book: &mut Book) {
//     if let Some(shelf_name) = &record.shelf_name {
//         let shelf = Bookshelf {
//             shelf_name: Some(shelf_name.clone()),
//         };
//         utils::add_to_vec(&mut book.bookshelves, shelf);
//     }
// }
