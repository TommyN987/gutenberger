use model::book::{Analytics, Author, Book, Bookshelf, Subject};

use actix_web::{get, web, HttpResponse, Responder};
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
                    analytics: None,
                };
                books.push(book);
            }

            HttpResponse::Ok().json(books)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error occurred{:?}", e)),
    }
}

#[get("/books/{id}")]
pub async fn get_book(pool: web::Data<PgPool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
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
        WHERE book_id = $1
        GROUP BY
            books.book_id,
            languages.language_name;
        "#, id
    ).fetch_one(&**pool).await;

    match res {
        Ok(res) => {
            let content: String = reqwest::get(res.content_url.as_ref().unwrap())
                .await
                .unwrap()
                .text()
                .await
                .unwrap_or_default();

            let subjects_json = res
                .subjects
                .as_ref()
                .map_or(String::from("[]"), |jv| jv.to_string());
            let subjects: Vec<Subject> = serde_json::from_str(&subjects_json).unwrap();

            let bookshelves_json = res
                .bookshelves
                .as_ref()
                .map_or(String::from("[]"), |jv| jv.to_string());
            let bookshelves: Vec<Bookshelf> = serde_json::from_str(&bookshelves_json).unwrap();

            let authors_json = res
                .authors
                .as_ref()
                .map_or(String::from("[]"), |jv| jv.to_string());
            let authors: Vec<Author> = serde_json::from_str(&authors_json).unwrap();

            let book = Book {
                book_id: res.book_id,
                authors,
                title: res.title.unwrap_or_default(),
                language: res.language_name.unwrap_or_default(),
                downloads: res.downloads.unwrap_or_default(),
                bookshelves: Some(bookshelves),
                subjects: Some(subjects),
                category: res.category.unwrap_or_default(),
                content_url: res.content_url,
                cover_image_url_small: res.cover_image_url_small,
                cover_image_url_medium: res.cover_image_url_medium,
                analytics: Some(Analytics::new(&content)),
            };

            HttpResponse::Ok().json(book)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error occurred{:?}", e)),
    }
}

#[get("/subjects")]
pub async fn get_top_subjects(pool: web::Data<PgPool>) -> impl Responder {
    let res = sqlx::query!(
        r#"
        SELECT
            subject_name,
            subject_id
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
        Ok(res) => {
            let subjects: Vec<Subject> = res
                .into_iter()
                .map(|s| Subject {
                    subject_id: s.subject_id,
                    subject_name: s.subject_name.unwrap_or_default(),
                })
                .collect();
            HttpResponse::Ok().json(subjects)
        }
        Err(_) => HttpResponse::InternalServerError().body("Internal service error"),
    }
}

#[get("/bookshelves")]
pub async fn get_top_bookshelves(pool: web::Data<PgPool>) -> impl Responder {
    let res = sqlx::query!(
        r#"
        SELECT
            shelf_name,
            shelf_id
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
        Ok(res) => {
            let bookshelves: Vec<Bookshelf> = res
                .into_iter()
                .map(|s| Bookshelf {
                    shelf_id: s.shelf_id,
                    shelf_name: s.shelf_name.unwrap_or_default(),
                })
                .collect();
            HttpResponse::Ok().json(bookshelves)
        }
        Err(_) => HttpResponse::InternalServerError().body("Internal service error"),
    }
}

#[get("/bookshelves/{shelf_id}")]
pub async fn get_books_from_bookshelf(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let shelf_id = path.into_inner();
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
        INNER JOIN
            books_bookshelves ON books.book_id = books_bookshelves.book_id
        WHERE books_bookshelves.shelf_id = $1
        GROUP BY
            books.book_id,
            languages.language_name
        ORDER BY
            books.downloads DESC;
        "#, shelf_id
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
                    analytics: None,
                };
                books.push(book);
            }

            HttpResponse::Ok().json(books)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error occurred{:?}", e)),
    }
}

#[get("/subjects/{subject_id}")]
pub async fn get_books_of_subject(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let subject_id = path.into_inner();
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
        INNER JOIN
            books_subjects ON books.book_id = books_subjects.book_id
        WHERE books_subjects.subject_id = $1
        GROUP BY
            books.book_id,
            languages.language_name
        ORDER BY
            books.downloads DESC;
        "#, subject_id
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
                    analytics: None,
                };
                books.push(book);
            }

            HttpResponse::Ok().json(books)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error occurred{:?}", e)),
    }
}

#[get("/authors/{author_id}")]
pub async fn get_books_from_author(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let author_id = path.into_inner();
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
        LEFT JOIN
            books_authors ON books.book_id = books_authors.book_id
        WHERE books_authors.author_id = $1
        GROUP BY
            books.book_id,
            languages.language_name
        ORDER BY
            books.downloads DESC;
        "#, author_id
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
                    analytics: None,
                };
                books.push(book);
            }

            HttpResponse::Ok().json(books)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error occurred{:?}", e)),
    }
}
