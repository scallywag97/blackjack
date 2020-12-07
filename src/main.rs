use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io;
use std::process;

pub fn main() {
    //variables for player money and bet amounts
    let mut player_money = 250.0;
    let mut bet= 0.0;
    let mut wantsToPlay = true;

    //prints rules of the game
    rules();
    //a vector for the deck with name and value
    let mut deck: Vec<(&str, i8)> = vec![("Ace of Clubs", 11), ("King of Clubs", 10), ("Queen of Clubs", 10), ("Jack of Clubs", 10), ("Ten of Clubs", 10), ("Nine of Clubs", 9), ("Eight of Clubs", 8),
                                         ("Seven of Clubs", 7), ("Six of Clubs", 6), ("Five of Clubs", 5), ("Four of Clubs", 4), ("Three of Clubs", 3), ("Two of Clubs", 2), ("Ace of Hearts", 11), ("King of Hearts", 10), ("Queen of Hearts", 10),
                                         ("Jack of Hearts", 10), ("Ten of Hearts", 10), ("Nine of Hearts", 9), ("Eight of Hearts", 8), ("Seven of Hearts", 7), ("Six of Hearts", 6), ("Five of Hearts", 5), ("Four of Hearts", 4), ("Three of Hearts", 3),
                                         ("Two of Hearts", 2), ("Ace of Spades", 11), ("King of Spades", 10), ("Queen of Spades", 10), ("Jack of Spades", 10), ("Ten of Spades", 10), ("Nine of Spades", 9), ("Eight of Spades", 8), ("Seven of Spades", 7),
                                         ("Six of Spades", 6), ("Five of Spades", 5), ("Four of Spades", 4), ("Three of Spades", 3), ("Two of Spades", 2), ("Ace of Diamonds", 11), ("King of Diamonds", 10), ("Queen of Diamonds", 10), ("Jack of Diamonds", 10),
                                         ("Ten of Diamonds", 10), ("Nine of Diamonds", 9), ("Eight of Diamonds", 8), ("Seven of Diamonds", 7), ("Six of Diamonds", 6), ("Five of Diamonds", 5), ("Four of Diamonds", 4), ("Three of Diamonds", 3), ("Two of Diamonds", 2)];
    //if player still wants to play, game will keep looping
    while wantsToPlay == true {

        //shuffles the deck vector
        deck.shuffle(&mut thread_rng());
        //variables for game conditions
        let mut player_playing: bool = true;
        let mut dealer_playing: bool = true;
        let mut value_total = deck[0].1 + deck[1].1;
        let mut card_count = 4;
        let mut dealer_busted = false;
        let mut player_bust = false;

        //checks to see if player actually has money
        money_check(&mut player_money);
        println!("How much would you like to bet out of your {} bet 50 press 1, bet 100 press 2, bet 250 press any other number", player_money);
        //gets user input for bet amount
        let mut newBet = String::new();
        io::stdin().read_line(&mut newBet);
        //following if else statements determine bet amount based on player's choice
        if newBet == "1\n" {    //option 1 for 10 dollars
            bet = 50.0;
            println!("you bet {}", bet);
        } else if newBet == "2\n" { //option 2 for 50 dollars
            bet = 100.0;
            println!("you bet {}", bet);
        } else {    //option 3 for 100 dollars
            bet = 250.0;
            println!("you bet {}", bet);
        }

        //prints player's first 2 cards to console
        println!("Your cards are the {} and the {}.", deck[0].0, deck[1].0);

        //if player's cards are add to 21, player's money amount is 1.5 times original
        if value_total == 21 {
            println!("B L A C K J A C K ! ! !             YOU WIN!");
            bet = bet * 1.5;
        }

        //prints value of player's cards
        println!("The value of your cards is {}.\n", value_total);

        //loops until player's turn is over
        while player_playing {
            //if player's cards add to 21, turn automatically ends and player wins
            if value_total == 21 {
                println!("You got 21! You win!");
                player_playing = false;
            } else if value_total < 21 {    // player has option to hit or stay if card value is less than 21
                println!("Would you like to hit or stay? (Enter 1 to hit or enter any other integer to stay)");
                let mut input_text = String::new(); //gets user input
                io::stdin().read_line(&mut input_text);

                if input_text == "1\n" {    //option to get another card
                    println!("You hit! Your new card is the {}.", deck[card_count].0);
                    value_total += deck[card_count].1;  //adds value of new card
                    println!("The value of your cards is {}.", value_total);
                    card_count += 1;    //gets next card in deck
                    player_playing = true;
                } else {    //moves on to next stage if player chooses to stay
                    println!("\nYou chose to stay! Dealer's turn...\n");
                    player_playing = false; //breaks player playing loop
                }
            } else {
                //conditions set if player busts, ends the round
                println!("You busted!");
                player_bust = true;
                player_playing = false;
                dealer_playing = false;
            }
        }
        //sets dealer's card value to current cards in hand
        let mut dealer_value_total = deck[2].1 + deck[3].1;
        println!("Dealer's cards are the {} and the {}.", deck[2].0, deck[3].0);
        dealer_value_total = deck[2].1 + deck[3].1;
        println!("The value of the dealer's cards is: {}.\n", dealer_value_total);

        //play's dealers turn until certain conditions are met
        while dealer_playing {
            //end's dealer's turn if dealer gets 21
            if dealer_value_total == 21 {
                println!("Dealer got 21!");
                dealer_playing = false;
                //dealer keeps receiving cards until over 16 or has 21
            } else if dealer_value_total < 22 {
                if dealer_value_total < 17 {
                    println!("Dealer hits! Dealer's new card is the {}.", deck[card_count].0);
                    dealer_value_total += deck[card_count].1;
                    println!("The value of dealer's cards is {}.", dealer_value_total);
                    card_count += 1;    //each hit is incremented in the card count
                    dealer_playing = true;
                } else {
                    //dealer stays if over 16, and ends the turn
                    println!("Dealer stays!\n");
                    dealer_playing = false;
                }
            } else {    //condition if dealer busts
                println!("\nDealer busted!");
                dealer_busted = true;
                dealer_playing = false;
            }
        }
        //win conditions based on previous round
        if dealer_busted && !player_bust {  //player wins if dealer busted and player does not
            player_money = player_wins(&mut player_money, &mut bet);    //player receives bet double the bet or 1.5x bet if blackjack if dealer busted
        } else if (value_total == dealer_value_total) { //player wins in the event of a tie
            player_money = player_wins(&mut player_money, &mut bet);    //player receives bet double the bet or 1.5x bet if blackjack
        } else if (value_total > dealer_value_total) && value_total < 22 {
            player_money = player_wins(&mut player_money, &mut bet);    //player receives bet double the bet or 1.5x bet if blackjack if neither bust but player has higher card value
        } else {
            player_money = dealer_wins(&mut player_money, &mut bet);    //player loses bet if player dealer wins
        }
        //player has choice of ending the game
        println!("Do you want to play more hands? Enter 1 for yes, enter any other integer for no.");
        let mut letsPlay = String::new();
        io::stdin().read_line(&mut letsPlay);
        if letsPlay != "1\n" {  // if player does not enter a 1, program exits
            wantsToPlay = false;
        }
    }
}
//Function: calculates player's new money amount and declares player as winner
//
// money: current player money amount
// bet: current player bet amount
//
// returns new player money amount(money + bet)
pub fn player_wins(money:&mut f64, bet:&mut f64) ->f64 {
    println!("You win! Congratulations!");
    *money = *money + *bet;
    return *money;
}
//Function: calculates player's new money amount and declares dealer as winner
//
// money: current player money amount
// bet: current player bet amount
//
// returns new player money amount(money - bet)
pub fn dealer_wins(money:&mut f64, bet:&mut f64) ->f64 {
    println!("The dealer wins! Sorry you lose!");
    *money = *money - *bet;
    return *money;
}
//Function: checks to see if player money pool is 0 or lower
//
// money: current player money amount
// bet: current player bet amount
//
// ends program if money
pub fn money_check(money:&mut f64) {
    if *money <= 0.0 {
        println!("You are out of money, thanks for playing!");
        process::exit(0x0100);
    } else {
        println!("You have {} money now.", *money);
    }
}

pub fn rules() {    //prints all of the rules for this version of blackjack
    println!("Welcome to Blackjack!\n");
    println!("The following are the rules for this version of Blackjack...\n");
    println!("The objective of Blackjack is to get as close as possible to 21 without going over while simultaneously beating the dealer.");
    println!("Player and dealer are initially dealt 2 cards from a full deck of 52. The player goes first.");
    println!("Player and dealer may choose to receive another card('hit') or to keep their current hand('stay').");
    println!("If a player or dealer goes over 21, they lose.");
    println!("Player automatically wins if first 2 cards add to 21 and get 1.5 times their original bet.");
    println!("Player wins if cards add to 21 or less and beat the dealer's hand. Player automatically wins if dealer goes over 21('bust') while not busting themselves.");
    println!("Dealer loses in the event of a tie.");
    println!("Dealer must hit if their hand is 16 or less.\n");
    println!("Card Values:");
    println!("Any non-face card is worth the number that is on the card. For example the 6 of Spades is worth 6.");
    println!("All face cards have a value of 10. For example the King of Hearts is worth 10.");
    println!("Aces are worth 11 points.\n");
}