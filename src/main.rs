use ferris_says::say ; 
use std::io::{stdout ,stdin ,  BufWriter} ; 
use rand::Rng; 
use std::cmp::Ordering ;

fn main() {
    let stdout  = stdout() ; 
    let message = String::from("Hello Fellow Rustaceans") ;
    let width = message.chars().count() ; 
    let mut writer =BufWriter::new(stdout.lock());
    say(&message , width , &mut writer).unwrap();
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is: {secret_number}");
        println!("Guess the Number Game Comes Below. Now, Guess a number : ");
        let mut guess = String::new() ;
        stdin().read_line(&mut guess).expect("Failed to Read Lines");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num ,
            Err(_) => continue, 
        };
        println!("You guessed {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
