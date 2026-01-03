use std::io;

fn main() {
    println!("-------- GUESSING GAME!!! !-------");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line :(");
    
    println!("Your guess: {guess}");
}
