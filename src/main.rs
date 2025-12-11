use rand::Rng;
use std::io::stdin;

fn guess_random_number(random_number: i32, tries: u8) {
    let mut count = 0;

    loop {
        let mut guess: String = String::new();
        println!("Please Enter your Guess:");
        match stdin().read_line(&mut guess) {
            Ok(_) => (),
            Err(err) => println!("Some error occured {} ", err),
        }
        let guess: i32 = guess.trim().parse().unwrap();
        if tries == count {
            println!("Tries over");
            println!(
                "You could not guessed the number  correctly, Guess: {} , Actual Number {} ",
                guess, random_number
            );
            break;
        }
        if guess == random_number {
            println!(
                "You guessed correctly, Guess: {} , Actual Number {} ",
                guess, random_number
            );
            break;
        } else if guess < random_number {
            println!(
                "You guessed incorrecty The number is greater  ({} tries left)",
                tries - count
            );

            count += 1;
        } else if guess > random_number {
            println!(
                "You guessed incorrecty The number is less  ({} tries left)",
                tries - count
            );

            count += 1;
        }
    }
}

fn main() {
    println!("------  WELCOME TO NUMBER GUSESSING GAME ------");
    println!("Select you dificulty level: ");
    println!("Press 1 for Easy (10 guesses)");
    println!("Press 2 for Medium (7 guesses)");
    println!("Press 3 for Hard (4 guesses)");
    let mut rng = rand::rng();
    let random_number = rng.random_range(1..=100);
    let mut input: String = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(err) => println!("Some error occured {}", err),
    }
    let difficulty: u8 = input.trim().parse().unwrap_or(0);

    match difficulty {
        1 => guess_random_number(random_number, 10),
        2 => guess_random_number(random_number, 7),
        3 => guess_random_number(random_number, 4),
        _ => println!("Invalid...try again "),
    }
}
