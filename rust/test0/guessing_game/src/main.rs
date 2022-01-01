use std::io::stdin;
use std::cmp::Ordering;
use rand::{Rng, thread_rng};
fn main() {
    let secret_number = thread_rng().gen_range(1..101);
    loop
    {
        println!("Guess the number!!");
        println!("Please input your Guess:");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse()
        {
            Ok(result) => result,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {println!("you win"); break;}
            Ordering::Greater => println!("too big"),
        }
    }
}
