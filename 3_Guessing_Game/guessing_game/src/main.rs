extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game !\n");

    let _secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("Secret is: {}", _secret_number);

    loop {
        println!("Guess a number: ");

        let mut _number: String = String::new();

        io::stdin()
            .read_line(&mut _number)
            .expect("Read line Fail !");

        let _number: u32 = match _number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("What you guess is: {}", _number);

        match _number.cmp(&_secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }
        println!("");
    }
}
