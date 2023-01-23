use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    print_some_numbers();
    guessing_game()
}

fn print_some_numbers() {
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);
}

fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //    println!("Number is: {secret_number}");
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}