use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_guess = rand::thread_rng().gen_range(1, 10);
    loop {
        println!("Guess the number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_guess) {
            Ordering::Greater => println!("{:?}", "big"),
            Ordering::Less => println!("{:?}", "low"),
            Ordering::Equal => break println!("Good Gues"),
        }
    }
    // println!("You guessed: {}", guess);
}
