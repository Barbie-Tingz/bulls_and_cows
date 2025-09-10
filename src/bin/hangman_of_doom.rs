// Hangman of Doom (Guess the Word)

use std::io; 

fn display_stage(lives: usize) -> &'static str {
    let display_stage = [
    // empty gallows; stage 0
   "
    +---+
    |   |
        |
        |
        |
        |
  =========", 

  // head; stage 1
    "
    +---+
    |   |
    O   |
        |
        |
        |
  =========",

  // head + body; stage 2
  "
    +---+
    |   |
    O   |
    |   |
        |
        |
  =========", 

  // head + body + left arm; stage 3 
  "
    +---+
    |   |
    O   |
   \\|  |
        |
        |
  =========",

  // head + body + both arms; stage 4
  "
    +---+
    |   |
    O   |
   \\|/ |
        |
        |
  =========", 

// head + body + both arms + left leg; stage 5
  "
    +---+
    |   |
    O   |
   \\|/ |
   /    |
        |
  =========", 

  // head + body + both arms + both legs; stage 6
  "
    +---+
    |   |
    O   |
   \\|/ |
   / \\ |
        |
  =========",
];

display_stage[lives as usize]
}


fn main() {
    
    println!("Welcome to Hangman of Doom!"); 

    // check so code does not panic when a bigger number than 6 is passed. Test check ONLY!
    for i in 0..=9 {
        println!("Stage number: {}", i); 
        println!("{}", display_stage(i)); 
    }
    println!("End of loop!"); 

    let secret_word = "orc"; 
    let mut mask = secret_word.len(); // uses the length of the secret word, not hardcoded

    // this variable stores mutable user_input. 
//    let mut guess_input = String::new();
//    io::stdin() // standard input handle 
//        .read_line(&mut guessed_letters) // read a line of input into the mutable String
//        .expect("Failed to read word. Please try again."); // handle potential errors 
    
    // this variable is used as a container or vector to hold the previous guesses from the user 
//    let mut guessed_letters: Vec<char> = guess_input.trim().chars().collect(); 
    // "chars" iterates through each letter one at a time
    //"collect" transforms an iterator into a collection; HashMap or Vec or FromIterator trait

//    println!("You entered: {},", guessed_letters.trim()); 


}