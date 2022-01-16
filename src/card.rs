use chrono::prelude::*;

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

#[derive(Debug)]
pub enum CardRes {
    Success,
    Failure,
}

impl Card {
    pub fn new(input_word: String, translation: String) -> Card {
        Card {
            input_word,
            translation,
            level: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            repetition_count: 0,
        }
    }

    pub fn revise_card(&mut self, response: CardRes) {
        self.updated_at = Utc::now();
        self.repetition_count += 1;
        self.level = match response {
            CardRes::Success => self.level + 1,
            CardRes::Failure => 0,
        };
    }

    pub fn get_scheduled_days(&self) -> usize {
        match self.level {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 5,
            4 => 14,
            5 => 28,
            _ => 54,
        }
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
    fn test_card_revision() {
        let mut card = create_card();
        let updated_at = card.created_at;
        wait();

        card.revise_card(CardRes::Success);

        // well reminded upgrades level
        assert_eq!(card.level, 1);
        assert_eq!(card.repetition_count, 1);
        assert_ne!(card.updated_at, updated_at);

        card.revise_card(CardRes::Failure);

        // forgot card downgrade level
        assert_eq!(card.level, 0);
        assert_eq!(card.repetition_count, 2);
    }
}
