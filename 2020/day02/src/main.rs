extern crate regex;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use regex::{Regex, Captures};


fn main() {

    let password_format_regex: Regex = Regex::new(r"(^\d{1,2})-(\d{1,2}) ([a-z]): (.+$)").unwrap();
    let mut good_password_count: usize = 0;

    println!("Good passwords for part one: {}", part_one(&password_format_regex));
    println!("Good passwords for part two: {}", part_two(&password_format_regex));

}

fn part_one(pw_regex: &Regex) -> usize {

    let mut good_passwords: usize = 0;

    match get_file_iterator_from_args(){
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(password_full_string) => {
                        println!("Full password: {}", password_full_string);

                        let captures: Captures = pw_regex.captures(&password_full_string).unwrap();

                        let min_count: usize = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        let max_count: usize = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
                        let character = captures.get(3).unwrap().as_str();
                        let password = captures.get(4).unwrap().as_str();

                        println!("Password contains {} {} time(s).", character, password.matches(character).count());
                        println!("Less than max count: {}", password.matches(character).count() <= max_count);
                        println!("More than min count: {}", password.matches(character).count() >= min_count);

                        if password.matches(character).count() <= max_count && password.matches(character).count() >= min_count {
                            println!("Good password found.");
                            good_passwords += 1;
                        }

                    }
                    Err(no_password) => println!("No password found for this line. Skipping.")
                }
            }
        }
        Err(err) => { println!("{}", err); return 0; }
    }

    return good_passwords;

}

fn part_two(pw_regex: &Regex) -> usize {

    let mut good_passwords: usize = 0;

    match get_file_iterator_from_args(){
        Ok(lines) => {
            for line in lines {

                let line_str: String = line.unwrap();

                let captures: Captures = pw_regex.captures(&line_str).unwrap();
                let position_one: usize = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let position_two: usize = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let character: char = captures.get(3).unwrap().as_str().chars().next().expect("Couldn't find character");
                let password: &str = captures.get(4).unwrap().as_str();

                if (password.chars().nth(position_one).unwrap() == character) ^ (password.chars().nth(position_two).unwrap() == character) {
                    println!("BEEPBOOP");
                }

            }
        }
        Err(error) => { println!("{}", error); return 0;}
    }

    return 0;
}

fn get_file_iterator_from_args() -> Result<Lines<BufReader<File>>, Box<dyn std::error::Error>>{
    //Get file path from args
    let arg: String = env::args().nth(1).ok_or("Filename not provided in command line args.")?;
    //Create file from path
    let file: File = File::open(arg)?;
    //Create reader from file
    let reader = BufReader::new(file);
    //Get lines iterator from buffered reader.
    let lines = reader.lines();
    return Ok(lines);
}
