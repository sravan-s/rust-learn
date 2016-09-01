extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {

    let mut guess = String::new();
    println!("Guess a number!");

    io::stdin().read_line(&mut guess)
      .expect("Something went wrong");

    let guess: u32 = guess.trim().parse()
      .expect("Please type a number!");

    println!("You guessed {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too Small {}", secret_number),
      Ordering::Greater => println!("Too Big {}", secret_number),
      Ordering::Equal => {
        println!("Good guess");
        break;
      }
    }
  }
}
