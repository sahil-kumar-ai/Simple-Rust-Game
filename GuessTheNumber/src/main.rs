use std::{cmp::Ordering, io::{self}};
use rand;
fn main() {
    let secret: u32 = rand::random_range(1..=10);
    println!("{secret}");
    loop {
        let mut user: String = String::new();

        println!("Guess a number : ");

        io::stdin()
            .read_line(&mut user)
            .expect("Error");

        let user: u32 = user.trim().parse().expect("error");

        match user.cmp(&secret) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                print!("You guesses the right number {secret}");
                break;
            }

        }
    }
}