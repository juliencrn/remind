use chrono::prelude::*;

use crate::date::{to_timestamp, DAY};

// A card means a wanted to learn word, with its translation.
// Then with training session, it's upgraded (saved in human memory)
#[derive(Debug)]
pub struct Card {
    pub input_word: String,  // from learned lang
    pub translation: String, // to spoken lang
    pub level: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub repetition_count: u32,
}

impl Card {
    pub fn new(input_word: String, translation: String) -> Card {
        let now = Utc::now();
        Card {
            input_word,
            translation,
            level: 0,
            created_at: now,
            updated_at: now,
            repetition_count: 0,
        }
    }

    pub fn upgrade(&mut self) {
        self.updated_at = Utc::now();
        self.level += 1;
        self.repetition_count += 1;
    }

    pub fn downgrade(&mut self) {
        self.updated_at = Utc::now();
        self.level = 0;
        self.repetition_count += 1;
    }

    pub fn get_scheduled_date_ts(&self) -> u64 {
        let day_count: u64 = match self.level {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 5,
            4 => 14,
            _ => 28,
        };
        to_timestamp(&self.updated_at) + (day_count * DAY)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    fn wait() {
        thread::sleep(time::Duration::from_millis(10));
    }

    fn create_card() -> Card {
        Card::new(String::from("apple"), String::from("pomme"))
    }

    #[test]
    fn test_create_card() {
        let card = create_card();

        assert_eq!(card.input_word, String::from("apple"));
        assert_eq!(card.translation, String::from("pomme"));
        assert_eq!(card.level, 0);
        assert_eq!(card.repetition_count, 0);
    }

    #[test]
    fn test_word_revision_success() {
        let mut card = create_card();
        let updated_at = card.created_at;
        wait();

        card.upgrade();

        assert_eq!(card.level, 1);
        assert_eq!(card.repetition_count, 1);
        assert_ne!(card.updated_at, updated_at);
    }

    #[test]
    fn test_word_revision_failed() {
        let mut card = create_card();
        let updated_at = card.created_at;
        wait();

        card.upgrade();

        assert_eq!(card.level, 1);

        card.downgrade();

        assert_eq!(card.level, 0);
        assert_eq!(card.repetition_count, 2);
        assert_ne!(card.updated_at, updated_at);
    }
}
