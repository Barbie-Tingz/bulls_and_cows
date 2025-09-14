// Hangman of Doom (Guess the Word)

// there are no accented letters or emojis in this project

use std::io; 

fn display_stage(lives: usize) -> &'static str {
    let stages = [
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
   \\|   |
        |
        |
  =========",

  // head + body + both arms; stage 4
  "
    +---+
    |   |
    O   |
   \\|/  |
        |
        |
  =========", 

// head + body + both arms + left leg; stage 5
  "
    +---+
    |   |
    O   |
   \\|/  |
   /    |
        |
  =========", 

  // head + body + both arms + both legs; stage 6
  "
    +---+
    |   |
    O   |
   \\|/  |
   / \\ |
        |
  =========",
];

// clamping so that code does not panic when it reaches a highet number than 6
let max_index = stages.len() -1; // instead of having the it clamped it takes the length of the frames so it won't panic

stages[max_index as usize] // this returns the array element, that is why there is no semicolon
// now you index with clamped not lives so that the code does not panic when it reach > 6 or < 0
}


    // rendering mask from the secret word dynamically to and reveals the letters when the usre guesses them correctly 
fn render_mask(intial_mask: &[char]) -> String {
    let mut output_string = String::new(); 

    for i in 0..(intial_mask.len()) {
        if i != 0 {
            output_string.push(' '); // use push(' ') to add space 
        }
        let ch = intial_mask[i]; // this is used to push the current char from the mask
        output_string.push(ch); 
    }
    output_string
}


// created function read the user input character 
fn read_guess(guessed_letters: &[char]) -> char { 
   // this variable stores mutable user_input. 
   loop {
    let mut guess_input = String::new();

    io::stdin() // standard input handle 
       .read_line(&mut guess_input) // read a line of input into the mutable String
       .expect("Failed to read input"); 
    
     let trimmed = guess_input.trim(); // trimming extra whitespace

    
       if trimmed.chars().count() != 1 { // conditional to ensure user inputs 1 character
        println!("Enter ONE letter!");
        continue; 
       }

       let ch = trimmed.chars().next().unwrap();

       // character is alphabetic only 
       if !ch.is_alphabetic() {
        println!("Letter only (a-z)."); 
        continue; 
       }

       // lowercase only
       let ch_lowercase = ch.to_ascii_lowercase(); 
       // this is checking for duplicate attempts and then requests to enter another letter.
       if guessed_letters.contains(&ch_lowercase) {
        println!("You already guessed '{}'. Try another letter.", ch_lowercase); 
        continue; 
       }
       return ch_lowercase; 
   }
}


fn main() {
    
    println!("Welcome to Hangman of Doom!"); 

/*    // check so code does not panic when a bigger number than 6 is passed. Test check ONLY!
     for i in 0..=9 {
        println!("Stage number: {}", i); 
        println!("{}", display_stage(i)); 
    }
    println!("End of loop!"); 
 */  
    // render secret word + mask 
    let secret_word = "goblin"; // test by changing the word

    // max lives for user 
    let MAX_LIVES = 6; 
    let mut lives = MAX_LIVES; 

    // this dynamically uses the secret word provided to dynamically masks the word with the amount of letters from the secret_word bank
    let mut mask: Vec<char> = vec!['_'; secret_word.len()]; 
    println!("{}", render_mask(&mask)); 

    // compute a stage index from lives using mapping & clamp
    display_stage(lives[0..6]);  

    // each function that is being used is referencing the parameter 
    while (lives > 0) && (!is_solved(&mask)) {
        println!(display_stage(stage_index_from_lives(lives))); 
        println!(render_mask(&mask);)
        guess = read_guess(&guessed_letters); 
        apply_guess(secret_word, &mut mask, &mut lives, guess); 

        println!("{}, {}", lives, guessed_letters); 
    }

}