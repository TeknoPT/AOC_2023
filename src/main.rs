use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn read_file(){
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

fn main() {
    read_file();
}
