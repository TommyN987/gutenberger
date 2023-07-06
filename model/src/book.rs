use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::utils::add_to_vec;

// pub struct Record {
//     pub title: Option<String>,
//     pub content_url: Option<String>,
//     pub book_id: i64,
//     pub language_name: Option<String>,
//     pub downloads: Option<i32>,
//     pub category: Option<String>,
//     pub cover_image_url_medium: Option<String>,
//     pub cover_image_url_small: Option<String>,
//     pub author_name: Option<String>,
//     pub year_of_birth: Option<f64>,
//     pub year_of_death: Option<f64>,
//     pub subject_name: Option<String>,
//     pub shelf_name: Option<String>,
// }

#[derive(Deserialize, Serialize, PartialEq, Clone, FromRow, Debug)]
pub struct Author {
    pub author_id: i32,
    pub author_name: String,
    pub year_of_birth: Option<f64>,
    pub year_of_death: Option<f64>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, FromRow, Debug)]
pub struct Subject {
    pub subject_name: String,
    pub subject_id: i32,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, FromRow, Debug)]
pub struct Bookshelf {
    pub shelf_name: String,
    pub shelf_id: i32,
}

#[derive(Deserialize, Serialize, FromRow, Default, Debug)]
pub struct Book {
    pub book_id: i64,
    pub authors: Vec<Author>,
    pub title: String,
    pub language: String,
    pub downloads: i32,
    pub bookshelves: Option<Vec<Bookshelf>>,
    pub subjects: Option<Vec<Subject>>,
    pub category: String,
    pub content_url: Option<String>,
    pub cover_image_url_small: Option<String>,
    pub cover_image_url_medium: Option<String>,
}

// impl Book {
//     pub fn new(record: &Record) -> Self {
//         let author = Author {
//             author_name: record.author_name.clone(),
//             year_of_birth: record.year_of_birth,
//             year_of_death: record.year_of_death,
//         };
//         let mut authors = None;
//         add_to_vec(&mut authors, author);

//         let mut subjects = None;
//         if let Some(subject_name) = &record.subject_name {
//             let subject = Subject {
//                 subject_name: Some(subject_name.clone()),
//             };
//             add_to_vec(&mut subjects, subject);
//         }

//         let mut bookshelves = None;
//         if let Some(shelf_name) = &record.shelf_name {
//             let shelf = Bookshelf {
//                 shelf_name: Some(shelf_name.clone()),
//             };
//             add_to_vec(&mut bookshelves, shelf);
//         }

//         Self {
//             book_id: record.book_id,
//             title: record.title.clone(),
//             language: record.language_name.clone(),
//             authors,
//             subjects,
//             bookshelves,
//             content_url: record.content_url.clone(),
//             downloads: record.downloads,
//             category: record.category.clone(),
//             cover_image_url_medium: record.cover_image_url_medium.clone(),
//             cover_image_url_small: record.cover_image_url_small.clone(),
//         }
//     }
// }
