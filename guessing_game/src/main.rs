use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MIN_VAL: u32 = 1;
const MAX_VAL: u32 = 69;

fn main() {
    // game begins
    // number is generated
    let secret_number = rand::thread_rng().gen_range(MIN_VAL..=MAX_VAL);
    // display game prompt
    println!("The secret number is between {} and {}", MIN_VAL, MAX_VAL);

    loop {
        // ask for guess
        println!("Please input your guess.");

        // get guess
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // store in guess variable
            .expect("Failed to read line.");

        // convert to number if acceptable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // test guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct! You win.");
                break; // end game
            }
        }
    }
}
