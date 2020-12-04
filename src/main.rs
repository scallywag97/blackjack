use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io;
use std::process;

pub fn main() {
    //prints rules of the game
    rules();
    let mut deck: Vec<(&str, i8)> = vec![("Ace of Clubs", 11), ("King of Clubs", 10), ("Queen of Clubs", 10), ("Jack of Clubs", 10), ("Ten of Clubs", 10), ("Nine of Clubs", 9), ("Eight of Clubs", 8),
                                         ("Seven of Clubs", 7), ("Six of Clubs", 6), ("Five of Clubs", 5), ("Four of Clubs", 4), ("Three of Clubs", 3), ("Two of Clubs", 2), ("Ace of Hearts", 11), ("King of Hearts", 10), ("Queen of Hearts", 10),
                                         ("Jack of Hearts", 10), ("Ten of Hearts", 10), ("Nine of Hearts", 9), ("Eight of Hearts", 8), ("Seven of Hearts", 7), ("Six of Hearts", 6), ("Five of Hearts", 5), ("Four of Hearts", 4), ("Three of Hearts", 3),
                                         ("Two of Hearts", 2), ("Ace of Spades", 11), ("King of Spades", 10), ("Queen of Spades", 10), ("Jack of Spades", 10), ("Ten of Spades", 10), ("Nine of Spades", 9), ("Eight of Spades", 8), ("Seven of Spades", 7),
                                         ("Six of Spades", 6), ("Five of Spades", 5), ("Four of Spades", 4), ("Three of Spades", 3), ("Two of Spades", 2), ("Ace of Diamonds", 11), ("King of Diamonds", 10), ("Queen of Diamonds", 10), ("Jack of Diamonds", 10),
                                         ("Ten of Diamonds", 10), ("Nine of Diamonds", 9), ("Eight of Diamonds", 8), ("Seven of Diamonds", 7), ("Six of Diamonds", 6), ("Five of Diamonds", 5), ("Four of Diamonds", 4), ("Three of Diamonds", 3), ("Two of Diamonds", 2)];

    //shuffles the deck vector
    deck.shuffle(&mut thread_rng());

    println!("Your cards are the {} and the {}.", deck[0].0, deck[1].0);

    let mut playing:bool = true;
    let mut value_total = deck[0].1 + deck[1].1;
    let mut card_count = 4;
    let mut dealer_busted = false;

    if value_total == 21 {
        println!("B L A C K J A C K ! ! !             YOU WIN!");
        process::exit(0x0100);
    }

    println!("The value of your cards is: {}.\n", value_total);

    while  playing {
        if value_total == 21 {
            println!("You got 21! You win!");
            playing = false;
        } else if value_total < 22 {
            println!("Would you like to hit or stay? (Enter 1 to hit or enter any other integer to stay)");
            let mut input_text = String::new();
            io::stdin().read_line(&mut input_text);

            if input_text == "1\n" {
                    println!("You hit! Your new card is the {}.", deck[card_count].0);
                    value_total += deck[card_count].1;
                    println!("The value of your cards is {}.", value_total);
                    card_count += 1;
                    playing = true;
            } else {
                println!("\nYou chose to stay! Dealer's turn...\n");
                playing = false;
            }
        } else {
            println!("You busted! You lose!");
            process::exit(0x0100);
        }
    }
    let mut dealer_value_total = deck[2].1 + deck[3].1;
    println!("Dealer's cards are the {} and the {}.", deck[2].0, deck[3].0);
    dealer_value_total = deck[2].1 + deck[3].1;
    println!("The value of the dealer's cards is: {}.\n", dealer_value_total);
    card_count += 2;

    playing = true;
    while  playing {
        if dealer_value_total == 21 {
            println!("Dealer got 21!");
            playing = false;
        } else if dealer_value_total < 22 {
            if dealer_value_total < 17 {
                println!("Dealer hits! Dealer's new card is the {}.", deck[card_count].0);
                dealer_value_total += deck[card_count].1;
                println!("The value of dealer's cards is {}.", dealer_value_total);
                card_count += 1;
                playing = true;
            } else {
                println!("Dealer stays!\n");
                playing = false;
            }
        } else {
            println!("\nDealer busted! You win!");
            dealer_busted = true;
            playing = false;
        }
    }
    if dealer_busted {
        process::exit(0x0100);
    }
    if value_total >= dealer_value_total {
        println!("Your cards win! Congratulations!");
    } else {
        println!("The dealer's cards win! Sorry you lose!");
    }
}

pub fn rules() {
    println!("Welcome to Blackjack!\n");
    println!("The following are the rules for this version of Blackjack...\n");
    println!("The objective of Blackjack is to get as close as possible to 21 without going over while simultaneously beating the dealer.");
    println!("Player and dealer are initially dealt 2 cards from a full deck of 52. The player goes first.");
    println!("Player and dealer may choose to receive another card('hit') or to keep their current hand('stay').");
    println!("If a player or dealer goes over 21, they lose.");
    println!("Player automatically wins if first 2 cards add to 21.");
    println!("Player wins if cards add to 21 or less and beat the dealer's hand. Player automatically wins if dealer goes over 21('bust') while not busting themselves.");
    println!("Dealer loses in the event of a tie.");
    println!("Dealer must hit if their hand is 16 or less.\n");
    println!("Card Values:");
    println!("Any non-face card is worth the number that is on the card. For example the 6 of Spades is worth 6.");
    println!("All face cards have a value of 10. For example the King of Hearts is worth 10.");
    println!("Aces are worth 11 points.\n");
}