
use std::io; // input/output library: "To obtain user input and then print the result as output"
// By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude.
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("Secret number: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();    

        io::stdin() // The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
            .read_line(&mut guess) // read_line puts whatever the user enters into the string we pass to it.
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Congrats champs!!!");
                break;
            }
        }
    }
}
