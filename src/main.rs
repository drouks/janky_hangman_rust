#![allow(unused)]

use getch_rs::Getch;
use getch_rs::Key;
use rand::prelude::*;
use std::fs::read_to_string;
use std::fs::File;
use std::io::*;
use std::process;

fn main() {
    let mut rng = rand::thread_rng();
    let y: i32 = rng.gen_range(0..152481);
    let mut word: String = get_word_from_file("assets/wordlist.txt", y);
    let max_tries: u8 = 5;
    let mut current_try: u8 = 0;
    let mut found: Vec<bool> = Vec::new();
    let mut incorrect_choices: Vec<char> = Vec::new();
    
    for letter in word.chars() {
        found.push(false);
    }

    println!("########################################## WELCOME TO JANKY HANGMAN ###################################################");
    
    let accepted: &str = "abcdefgijklmnopqrstuvwxyz";

    let getter = Getch::new();
    loop {
        println!("#######################################################################################################################\n");
        print_found(word.clone(), found.as_mut_slice());
        println!("Enter a lower_case character or X(upper_case) to close:\n");
        
        match getter.getch() {
            Ok(Key::Char('X')) => {
                println!("YOU ENTERED: X");
                println!("Exiting...");
                break;
            },
            Ok(Key::Char(char)) => {
                println!("YOU ENTERED: {}",char);
                if (accepted.contains(char)) {
                    if (check_letter(char, word.clone())) {  //letter exists in the word
                        apply_found(char, word.clone(), found.as_mut_slice());
                        let mut false_exists = false;
                        for spot in found.as_slice() { //checking if all the letters are found
                            if (!spot) {
                                false_exists = true;
                            }
                        }

                        if (false_exists) { //there still are letters to be found
                            print_found(word.clone(), found.as_mut_slice());
                            print_incorrect_choices(incorrect_choices.as_mut_slice());
                        } else { //victory
                            print_found(word.clone(), found.as_mut_slice());
                            println!("You win!");
                            process::exit(0);
                        }
                    } else {
                        println!("The word doesn't contain the character {}", char);
                        
                        let mut letterfound = false;
                        for letter in incorrect_choices.as_slice(){
                            if(*letter == char){
                                println!("You have already tried the letter \"{}\". You get another shot",char);
                                letterfound = true;
                            }
                        }
                        if(letterfound == false){
                            current_try = current_try + 1;
                            incorrect_choices.push(char);
                        }
                        println!("Attempts left: {}",max_tries-current_try);
                        print_incorrect_choices(&mut incorrect_choices.as_mut_slice());

                        if (current_try == max_tries) {
                            println!("You lost! The word was: {}", word);
                            process::exit(0);
                        } 
                    }
                } else {
                    println!("Invalid input.");
                }
            }
            Ok(key) => {
                println!("Invalid input.")
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}

fn get_word_from_file(filename: &str, randint: i32) -> String {
    let mut counter: i32 = 0;
    let mut result = String::from("");
    for line in read_to_string(filename).unwrap().lines() {
        if (counter == randint) {
            result = line.to_string();
            break;
        } else {
            counter = counter + 1;
        }
    }

    return result;
}

fn check_letter(char: char, word: String) -> bool {
    if (word.contains(char)) {
        return true;
    }
    return false;
}

fn apply_found(char: char, word: String, vec: &mut [bool]) {
    for (i, letter) in word.chars().enumerate() {
        if (letter as char == char) {
            if (vec[i] == true) {
                println!("You have already entered the character \"{}\"", letter);
                return;
            } else {
                println!("You found a new character! \"{}\"", letter);
                vec[i] = true;
            }
        }
    }
    return;
}

fn print_found(word: String, vec: &mut [bool]) {
    for (i, letter) in word.chars().enumerate() {
        if (vec[i] == true) {
            print!("{}", letter);
        } else {
            print!("_");
        }
    }
    print!("\n");
    return;
}

fn print_incorrect_choices(vec: &mut [char]){
    print!("Incorrect characters: ");
    for letter in vec {
        print!("{} ",letter);
    }
    print!("\n");
    return;
}
