use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failled to read");

        let guess: u32 = guess.trim().parse().expect("Please enter a number !");

        println!("You guessed: {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("GG");
                break;
                }
        }
    }
}
