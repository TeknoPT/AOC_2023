use std::fs::File;
use std::io::{Read, ErrorKind};
struct RoundData {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

struct GameData {
    id: i32,
    rounds: Vec<RoundData>,
}

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

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

fn parse_data(contents: String) -> Vec<GameData> {
    let mut data: Vec<GameData> = Vec::new();
    for line in contents.lines() {
        let mut game_data = GameData {
            id: 0,
            rounds: Vec::new(),
        };
        
        let mut round_data_vec: Vec<RoundData> = Vec::new();
        let mut round_data_vec_index = 0;
        let mut data_split = line.split(":").collect::<Vec<&str>>();
        let mut game_id = data_split[0];
        game_data.id = game_id.replace("Game ", "").parse::<i32>().unwrap();

        for word in data_split[1].split(";") {
            let mut round_data_split = word.split(",").collect::<Vec<&str>>();
            let mut round_data = RoundData {
                id: round_data_vec_index,
                red: 0,
                green: 0,
                blue: 0,
            };

            for color_text in round_data_split {
                let mut color_split = color_text.split_ascii_whitespace().collect::<Vec<&str>>();
                let mut color_value = color_split[0];
                let mut color = color_split[1];
                if color == "red" {
                    round_data.red = color_value.parse::<i32>().unwrap();
                } else if color == "green" {
                    round_data.green = color_value.parse::<i32>().unwrap();
                } else if color == "blue" {
                    round_data.blue = color_value.parse::<i32>().unwrap();
                }

            }

            round_data_vec.push(round_data);
            round_data_vec_index += 1;
        }

        game_data.rounds = round_data_vec;
        data.push(game_data);
    }

    return data;
}

fn check_game_data_part1(data: &Vec<GameData>) -> i32 {
    let mut sum_of_valid_game_ids = 0;
    for game in data {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut is_game_valid = true;
        for round in &game.rounds {
            if round.red > MAX_RED || round.green > MAX_GREEN || round.blue > MAX_BLUE {
                is_game_valid = false;
                break;
            }

            red += round.red;
            green += round.green;
            blue += round.blue;
        }

        if is_game_valid {
            sum_of_valid_game_ids += game.id;
        }
    }

    return sum_of_valid_game_ids;
}

fn check_game_data_part2(data: &Vec<GameData>) -> i32 {
    let mut sum_games = 0;
    for game in data {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for round in &game.rounds {
            if round.red > red {
                red = round.red;
            }

            if round.green > green {
                green = round.green;
            }

            if round.blue > blue {
                blue = round.blue;
            }

        }

        if red == 0 {
            red = 1;
        }

        if green == 0 {
            green = 1;
        }

        if blue == 0 {
            blue = 1;
        }

        sum_games += red * green * blue;
    }

    return sum_games;
}

fn main() {
    let contents = read_file_to_string();
    let data = parse_data(contents);
    let sum_of_valid_game_ids = check_game_data_part1(&data);
    let sum_of_games_cubes = check_game_data_part2(&data);
    println!("Sum of valid game ids: {}", sum_of_valid_game_ids);
    println!("Sum of games cubes: {}", sum_of_games_cubes);
}
