use rand::{seq::SliceRandom, Rng};
use serde::Serialize;
use sqlx::FromRow;
use std::cell::LazyCell;
use uuid::Uuid;

const WORDS_STRING_LIST: &str = include_str!("../../static/all_words_cleaned.txt");
const WORDS_ARRAY: LazyCell<Vec<&str>> = LazyCell::new(|| WORDS_STRING_LIST.lines().collect());

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub owner_id: Uuid,
}

impl Group {
    pub fn generate_code() -> String {
        let mut rng = rand::thread_rng();

        let words = WORDS_ARRAY
            .choose_multiple(&mut rng, 2)
            .map(ToString::to_string)
            .collect::<Vec<String>>();

        let num = rng.gen_range(0..=9);
        let [first_word, second_word] = &words[..] else {
            unreachable!();
        };

        format!("{first_word}{num}{second_word}")
    }
}
