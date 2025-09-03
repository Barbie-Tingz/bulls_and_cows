use rand::Rng; 
use std::io; 
use std::cmp::Ordering; 

fn main() {
    println!("Welcome the Bulls and Cows Random Number Game!"); 
    // this line uses the rand module to generate a random number and input the range
    println!("Guess a number between 1 and 20");
    let secret_num = rand::thread_rng().gen_range(1..=20);
    // this is the number of attempts the player has at the start of the game
    let mut attempts: u32 = 0; 


    // put this section into a loop to give the user multiple attempts at the random number
loop {
    // now we will be reading user input by utilizing the io module
    let mut user_guess = String::new();
    // for any reason the program may fail the error message will populate
    io::stdin().read_line(&mut user_guess).expect("Wrong guess!"); 

    // this trims the unnecessary whitespace on the orginal variable
    // parse is used to convert a string to an integer
    let guess: u32 = match user_guess.trim().parse() {
        Ok(num) => num, 
        Err(_) => {
            println!("Please input a valid number");
            continue; 
        }
    }; 

    // Range check uses the parsed number 
    if !(1..=20).contains(&guess) {
        println!("Please input a number between 1 and 20.");
        continue; 
    }
    
    attempts +=1; 

    // we are matching the secret number to the user input guess and giving a sutble hint
    // there is also the if statement that shows if the user is greater than 5 attempts. If so it will terminate.
    match guess.cmp(&secret_num) {
        Ordering::Less => {
            println!("Number is too small! Try something bigger."); 
            if attempts > 5 {
                println!("Number of attempts: {}", attempts); 
            }
        }
        Ordering::Greater => {
            println!("Number is to large! Try something smaller."); 
            if attempts > 5 {
                println!("Number of attempts: {},", attempts);
            }
        }
        Ordering::Equal => {
            println!("Congrats! That is the correct number!"); 
            println!("Number of attempts: {}", attempts);
            break; 
        }

    }
            //this is stating that the user used up all their attempts 
        if attempts == 10 {
            println!("You have tried 10 times to many. Game Over!");
            break; 
        }

}
}
