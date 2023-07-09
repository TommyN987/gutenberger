use std::collections::HashMap;

use serde::{Deserialize, Serialize};
//use sqlx::FromRow;
use unicode_segmentation::UnicodeSegmentation;

use crate::stop_words::is_stopword;

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

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Author {
    pub author_id: i32,
    pub author_name: String,
    pub year_of_birth: Option<f64>,
    pub year_of_death: Option<f64>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Subject {
    pub subject_name: String,
    pub subject_id: i32,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Bookshelf {
    pub shelf_name: String,
    pub shelf_id: i32,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Analytics {
    pub word_map: HashMap<String, u32>,
    pub sorted_words: Vec<(String, u32)>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Default, Debug)]
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
    pub analytics: Option<Analytics>,
}

impl Analytics {
    pub fn new(content: &String) -> Self {
        let mut word_map = HashMap::new();
        let word_vec: Vec<String> = content
            .split_whitespace()
            .map(|word| {
                word.chars()
                    .filter(|ch| ch.is_alphabetic())
                    .collect::<String>()
                    .to_lowercase()
            })
            .collect();

        for word in word_vec {
            *word_map.entry(word).or_insert(0) += 1;
        }

        let mut sorted_words: Vec<(String, u32)> = word_map
            .iter()
            .filter(|&(word, _)| !is_stopword(word))
            .map(|(word, &count)| (word.clone(), count))
            .collect();

        sorted_words.sort_by(|a, b| b.1.cmp(&a.1));

        Self {
            word_map,
            sorted_words,
        }
    }

    pub fn get_count(&self, word: &String) -> Option<&u32> {
        let lower_word = word.to_lowercase();
        self.word_map.get(&lower_word)
    }

    pub fn get_top_words(&self, amount: u32) -> Vec<(usize, &(String, u32))> {
        let len = self.sorted_words.len();
        let amount = amount as usize;

        if amount > len {
            self.sorted_words
                .iter()
                .enumerate()
                .map(|(index, word)| (index + 1, word))
                .collect()
        } else {
            self.sorted_words
                .iter()
                .enumerate()
                .take(amount)
                .map(|(index, word)| (index + 1, word))
                .collect()
        }
    }

    pub fn get_total_word_count(&self) -> u32 {
        self.word_map.values().sum()
    }

    pub fn get_character_count(&self) -> usize {
        self.word_map.iter().fold(0, |acc, (word, count)| {
            let word_graphemes = word.graphemes(true).count() * *count as usize;
            acc + word_graphemes
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal() {
        let analytics = Analytics::new(&"Hello, world! Hello, everyone!".to_string());

        assert_eq!(analytics.word_map.get("hello"), Some(&2));
        assert_eq!(analytics.word_map.get("world"), Some(&1));
        assert_eq!(analytics.word_map.get("everyone"), Some(&1));
        assert_eq!(analytics.sorted_words[0], ("hello".to_string(), 2));
    }

    #[test]
    fn test_empty() {
        let analytics = Analytics::new(&"".to_string());

        assert_eq!(analytics.word_map.len(), 0);
        assert_eq!(analytics.sorted_words.len(), 0);
    }

    #[test]
    fn test_special_characters() {
        let content = "Hello, world! Hello, world! Hello, world!".to_string();
        let analytics = Analytics::new(&content);

        assert_eq!(analytics.word_map.get("hello"), Some(&3));
        assert_eq!(analytics.word_map.get("world"), Some(&3));

        assert!(analytics
            .sorted_words
            .iter()
            .any(|&(ref s, count)| s == "hello" && count == 3));
        assert!(analytics
            .sorted_words
            .iter()
            .any(|&(ref s, count)| s == "world" && count == 3));
    }

    #[test]
    fn test_stopwords() {
        let content = "the quick brown fox jumps over the lazy dog".to_string();
        let analytics = Analytics::new(&content);
        assert_eq!(analytics.word_map.get("the"), Some(&2));
        let in_sorted = analytics.sorted_words.iter().any(|(word, _)| word == "the");
        assert_eq!(in_sorted, false);
    }

    #[test]
    fn test_get_count() {
        let content = "Hello hello world world world".to_string();
        let analytics = Analytics::new(&content);

        let count = analytics.get_count(&"hello".to_string());
        assert_eq!(count, Some(&2));

        let count = analytics.get_count(&"WORLD".to_string());
        assert_eq!(count, Some(&3));

        let count = analytics.get_count(&"goodbye".to_string());
        assert_eq!(count, None);
    }

    #[test]
    fn test_get_top_words() {
        let content = "hello world hello world hello test".to_string();
        let analytics = Analytics::new(&content);

        let top_2_words = analytics.get_top_words(2);
        assert_eq!(top_2_words.len(), 2);
        assert_eq!(
            top_2_words,
            vec![
                (1, &("hello".to_string(), 3)),
                (2, &("world".to_string(), 2))
            ]
        );
    }

    #[test]
    fn test_get_top_words_more_than_exists() {
        let content = "hello world hello world hello test".to_string();
        let analytics = Analytics::new(&content);

        let top_5_words = analytics.get_top_words(5);
        assert_eq!(top_5_words.len(), 3); // only 3 unique words in content
        assert_eq!(
            top_5_words,
            vec![
                (1, &("hello".to_string(), 3)),
                (2, &("world".to_string(), 2)),
                (3, &("test".to_string(), 1)),
            ]
        );
    }

    #[test]
    fn test_total_word_count() {
        let content = "hello world hello world hello test".to_string();
        let analytics = Analytics::new(&content);
        assert_eq!(analytics.get_total_word_count(), 6);
    }

    #[test]
    fn test_total_character_count() {
        let content = "hello world hello world hello test".to_string();
        let analytics = Analytics::new(&content);
        assert_eq!(analytics.get_character_count(), 29);
    }
}
