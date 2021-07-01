use std::io;
use std::io::prelude::*;

fn main() {
    print!("Enter with the magic word: ");

    io::stdout().flush()
        .unwrap();

    let mut magic_word = String::new();
    let mut progress = String::new();
    let mut log = progress;
    let mut input2cmp = String::new();
    let mut lifes: u8 = 8;

    io::stdin().read_line(&mut magic_word)
        .expect("Error reading user input");

    magic_word.pop();
    progress = str::repeat("*", magic_word.len());

    while progress != magic_word {
        update_screen(&lifes, &progress, &log);

        io::stdin().read_line(&mut input2cmp)
            .expect("Error reading user input");

        check_string(&magic_word, &mut progress, &mut lifes, &mut log, input2cmp.chars().collect::<Vec<char>>()[input2cmp.len() -2]);

        if lifes <= 0 {
            println!("You lose!");
            return;
        }
    }

    update_screen(&lifes, &progress, &log);

    println!("You win!");
}

fn update_screen(lifes: &u8, progress: &String, log: &String) {
    print!("\x1B[2J\x1B[1;1H");
    print!("Lifes: {}\n\nProgress: {}\n\n{}", lifes, progress, log);
    print!("Next guess: ");
    io::stdout().flush().unwrap();
}

fn check_string(magic_word: &String, progress: &mut String, lifes: &mut u8, log: &mut String, char2cmp: char) {
    let magic_word_vec: Vec<char> = magic_word.chars().collect();
    let mut has_char = false;
    for i in 0..magic_word_vec.len() {
        if magic_word_vec[i] == char2cmp {
            progress.replace_range(i..i+1, magic_word_vec[i].to_string().as_str());
            has_char = true;
            *log = String::from("");
        }
    }

    if !has_char {
        *lifes -= 1;
        *log = String::from(format!("No '{}' character on magic-word\n\n", char2cmp));
    }
}
