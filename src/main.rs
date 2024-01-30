use ferris_says::say ; 
use std::io::{stdout ,stdin ,  BufWriter} ; 
use rand::Rng; 

fn main() {
    let stdout  = stdout() ; 
    let message = String::from("Hello Fellow Rustaceans") ;
    let width = message.chars().count() ; 
    let mut writer =BufWriter::new(stdout.lock());
    say(&message , width , &mut writer).unwrap();

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("Guess the Number Game Comes Below. Now, Guess a number : ");
    let mut guess = String::new() ;
    stdin().read_line(&mut guess).expect("Failed to Read Lines");
    println!("You guessed {guess}");
}
