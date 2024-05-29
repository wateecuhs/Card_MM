use rand::seq::SliceRandom;
use rand::Rng;
use std::io;

#[derive(Debug, Clone, Copy)]
enum Suit {
	Hearts,
	Diamonds,
	Clubs,
	Spades
}

#[derive(Debug, Clone, Copy)]
enum Rank {
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
	Ace
}

#[derive(Debug, Clone, Copy)]
struct Card {
	rank: Rank,
	suit: Suit
}

fn generate_random_card() -> Card {
    let mut rng = rand::thread_rng();
    
    let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
    let suit = *suits.choose(&mut rng).unwrap();
    
    let ranks = [
        Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
        Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
        Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
    ];
    let rank = *ranks.choose(&mut rng).unwrap();
    
    Card { rank, suit }
}

fn get_cards() -> (Card, Card, Card) {
	generate_random_card();
	let card1 = generate_random_card();
	let card2 = generate_random_card();
	let card3 = generate_random_card();
	(card1, card2, card3)
}

fn get_amount() -> i32 {
	let mut amount_str = String::new();
	io::stdin().read_line(&mut amount_str).expect("Failed to read line");
	while amount_str.trim().parse::<i32>().is_err() {
		println!("Please enter a number");
		io::stdin().read_line(&mut amount_str).expect("Failed to read line");
	}
	amount_str.trim().parse::<i32>().unwrap()
}

fn get_action() -> i32 {
	let mut action = String::new();
	io::stdin().read_line(&mut action).expect("Failed to read line");
	while action.trim() != "Buy" && action.trim() != "Sell" && action.trim() != "Skip" {
		println!("Invalid action");
		io::stdin().read_line(&mut action).expect("Expecting \"Buy\", \"Sell\" or \"Skip\"");
	}
	if action.trim() == "Buy" {
		println!("Enter the order amount:");
		get_amount()
	} else if action.trim() == "Sell" {
		println!("Enter the order amount:");
		get_amount() * -1
	} else {
		0
	}
}

fn run_round(card1: Card, card2: Card, card3: Card) -> i32 {
	let hand_value = card1.rank as i32 + card2.rank as i32 + card3.rank as i32 + 6;
	let lower_bound = if hand_value < 10 {0} else {hand_value - 10};
	let mm_value = rand::thread_rng().gen_range(lower_bound..hand_value+10);
	let bid = mm_value - 1;
	let ask = mm_value + 1;
	println!("Hand:\n{:?} of {:?}, Hidden, Hidden", card1.rank, card1.suit);
	println!("The market maker quotes {} at {}", bid, ask);
	println!("Type \"Buy\", \"Sell\" or \"Skip\"");
	let action = get_action();
	if action > 0 {
		action * (hand_value - ask)
	}
	else if action < 0 {
		action * (hand_value - bid)
	}
	else {
		0
	}
}

fn round() {
	let (card1, card2, card3) = get_cards();
	let pnl = run_round(card1, card2, card3);
	println!("Cards dealt:\n{:?} of {:?} - Value ({})\n{:?} of {:?} - Value ({})\n{:?} of {:?} - Value ({})",
		card1.rank, card1.suit, card1.rank as i32 + 2, card2.rank, card2.suit, card2.rank as i32 + 2, card3.rank, card3.suit, card3.rank as i32 + 2);
	println!("PnL: {}", pnl)
}

fn main() {
	round();
}
