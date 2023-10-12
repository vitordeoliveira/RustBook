use std::{cmp::Ordering, io};

pub mod chapter_2 {
    use rand::Rng;
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    pub fn creating_custom_types_for_validation() {
        let secret_number = rand::thread_rng().gen_range(1..=10000);
        loop {
            println!("Please input your guess:");

            let mut guess: String = String::new();

            crate::io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            // let guess: u32 = match guess.trim().parse() {
            //     Ok(num) => num,
            //     Err(_) => continue,
            // };:w

            let guess = Guess::new(guess.trim().parse().unwrap());

            println!("You guessed: {}",guess.value());

            match guess.value().cmp(&secret_number) {
                crate::Ordering::Less => println!("To small"),
                crate::Ordering::Greater => println!("To big"),
                crate::Ordering::Equal => {
                    println!("You win");
                    break;
                }
            }
        }
    }
}
