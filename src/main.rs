use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("-------- GUESSING GAME!!! !-------");

    let answer: u32 = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line :(");
        
        let guess: u32  = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Number only, pwease!");
                continue;
            },
        };

        println!("The answer isss... {answer}");
        println!("Your guess: {guess}");

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal =>  {
                println!("You Win!");
                break;
            },
        }
    }
}
