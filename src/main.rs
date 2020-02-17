// to obtain input and print result we need to bring in the io library
// the io library comes from the standard library known as std
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the hidden number between 1 and 100");

    // gen_range includes the first param and is up to but not including 
    // the second param so this will generate a number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess: ");

    // gets a guess from the user and prints it 
    let mut guess = String::new();

    // calling the stdin function from the io module
    // saves users answer to a mutable reference of guess
    io::stdin().read_line(&mut guess)
    //  read_line returns a Result type which is used in .expect
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
