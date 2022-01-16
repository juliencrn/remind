use chrono::prelude::*;
use std::convert::TryFrom;

const DAY: u64 = 24 * 60 * 60 * 1000;

// Each user can choose its language pair (speak, learn) and own its word card collection.
#[derive(Debug)]
struct User {
    name: String,
    speak: Lang,
    learn: Lang,
    cards: Vec<Card>,
}

// A card means a wanted to learn word, with its translation.
// Then with training session, it's upgraded (saved in human memory)
#[derive(Debug)]
struct Card {
    input_word: String,  // from learned lang
    translation: String, // to spoken lang
    level: usize,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    repetition_count: u32,
}

#[derive(Debug, PartialEq)]
enum Lang {
    EN,
    FR,
}

pub fn to_timestamp(date: &DateTime<Utc>) -> u64 {
    u64::try_from(date.timestamp()).unwrap()
}

pub fn from_timestamp(ts: u64) -> DateTime<Utc> {
    Utc.timestamp(i64::try_from(ts).unwrap(), 0)
}

impl User {
    fn new(name: String, speak: Lang, learn: Lang) -> User {
        User {
            name,
            speak,
            learn,
            cards: Vec::new(),
        }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn get_revisable_cards(&self) -> Vec<&Card> {
        let mut revisable_cards: Vec<&Card> = vec![];
        let now_ts = to_timestamp(&Utc::now());

        for card in self.cards.iter() {
            if now_ts >= card.get_scheduled_date_ts() {
                revisable_cards.push(card);
            }
        }

        revisable_cards
    }
}

impl Card {
    fn new(input_word: String, translation: String) -> Card {
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

    fn upgrade(&mut self) {
        self.updated_at = Utc::now();
        self.level += 1;
        self.repetition_count += 1;
    }

    fn downgrade(&mut self) {
        self.updated_at = Utc::now();
        self.level = 0;
        self.repetition_count += 1;
    }

    fn get_scheduled_date_ts(&self) -> u64 {
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

fn main() {
    let mut julien = User::new(String::from("julien"), Lang::FR, Lang::EN);

    let julien_s_card_1 = Card::new(String::from("bike"), String::from("vélo"));
    let julien_s_card_2 = Card::new(String::from("apple"), String::from("pomme"));
    let julien_s_card_3 = Card::new(String::from("beach"), String::from("plage"));

    julien.add_card(julien_s_card_1);
    julien.add_card(julien_s_card_2);
    julien.add_card(julien_s_card_3);

    let to_learn = julien.get_revisable_cards();

    println!("Client julien: {:#?}", julien);
    println!("Revisable cards: {:#?}", to_learn);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    fn create_user(name: String) -> User {
        User::new(name, Lang::FR, Lang::EN)
    }

    #[test]
    fn test_create_user() {
        let name = "Julien";
        let julien = create_user(String::from(name));

        assert_eq!(julien.name, String::from(name));
        assert_eq!(julien.learn, Lang::EN);
        assert_eq!(julien.cards.len(), 0);
    }

    #[test]
    fn test_create_card() {
        let learn = "apple";
        let translation = "pomme";
        let card = Card::new(String::from(learn), String::from(translation));

        assert_eq!(card.input_word, String::from(learn));
        assert_eq!(card.translation, String::from(translation));
        assert_eq!(card.level, 0);
        assert_eq!(card.repetition_count, 0);
    }

    fn create_apple_card() -> Card {
        Card::new(String::from("apple"), String::from("pomme"))
    }
    fn create_beach_card() -> Card {
        Card::new(String::from("beach"), String::from("plage"))
    }

    fn wait() {
        thread::sleep(time::Duration::from_millis(10));
    }

    #[test]
    fn test_add_card_to_user() {
        let mut user = create_user(String::from("Julien"));
        let card = create_apple_card();
        user.add_card(card);

        assert_eq!(user.cards.len(), 1);
    }

    #[test]
    fn test_word_revision_success() {
        let mut card = create_apple_card();
        let updated_at = card.created_at;
        wait();

        card.upgrade();

        assert_eq!(card.level, 1);
        assert_eq!(card.repetition_count, 1);
        assert_ne!(card.updated_at, updated_at);
    }

    #[test]
    fn test_word_revision_failed() {
        let mut card = create_apple_card();
        let updated_at = card.created_at;
        wait();

        card.upgrade();

        assert_eq!(card.level, 1);

        card.downgrade();

        assert_eq!(card.level, 0);
        assert_eq!(card.repetition_count, 2);
        assert_ne!(card.updated_at, updated_at);
    }

    #[test]
    fn test_get_revision_list() {
        let mut user = create_user(String::from("julien"));
        let mut card1 = create_apple_card();
        let card2 = create_beach_card();

        card1.upgrade();

        user.add_card(card1);
        user.add_card(card2);

        println!("{:#?}", user);

        assert_eq!(user.cards.len(), 2);
        assert_eq!(user.get_revisable_cards().len(), 1);
    }
}
