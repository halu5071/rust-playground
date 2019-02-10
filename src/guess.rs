extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guess() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess Number");

    loop {
        println!("Input number");
        let mut guess_number = String::new();

        io::stdin().read_line(&mut guess_number)
            .expect("failure");

        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Your guess: {}", guess_number);

        match guess_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small")
        }
    }
}