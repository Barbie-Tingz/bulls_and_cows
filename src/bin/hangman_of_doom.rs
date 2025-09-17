// Hangman of Doom (Guess the Word)

// there are no accented letters or emojis in this project

/* the spooky word bank : 
agonizing, bizarre, cauldron, cemetery, phantom, enchanting, frankenstein, horror, laboratory, masquerade, nefarious,
poltergeist, revolting, scarecrow, sorcerer, transylvania, vengeance, werewolf, zombie
*/


/*

\\                  // || ||\\    || ||\\    || ||======  ||=====\\  ||
 \\       __       //  || || \\   || || \\   || ||        ||     ||  ||
  \\     //\\     //   || ||  \\  || ||  \\  || ||====    ||====\\   ||
   \\   //  \\   //    || ||   \\ || ||   \\ || ||        ||     \\  ||
    \\=//    \\=//     || ||    \\|| ||    \\|| ||======= ||      \\ ++

||        ||======|| ========  ||======= ||=====\\                   
||        ||      || ||        ||        ||      ||         
||        ||      || ||        ||        ||=====//             
||        ||      || ||=====|| ||====    ||     \\            
||        ||      ||        || ||        ||      \\            
||======= ||======|| =======|| ||======= ||       \\          
*/


use std::io; 
use rand::seq::SliceRandom; 
use rand::thread_rng;

fn display_stage(index: usize) -> &'static str {
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
let max_index = stages.len(); // instead of having the it clamped it takes the length of the frames so it won't panic

// clamped the index into a the safe range
let clamped = index.clamp(0, max_index); 

stages[clamped] // this returns the array element, that is why there is no semicolon
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

// enum for the outcome of the apply_guess function
enum Outcome {
  Hit(char), 
  Miss(char),
}

// checks the new guess against the secret_word aka "goblin"
fn apply_guess(secret_word: &str, mask: &mut Vec<char>, lives: &mut u32, guess: char) -> Outcome {
  // inputs : secret word(lowercase), 
  // the mask(writable) = secret's length, holds '_' or revealed lettters
  // lives counter(writable), the new guess(char)
  // outputs: hit or miss

  let mut found: bool = false; // this is if the guess matched any letter in the secret word while we do a check

  for (i, ch) in secret_word.chars().enumerate() { // goes through all indexed characters
    if ch == guess {
      mask[i] = guess; 
      found = true; 
    }
  }
    // returns hit or miss depending on if the guess is correct
  if found {
    return Outcome::Hit(guess);
    } else {
      *lives -= 1;
      return Outcome::Miss(guess); 
    }
}

// this loops through all of the characters in the mask to see if there are any missing letters
fn is_solved(mask: &[char]) -> bool{
  for ch in mask {
    if *ch == '_' {
      return false;
    }
  }
  true
}

 fn winner_loser() -> 'static &str {
  if is_solved(&mask) == true {
    println!("
\\\                  // || ||\\\    || ||\\\    || ||======  ||=====\\\  ||
 \\\       __       //  || || \\\   || || \\\   || ||        ||     ||  ||
  \\\     //\\\     //   || ||  \\\  || ||  \\\  || ||====    ||====\\\   ||
   \\\   //  \\\   //    || ||   \\\ || ||   \\\ || ||        ||     \\\  ||
    \\\\=//    \\\\=//     || ||    \\\|| ||    \\\|| ||======= ||      \\\ ++"); 
  }else {
    println!("
||        ||======|| ========  ||======= ||=====\\\                  
||        ||      || ||        ||        ||      ||           
||        ||      || ||        ||        ||=====//               
||        ||      || ||=====|| ||====    ||     \\\             
||        ||      ||        || ||        ||      \\\             
||======= ||======|| =======|| ||======= ||       \\\ ");
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
    let word_bank: Vec<&str> = vec!["agonizing", "bizarre", "cauldron", "cemetery", "phantom", "enchanting", "frankenstein", "horror", "laboratory", "masquerade", "nefarious",
    "poltergeist", "revolting", "scarecrow", "sorcerer", "transylvania", "vengeance", "werewolf", "zombie"
];
    // render secret word + mask 
    let secret_word = word_bank.choose(&mut thread_rng()).unwrap();

    // max lives for user 
    const MAX_LIVES: u32  = 6; 
    let mut lives = MAX_LIVES; 

    // guessed letters is the vector that hold the users guesses 
    let mut guessed_letters: Vec<char> = vec![];
  
    // this dynamically uses the secret word provided to dynamically masks the word with the amount of letters from the secret_word bank
    let mut mask: Vec<char> = vec!['_'; secret_word.len()]; 
    println!("{}", render_mask(&mask)); 

    // each function that is being used is referencing the parameter 
    while (lives > 0) && (!is_solved(&mask)) {
        let stage_idx = MAX_LIVES - lives; 
        println!("{}", display_stage(stage_idx.try_into().unwrap())); 
        println!("{}", render_mask(&mask));

        println!("Lives: {}", lives); 
        println!("Enter a letter: "); 

        use std::io::Write;
        io::stdout().flush().unwrap(); // .flush(): flushes the buffer; ensure output is displayed immediately
        // .unwrap(): handles potential errors in the flush operation      
        let guess = read_guess(&guessed_letters);

        guessed_letters.push(guess); // holding the previous guesses within the vector guess_letters
        let outcome = apply_guess(secret_word, &mut mask, &mut lives, guess); 
        
        match outcome {
          Outcome::Hit(ch) => println!("Good guess!'{}' is in the spooky word.", ch), 
          Outcome::Miss(ch) => println!("Sorry, '{}' is not in the word", ch),
        }
        println!("{}", display_stage(stage_idx.try_into().unwrap())); 

        println!("Lives left: {}, Guessed Letters: {:?}", lives, guessed_letters); 

        println!("Secret Word : {}", secret_word);

    }

}