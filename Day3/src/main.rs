use std::fs::File;
use std::io::{Read, ErrorKind};

#[derive(Clone)]
struct Item {
    symbol: String,
    x: i32,
    y: i32,
}

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
        Ok(_) => println!("File read successfully!"),
        Err(e) => panic!("Problem reading the file: {:?}", e),
    }

    return contents;
}

fn get_symbol_positions(data: String) -> Vec<Item> {
    let mut symbols: Vec<Item> = Vec::new();
    let mut y = 0;

    for line in data.lines(){
        let mut x = 0;

        for c in line.chars(){
            let mut item_value: i32 = c as i32;
                // Number Check
            if  item_value >= 48 && item_value <= 57 || item_value == 46 {
                x += 1;
                continue;
            }

            // here we have a symbol
            let mut item: Item = Item {
                symbol: c.to_string(),
                x: x,
                y: y,
            };

            //println!("Symbol:{} y:{} x:{} ", item.symbol, item.y, item.x);
            symbols.push(item);
            x += 1;

        }
        y += 1;

    }
    return symbols;
}

fn get_number_positions(data: String) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    let mut y = 0;

    for line in data.lines(){
        let mut my_item = Item {
            symbol: "".to_string(),
            x: 0,
            y: 0,
        };

        let mut x: i32 = 0;
        let mut number = "".to_string();

        for c in line.chars() {
            let mut item_value: i32 = c as i32;
            //println!("{} - {}", c, number);
            if item_value < 48 || item_value > 57 {
                if number.len() > 0 {
                    my_item = Item {
                        symbol: number.clone(),
                        x: x,
                        y: y,
                    };

                    items.push(my_item);
                    x += number.len() as i32;
                    number = "".to_string();
                }
                
                x += 1;
                continue;
            }
            
            number.push(c);
        }

        if number.len() > 0 {
            my_item = Item {
                symbol: number.clone(),
                x: x,
                y: y,
            };

            items.push(my_item);
        }

        y += 1;
    }
    return items;
}

fn check_data(symbol_found: Vec<Item>, numbers_found: Vec<Item>)
{
    let mut count:i64 = 0;
    for number in numbers_found{
        //println!("y:{}|x:{}={}", number.y, number.x, number.symbol);
        let symbol_found_copy = symbol_found.clone();
        if check_adjacent_symbol(symbol_found_copy, number.symbol.clone(), number.x,number.y) {
            count += number.symbol.parse::<i32>().unwrap() as i64;
        }
    }

    println!("Part 1: {}", count);
}

fn check_adjacent_symbol(symbol_found : Vec<Item>, number: String, x_number: i32, y_number: i32) -> bool{
    for symbol in symbol_found {
        if symbol.y + 1 >= y_number && symbol.y - 1 <= y_number {
            //println!("{} -- {} >= {} || {} <= {}", symbol.symbol, symbol.x, x_number - 1, symbol.x, x_number + number.len() as i32);
            if (symbol.x >= x_number - 1) && (symbol.x <= x_number + number.len() as i32) {
                //println!("Sy:{}|Sx:{} && Ny:{}|Nx:{} n:{} s:{}", symbol.y, symbol.x, y_number, x_number, number, symbol.symbol);
                //println!("x:{}|y:{}={}", symbol.x, symbol.y, symbol.symbol);
                return true;
            }
        }
    }
    return false;
}

fn check_adjacent_numbers(numbers_found : Vec<Item>, symbol: String, x_symbol: i32, y_symbol: i32) -> i32 {
    let mut count:i32 = 1;
    let mut symbol_found_num = 0;
    for number in numbers_found {
        if number.y + 1 >= y_symbol && number.y - 1 <= y_symbol {
            if x_symbol >= number.x - 1 && x_symbol <= number.x + number.symbol.len() as i32 {
                //println!("{} -- y:{} || x:{} || len: {}", number.symbol, number.y, number.x, number.x + number.symbol.len() as i32);

                symbol_found_num += 1;
                //println!("num:{} x {} = {}", number.symbol, count, number.symbol.parse::<i32>().unwrap() as i32 * count);
                count *= number.symbol.parse::<i32>().unwrap() as i32;
            }
            /*if (number.x <= x_symbol - 1) && (number.x + number.symbol.len() as i32 <= x_symbol)  ||
                (number.x >=  x_symbol + 1) && (number.x + number.symbol.len() as i32 >= x_symbol) {
                symbol_found_num += 1;
                println!("num:{} x {} = {}", number.symbol, count, number.symbol.parse::<i32>().unwrap() as i32 * count);
                count *= number.symbol.parse::<i32>().unwrap() as i32;
            }*/
        }
    }

    if symbol_found_num != 2 {
       return 0;
    }
    
    return count;
}

fn part_2(symbol_found: Vec<Item>, numbers_found: Vec<Item>)
{
    let mut count:i32 = 0;
    for symbol in symbol_found{
        if symbol.symbol != "*" {
            continue;
        }

        //println!("y:{}|x:{}={}", symbol.y, symbol.x, symbol.symbol);
        let numbers_found_copy = numbers_found.clone();
        count += check_adjacent_numbers(numbers_found_copy, symbol.symbol.clone(), symbol.x,symbol.y);

    }

    println!("Part 2: {}", count);
}


fn main() {
    let contents = read_file_to_string();
    let parsed_data = get_number_positions(contents.clone());
    let data = get_symbol_positions(contents.clone());
    check_data(data.clone(), parsed_data.clone());
    part_2(data, parsed_data);
}
