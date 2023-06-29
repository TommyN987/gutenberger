use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Author {
    pub author_name: String,
    pub year_of_birth: i32,
    pub year_of_death: i32,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Subject {
    pub subject_name: String,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Bookshelf {
    pub shelf_name: String,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Book {
    pub book_id: i64,
    pub authors: Option<Vec<Author>>,
    pub title: Option<String>,
    pub language: String,
    pub release_date: Option<String>,
    pub downloads: Option<i32>,
    pub bookshelves: Option<Vec<Bookshelf>>,
    pub subjects: Option<Vec<Subject>>,
    pub category: Option<String>,
    pub content_url: Option<String>,
    pub cover_image_url_small: Option<String>,
    pub cover_image_url_medium: Option<String>,
}
