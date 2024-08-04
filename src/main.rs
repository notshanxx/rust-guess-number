use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Hello, world!");

    println!("Input a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let error = "FAiled to read line";

    println!("The secret number is: {secret_number}");

    loop {
        let mut guess = String::new(); // <- create new empty string

        // println!("empt{}y", guess);

        

        println!("\nInput your guess");

        io::stdin().read_line(&mut guess).expect(error);
        println!("You guessed: {}", guess);

        // trim whitespaces in beggining and end of the string
        // trim method eliminates \n or \r\n
        // parse string to num if possible
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Nigga thats not a number");
                continue;
            },

        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
