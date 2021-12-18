// base unit for cards
#[derive(Clone)]
pub struct Card {
    suit : String,
    value : String,
    weight : usize
}

// Black unicode:
// U+2660	U+2665	U+2666	U+2663
// ♠	    ♥	    ♦	    ♣
// White unicode:
// U+2664	U+2661	U+2662	U+2667
// ♤	    ♡	    ♢	    ♧

// function to spin up cards
// no return yet hehe
pub fn make_deck() -> Vec<Card> {

    let mut curr_deck : Vec<Card> = Vec::new();

    // set up suits
    let suit_vect : Vec<String> = vec!["♤".to_string(),
                                       "♧".to_string(),
                                       "♥".to_string(),
                                       "♦".to_string()];

    // this really is a dumb way to do this
    // values:  2, 3, 4, 5, 6, 7, 8, 9, 10, J,  Q,  K,  A
    // weights: 0, 1, 2, 3, 4, 5, 6, 7, 8,  9, 10, 11, 12
    let value_vect : Vec<String> = vec!["2".to_string(),
                                        "3".to_string(),
                                        "4".to_string(),
                                        "5".to_string(),
                                        "6".to_string(),
                                        "7".to_string(),
                                        "8".to_string(),
                                        "9".to_string(),
                                        "10".to_string(),
                                        "J".to_string(),
                                        "Q".to_string(),
                                        "K".to_string(),
                                        "A".to_string()];

    for v in 0..value_vect.len() {
        for s in 0..suit_vect.len() {
            let mut curr_card = Card{suit: suit_vect[s].clone(), value: value_vect[v].clone(), weight: v};
            curr_deck.push(curr_card);
        }
    }
    return curr_deck;
}

pub fn view_deck(Deck : Vec<Card>) {
    for c in 0..Deck.len(){
        let mut st : String = Deck[c].suit.clone();
        let mut val : String = Deck[c].value.clone();
        println!("{0}{1}", val, st);
    }
}

pub fn shuffle_deck() {
    println!("Shuffling deck, stand by...");
    println!("Deck shuffled!");
}
