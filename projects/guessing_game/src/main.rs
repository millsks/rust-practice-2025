use rand::Rng;
use std::io;
use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("Please enter your guess (a number between 1 and 10):");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("Type before parse: {}", type_of(&guess)); 

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        println!("Type after parse: {}", type_of(&guess));

        if guess < 1 || guess > 10 {
            println!("Your guess is out of bounds! Please guess a number between 1 and 10.");
            continue;
        }

        if guess < secret_number {
            println!("Too low! Try again.");
        } else if guess > secret_number {
            println!("Too high! Try again.");
        } else {
            println!("Congratulations! You guessed the correct number: {}", secret_number);
            break;
        }
    }
}