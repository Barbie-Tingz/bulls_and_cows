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

// clamping so that code does not panic when it reaches a highet number than 6
let max_index = stages.len() -1; // instead of having the it clamped it takes the length of the frames so it won't panic

stages[lives as usize] // this returns the array element, that is why there is no semicolon
// now you index with clamped not lives so that the code does not panic when it reach > 6 or < 0
}


fn main() {
    
    println!("Welcome to Hangman of Doom!"); 

/*     // check so code does not panic when a bigger number than 6 is passed. Test check ONLY!
     for i in 0..=9 {
        println!("Stage number: {}", i); 
        println!("{}", display_stage(i)); 
    }
    println!("End of loop!"); 
 */ 
    let secret_word = "orc"; 
    let mut intial_mask = &secret_word.len(); // uses the length of the secret word, not hardcoded

    // rendering mask 
    fn render(intial_mask) {
        let mut output_string = ""; 

        for i in 0..(intial_mask.len()-1) {
            if i != 0 {
                " " + output_string; 
            }
            intial_mask[i] + output_string
        }
        output_string
    }
 

    // this variable stores mutable user_input. 
   let mut guess_input = String::new();
   io::stdin() // standard input handle 
       .read_line(&mut guessed_letters) // read a line of input into the mutable String
       .expect("Failed to read word. Please try again."); // handle potential errors 
    

//    println!("You entered: {},", guessed_letters.trim()); 


}