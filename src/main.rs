use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
    let correct = rand::rng().random_range(1..=10);
    // println!("The correct number is: {}", correct);
    println!("Hey, guess a number in between 1-10?");
    

    loop{
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e)=> {
                println!("Error with parse, try again!");
                continue;
            }
        };
        
        // MATCH EXPRESSION
        match guess.cmp(&correct){
            Ordering::Greater => println!("You guessed too high!"),
            Ordering::Less => println!("You guessed too low!"),
            Ordering::Equal => {
                println!("Correct guess!");
                break;
            }

        };
    };
        
    
    // MATCH EXPRESSION
    // let message = match guess.cmp(&correct){
    //     Ordering::Greater => "You guessed too high!",
    //     Ordering::Less => "You guessed too low!",
    //     Ordering::Equal => "Correct guess!"
    // };
    // println!("{}", message);  
    

    // IF EXPRESSION (2)
    // let message = if correct < guess{
    //     "You guessed too high!"
    // } else if correct > guess{
    //     "You guessed too low!"
    // } else {
    //     "Correct guess!"
    // };
    // println!("{}", message);

    // IF EXPRESSION (1)
    // let mut message = String::new();
    // if correct < guess{
    //     message = String::from("You guessed too high!");
    // } else if correct > guess{
    //     message = String::from("You guessed too low!");
    // } else {
    //     message = String::from("Correct guess!");
    // }
    // println!("{}", message);

    // STRING LITERAL
    // let first = "Aiswarya";
    // let last = "Walter"; 

    // String TYPE
    // let first = String::from("Aiswarya"); 
    // let last = String::from("Walter"); 
    // println!("Hello, {} {}!", first, last.to_ascii_lowercase());

    // // ARRAY
    // let data = [1, 2, 3, 4, 5];
    // // PRINT ARRAY
    // println!("{data:?}");  // in debug format

    // println!("Hey, what's your name?");
    // let mut name = String::new();

    // // READ INPUT FROM USER
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Error reading input");
    // println!("Hello {}, Welcome!", name.trim());

}