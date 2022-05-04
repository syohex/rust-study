#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut total = 0;
        for card in self.cards.iter() {
            let point = match card {
                Card::Ace => {
                    if total + 11 <= 21 {
                        11
                    } else {
                        1
                    }
                }
                Card::Two => 2,
                Card::Three => 3,
                Card::Four => 4,
                Card::Five => 5,
                Card::Six => 6,
                Card::Seven => 7,
                Card::Eight => 8,
                Card::Nine => 9,
                Card::Ten | Card::Jack | Card::Queen | Card::King => 10,
            };
            total += point;
        }

        total
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);

    println!(
        "value = {}, is_loosing_hand={}",
        hand.value(),
        hand.is_loosing_hand()
    );

    {
        let mut h = Hand::new();
        h.add(Card::Two);
        h.add(Card::Three);
        h.add(Card::Four);
        h.add(Card::Five);
        h.add(Card::Six);
        h.add(Card::Seven);
        h.add(Card::Eight);
        h.add(Card::Nine);
        h.add(Card::Ten);
        h.add(Card::Jack);
        h.add(Card::Queen);
        println!(
            "value = {}, is_loosing_hand={}",
            h.value(),
            h.is_loosing_hand()
        );
    }
}

#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);

    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}
