use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("-------- GUESSING GAME!!! !-------");

    let answer: u32 = rand::thread_rng().gen_range(1..=10);
    let mut guess = String::new();

    println!("Please input your guess.");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line :(");
    
    let guess: u32  = guess.trim().parse()
        .expect("number only, pwease!");

    println!("The answer isss... {answer}");
    println!("Your guess: {guess}");

    match guess.cmp(&answer) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You Win!"),
        Ordering::Greater => println!("Too Big!"),
    }
}
