use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    // in this instance, rand is our library, thread_rng() is our function, and gen_range is a method belonging that function
    // inclusive on upper bound, exclusive on lower.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    loop {
            println!("Please input a guess!");

            // variables are immutable by default in rust
            // mut lets us indicate we want the variable to be mutable
            // lets initialize guess with the return of the String::new() function, an empty String
            let mut guess = String::new();

            // we could have also written our function call as std::io::stdin().read_line()
            // we don't have to because we've imported std::io at the top of our file
            // & indicates a reference
            // expect allows us to access the Result type returned from read_line, stoked on Result types tbh
            io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

            // shadowing allows us to pull the value of a previously declared variable
            // we can bind guess to the following expression
            // we need .trim() because reading from stdin inherently implies the user will press enter at the end of their input adding a \n newline character to the string
            // u32 lets parse() know what kind of number expect to receive back after it parses the string
            // the colon after guess tells rust to annotate the shadowed variable with a new type, in this case, u32
            // when we perform our comparison rust will infer that, because guess' type is now u32, secret_number must also be inferred as u32
            // parse() could fail if the string it is passed cannot be converted into a numeric value, we can handle this by calling expect on the result type returned by parse()
            // we can also perform conditional logic depending on the result type's data to ensure certain logic follows
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {}", guess);

            // like Result, ordering is an enum with a fixed set of values
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => { 
                    println!("You won!");
                    break;
            }
        }
    }
}
