use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // println!("The Secret number is: {secret_number}");

    loop {

        println!("Please input your guess.");
    
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // shadowing which is like casting a value.
        // this version crashes on an error
        // let guess: u32 = guess
        //     .trim()
        //     .parse()
        //     .expect("Please type a number!");
        
        
        // Replaced expect() w/ match: handles errors
        // page (29) of rust programming language book
        

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }

    }
    
}
