use std::fs::File;
use std::io::{Read, ErrorKind};

const NUMBERS: [&str; 9] = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn read_file_to_string() -> String {
    let mut contents = String::new();

    let file = File::open("input");
    let mut file = match file {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("input"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    match file.read_to_string(&mut contents){
        Ok(_) => println!("File contents: {}", contents),
        Err(e) => panic!("Problem reading the file: {:?}", e),
    }

    return contents;
}

fn part_one(){
    let mut contents = String::new();

    let file = File::open("input");
    let mut file = match file {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("input"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    let mut total: i32 = 0;
    match file.read_to_string(&mut contents){
        Ok(_) => {
            for line in contents.lines() {
                let mut total_line = 0;
                let mut first_char = -1;
                let mut last_char = -1;
                for word in line.chars() {
                    if word.is_numeric() {
                        println!("{}", word);
                        if first_char == -1 {
                            first_char = word.to_digit(10).unwrap() as i32;
                        }

                        last_char = word.to_digit(10).unwrap() as i32
                    }
                }

                total_line = first_char * 10 + last_char;
                total += total_line;
                println!("Total line: {}", total_line);

            }
            //println!("File contents: {}", contents)
        },
        Err(e) => panic!("Problem reading the file: {:?}", e),
    }

    println!("Total: {}", total);
}

fn part_two(){
    let mut contents = String::new();

    let file = File::open("input");
    let mut file = match file {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("input"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    let mut total: i32 = 0;
    match file.read_to_string(&mut contents){
        Ok(_) => {
            for line in contents.lines() {
                let mut total_line = 0;
                let mut first_char = -1;
                let mut first_char_pos = -1;
                let mut last_char = -1;
                let mut last_char_pos = -1;

                for word in line.chars(){
                    if word.is_numeric() {
                        if first_char == -1 {
                            first_char = word.to_digit(10).unwrap() as i32;
                            first_char_pos = line.find(word).unwrap() as i32;
                        }

                        last_char = word.to_digit(10).unwrap() as i32;
                        last_char_pos = line.rfind(word).unwrap() as i32;
                    }
                }

                for number in NUMBERS {
                    if line.contains(number) {
                        if ( line.find(number).unwrap() as i32 ) < first_char_pos || ( first_char_pos == -1  && (line.find(number).unwrap() as i32 ) != -1 ) {
                            first_char = NUMBERS.iter().position(|&s| s == number).unwrap()  as i32 + 1;
                            first_char_pos = line.find(number).unwrap() as i32;
                        }

                        if ( line.rfind(number).unwrap() as i32 ) > last_char_pos || ( last_char_pos == -1  && (line.rfind(number).unwrap() as i32 ) != -1 ){
                            last_char = NUMBERS.iter().position(|&s| s == number).unwrap() as i32 + 1;
                            last_char_pos = line.rfind(number).unwrap() as i32;
                        }
                    }
                }

                if last_char == -1 {
                    last_char = first_char;
                }

                total_line = first_char * 10 + last_char;
                total += total_line;
                println!("Total line: {}", total_line);

            }
            //println!("File contents: {}", contents)
        },
        Err(e) => panic!("Problem reading the file: {:?}", e),
    }

    println!("Total: {}", total);
}

fn main() {
    //part_one();

    //let myString = read_file_to_string();
    //println!("{}", myString);
    part_two();
}
