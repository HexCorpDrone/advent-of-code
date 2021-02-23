use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {

    let mut values: Vec<i32> = Vec::new();

    match get_iterator_from_args(){
        Ok(lines) => {
            for line in lines {
                //Parse each line in the file from string to i32.
                let line: i32 = line.unwrap().parse::<i32>().unwrap();
                //Add it to the list of values.
                values.push(line);
            }
        },
        Err(err) => panic!("{}", err),
    };

    part1(values)

}

fn get_iterator_from_args() -> Result<Lines<BufReader<File>>, Box<dyn std::error::Error>>{
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

fn part1(values: Vec<i32>) {
    for outer_value in &values {
        for inner_value in &values {
            if inner_value + outer_value == 2020 {
                println!("Answer: {}", inner_value * outer_value);
                return;
            }
        }
    }
}

fn part2(values: Vec<i32>) {

}