extern crate regex;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use regex::{Regex, Captures};


fn main() {

    let password_format_regex: Regex = Regex::new(r"(^\d{1,2})-(\d{1,2}) ([a-z]): (.+$)").unwrap();
    let mut good_password_count: usize = 0;

    match part_one(password_format_regex) {
        Ok(pw_found) => println!("Good passwords: {}", pw_found),
        Err(err) => panic!(err),
    }

}

fn part_one(pw_format: Regex) -> Result<usize, String> {

    let mut good_passwords: usize = 0;

    match get_file_iterator_from_args(){
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(password_full_string) => {
                        println!("Full password: {}", password_full_string);

                        let captures: Captures = pw_format.captures(&password_full_string).unwrap();

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
        Err(err) => return Err("Could not get iterator of provided file.".to_string())
    }

    return Ok(good_passwords);

}

fn part_two() -> Result<usize, String> {

    return Ok(5890);
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
