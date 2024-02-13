extern crate hello_rust;

use ferris_says::say ; 
use std::io::{stdout ,stdin ,  BufWriter} ; 
use rand::Rng; 
use std::cmp::Ordering ;
use hello_rust::english::greetings;
//use std::fs::File ; 
// use std::io::ErrorKind;
use hello_rust::largest::checker;

fn echo_name(){
    println!("I am printing out your name") ; 
}
#[derive(Debug)]
struct User {
    name : String,
    active : bool,
    age : u64
}

//Build user 
fn build_user(name : String , age : u64) -> User {
    User {
        active : true,
        name , 
        age
    }
}
//Print User 
fn print_user(user : User){
    println!("Is user active? {active} . User name is {name} , age is {age}" , name=user.name , age=user.age , active=user.active); //Use positional argument
}

//Using struct to manage complexity 
struct Rectangle {
    width : u32 ,
    height : u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self , other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    //An Example of an associated function that can be used to create new instance
    fn square(size : u32) -> Self {
        Self {
            height : size,
            width : size
        }
    }
}

 #[derive(Debug)]
 //Working with enum 
 enum IPAddress {
    V4(String),
    _V6(String),
}

//Elaborate Example with 
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}


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
    //Working with Struct 
    let user1 = build_user(String::from("Jonah"), 35);
    println!("user1 is {:#?}" , user1);
    print_user(user1);
    let user2 = build_user(String::from("Timothy"), 20);
    print_user(user2);
    
    let rect1 : Rectangle = Rectangle {
        width : 3 ,
        height : 4,
    };

    //let area = area(&Rectangle{width : 3 , height : 4});
    println!("The area is {area} :" , area = rect1.area());
    let rect2 : Rectangle = Rectangle {
        width : 2 ,
        height : 5,
    };
    println!("Can rect1 hold rect2 ? {} :" , rect1.can_hold(&rect2));
    let sq = Rectangle::square(4);
    
    println!("Square Area is {} meter square" , sq.area());

    let x = 5;
    let y = x; 
    println!("The value of x is {x} , y is {y}" , x = x  , y=y);//x is still available because it is a Literal 
    let s1 = String::from("hello");
    let s2 = s1; //After this borrowing, s1 is dropped from the memory 
    //Trying to access s1 anywhere down below in your code is an error 
    println!("The value of s2 is {s2}" , s2=s2);
    //Doing copy 
    let s3 = s2.clone() ; //Copy the heap data also alongside data stored in stack

    println!("s2 = {}, s3 = {}", s2, s3);
    let the_string = String::from("New String");
    takes_ownership(the_string);
    // println!("Is the string visible here ? {}" , the_string); //Error 

    let x = 5;                      // x comes into scope

    makes_copy(x);     // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let len = calculate_length(&s2);

    println!("The length of '{}' is {}.", s2, len); 
    let mut s4 = String::from("Adeleke");
    change(&mut s4);
    println!("{}" , s2);
    let test_word = String::from("  Money and Fame") ; 
    let result = first_word(&test_word);
    println!("The result is {}" , result);
   
    let use_enum = IPAddress::V4(String::from("127.0.0.1"));

    println!("Na use_enum {:?}", use_enum);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    println!("Hello in English {}" , hello_rust::english::greetings::hello());
    println!("Hello in English using use keyword import pattern {}" , greetings::hello()); 
    //panic!("Crash and Abort");
    //let greetings_file_result = File::open("hello.txt") ; 
    // let greetings_file = match greetings_file_result  {
    //     Ok(file) => file ,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    let nums = vec![10 , 30 , 20 , 1 , 3 , 0] ; 
    checker::largest_num(&nums);


}

fn takes_ownership(some_string : String) {  //some_string comes into scope
    println!("{}",some_string); //It is used here 
} //It is no longer available outside here 

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s : &String) -> &str {
    let bytes =  s.as_bytes();
    for (i , &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i] ;
        }
    }
    &s[..]
}

