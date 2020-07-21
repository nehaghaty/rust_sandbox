use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // println!("Hello, world!");
    loop
    {
        println!("Guess the number!");
        let secret_number= rand::thread_rng().gen_range(1,101);
        //println!("the secret number is {}", secret_number);
        println!("Enter input");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //let guess: u32 = guess.trim().parse().expect("enter a number"); 
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess); 
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
