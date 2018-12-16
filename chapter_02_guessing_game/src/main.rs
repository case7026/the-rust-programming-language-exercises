use std::io;

fn main() {
    println!("Guess the number!");
    
    println!("Please input a guess!");

    // variables are immutable by default in rust
    // mut lets us indicate we want the variable to be mutable
    // lets initialize guess with the return of the String::new() function, an empty String
    let mut guess = String::new();

    // we could have also written our function call as std::io::stdin().read_line()
    // we don't have to because we've imported std::io at the top of our file
    // & indicates a reference
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
