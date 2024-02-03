use ferris_says::say ; 
use std::io::{stdout ,stdin ,  BufWriter} ; 
use rand::Rng; 
use std::cmp::Ordering ;

fn echo_name(){
    println!("I am printing out your name") ; 
}

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
    //Working with Variables 
    let y = 10 ;//Y cannot be reassinged a new value 
    println!("The value of y is {y}");
    let y = 15 ; //and I also shadowed y 
    println!("I have now shadowed y : {y}");
    let mut z = 20; 
    //I make z to be mutable . Only make a variable mutable if you will mutate it
    println!("z is now {z}");
    z = 15 ; 
    println!("z is {z}");
    //Array 
    let scores = [1 , 3 , 4 , 2 , 6] ;
    for score in scores {
        println!("The score now is {score}");
    }
    echo_name() ;
    let mut todays_date = String::new() ;
    stdin().read_line(&mut todays_date).expect("Failed to read todays date");
    let todays_date : u32 = match todays_date.trim().parse(){
        Ok(num) => num ,
        Err(_) => 0,
    }; 

    if todays_date > 1 {
        println!("You ought to be in the office");
    }else{
        println!("Todays date is {todays_date} : You should be enjoying your weekend");
    }
    
}
