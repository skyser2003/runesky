pub struct Game {
    pub p1: Player,
    pub p2: Player
}

pub struct Player {
    pub deck: Deck,
    pub field: Field,
    pub grave: Grave,
    pub hand: Hand
}

pub struct Field {
    pub cards: Vec<Card>
}

pub struct Hand {
    pub cards: Vec<Card>
}

pub struct Grave {
    pub cards: Vec<Card>
}

pub struct Deck {
    pub cards: Vec<Card>
}

pub struct Card {
    pub name: String,
    pub text: String,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            deck: Deck::default(),
            field: Field::default(),
            grave: Grave::default(),
            hand: Hand::default()
        }
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck { cards: vec![] }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field { cards: vec![] }
    }
}

impl Default for Grave {
    fn default() -> Self {
        Grave { cards: vec![] }
    }
}

impl Default for Hand {
    fn default() -> Self {
        Hand { cards: vec![] }
    }
}