use std::collections::HashSet;
use std::io;
use std::io::Write;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    const TRIES:u32 = 5;
    let word: String;
    if args.len() > 1 {
        word = args[1].clone();
    } else {
        word = String::from("jazz");
    }
    

    let mut player_pov = word.clone();
    player_pov.replace_range(0 .. player_pov.len(), &"-".repeat(player_pov.len()));

    let mut guessed: HashSet<char> = HashSet::new();
    let mut wrong = 0;

    while wrong < TRIES {
        println!();
        println!("you have {} guesses remaining", TRIES - wrong);
        if take_guess(&word, &mut player_pov, &mut guessed, &mut wrong) {
            break;
        }
    }
    println!();
    println!("You won!!!!");
    println!("The word was: {}", word);
}

fn take_guess(answer: &str, pov: &mut String, guessed: &mut HashSet<char>, wrong: &mut u32) -> bool {
    println!();
    println!(); // some space
    println!("{}", pov);

    // take guess
    let mut input = Default::default();
    let guess: char;
    print!("Enter a Letter: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("failed to readline");
    input.pop(); // strip the newline
    input.pop(); // strip the carriage return

    guess = input.as_bytes()[0] as char;

    if input.len() > 1 { // the user guessed the entire word
        if input == answer {
            return true;
        } else {
            println!("{} is not the word", input);
            *wrong += 1;
            return false;
        }
    }

    if !guessed.contains(&guess) {
        if !answer.contains(guess) {
            *wrong += 1;
            guessed.insert(guess);
            return false;
        }

        let vec = find_all(&answer, guess);

        for index in vec {
            pov.replace_range(index .. index + 1, &String::from(answer.as_bytes()[index] as char));
        }
        guessed.insert(guess);
    } else {
        println!("You already guessed {} dummy!", guess);
        println!();
        return false;
    }

    return pov == answer;
}

fn find_all(original: &str, key: char) -> Vec<usize>{
    let mut text: String = original.to_string().clone();
    let mut indices: Vec<usize> = Vec::new();
    
    let mut index_offset: usize = 0;

    while text.len() > 0 {
       match text.find(key) {
        Some(x) => {
            indices.push(x + index_offset);
            text.drain(0 .. x+1);
            index_offset += (0 .. x+1).count();
        }
        None => {text = String::from("");}
       }
    }

    return indices;
}