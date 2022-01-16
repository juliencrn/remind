use chrono::prelude::*;

mod card;
mod date;

use card::Card;
use date::to_timestamp;

// Each user can choose its language pair (speak, learn) and own its word card collection.
#[derive(Debug)]
struct User {
    name: String,
    speak: Lang,
    learn: Lang,
    cards: Vec<Card>,
}

#[derive(Debug, PartialEq)]
enum Lang {
    EN,
    FR,
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

    fn create_user() -> User {
        User::new(String::from("Julien"), Lang::FR, Lang::EN)
    }

    #[test]
    fn test_create_user() {
        let julien = create_user();

        assert_eq!(julien.name, String::from("Julien"));
        assert_eq!(julien.learn, Lang::EN);
        assert_eq!(julien.cards.len(), 0);
    }

    fn create_apple_card() -> Card {
        Card::new(String::from("apple"), String::from("pomme"))
    }
    fn create_beach_card() -> Card {
        Card::new(String::from("beach"), String::from("plage"))
    }

    #[test]
    fn test_add_card_to_user() {
        let mut user = create_user();
        let card = create_apple_card();

        user.add_card(card);

        assert_eq!(user.cards.len(), 1);
    }

    #[test]
    fn test_get_revision_list() {
        let mut user = create_user();
        let mut card1 = create_apple_card();
        let card2 = create_beach_card();

        card1.upgrade();

        user.add_card(card1);
        user.add_card(card2);

        assert_eq!(user.cards.len(), 2);
        assert_eq!(user.get_revisable_cards().len(), 1);
    }
}
