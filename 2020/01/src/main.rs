use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {

    //Make new vector to hold values from file
    let mut values: Vec<u32> = Vec::new();

    match get_file_iterator_from_args(){
        Ok(lines) => {
            for line in lines {
                //Parse each line in the file from string to i32.
                let line: u32 = line.unwrap().parse::<u32>().unwrap();
                //Add it to the list of values.
                values.push(line);
            }
        },
        Err(err) => panic!("{}", err),
    };

    //part1(values)
    part2(values)

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

fn part1(values: Vec<u32>) {
    for outer_value in &values {
        for inner_value in &values {
            if inner_value + outer_value == 2020 {
                println!("Answer: {}", inner_value * outer_value);
                return;
            }
        }
    }
}

fn part2(values: Vec<u32>) {

    /*
    The closures take in a reference to their values because: ?
    */
    
    let result: Option<u32> = values.iter().enumerate().find_map(|(idx1, &val1)| {
        values.iter().enumerate().find_map(|(idx2, &val2)| {
            /*
            The inner find_map()'s closure is to call .find() on a third iterator.
            find() returns based on a predicate rather than an option.
            */
            match values.iter().enumerate().find(|(idx3, &val3)| { 
                /* Predicate is: 
                 - If all three values add to 2020
                 - If indexes are different (not implemented). 
                */
                val1 + val2 + val3 == 2020
            }) {
                /* 
                If the find() function returns a value, return all three values
                multiplied to the inner find_map() function.
                */
                Some((_, val3)) => Some(val1 * val2 * val3),
                /*
                Otherwise, return None, which will make the inner find_map()
                iterate to the next value.
                */
                None => None,
            }
        })
    });

    println!("Result is: {}", result.unwrap_or(0));
}