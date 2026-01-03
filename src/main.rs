use std::io;
use rand::Rng;

fn main() {
    println!("-------- GUESSING GAME!!! !-------");

    let answer = rand::thread_rng().gen_range(1..=10);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line :(");
    
    println!("The answer isss... {answer}");
    println!("Your guess: {guess}");
}
