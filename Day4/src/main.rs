use core::num;
use std::collections::hash_map;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, ErrorKind};

#[derive(Clone)]
struct Card{
    id: i32,
    numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
    points : i32,
}

struct CardInstances{
    id: i32,
    instances: i32,
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

fn parse_data(data: String) -> Vec<Card> {
    let mut cards_list: Vec<Card> = Vec::new();
    let mut card_id = 0;

    for line in data.lines() {
        let mut card: Card = Card {
            id: 0,
            numbers: Vec::new(),
            winning_numbers: Vec::new(),
            points: 0,
        };

        // Set card id
        let mut line_split: Vec<&str> = line.split(":").collect();
        let card_id_str: String = line_split[0].to_string().replace("Card", "").replace(" ", "");
        card_id = card_id_str.parse::<i32>().unwrap();
        card.id = card_id;

        // Divide Numbers and Winning Numbers
        let mut numbers_split: Vec<&str> = line_split[1].split("|").collect();
        let mut card_numbers: Vec<&str> = numbers_split[0].split_ascii_whitespace().collect();
        let mut card_winning_numbers: Vec<&str> = numbers_split[1].split_ascii_whitespace().collect();

        // Convert Vec<&str> to Vec<i32>
        let card_numbers_i32: Vec<i32> = card_numbers.iter()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        let card_winning_numbers_i32: Vec<i32> = card_winning_numbers.iter()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        card.numbers = card_numbers_i32;
        card.winning_numbers = card_winning_numbers_i32;


        cards_list.push(card);
        //println!("Card {} Numbers: {:?}", card_id, card_numbers);
    }

    return cards_list;
}

fn calculate_points(card: Card) -> i32 {
    let mut points = 0;

    for number in card.numbers {
        if card.winning_numbers.contains(&number) {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
    }

    return points;
}

fn calculate_total_points(cards: Vec<Card>) -> i32 {
    let mut total_points = 0;

    for card in cards {
        total_points += calculate_points(card);
    }

    return total_points;
}

fn calculate_instances(card: Card) -> Vec<CardInstances> {
    let mut instances: Vec<CardInstances> = Vec::new();
    let mut points = 0;

    for number in card.numbers {
        if card.winning_numbers.contains(&number) {
            points += 1;

            let my_card_instance = CardInstances {
                id: card.id + points,
                instances: 1,
            };
            instances.push(my_card_instance);
        }
    }

    return instances;
}

fn calculate_number_of_instances(cards: Vec<Card>) -> HashMap<i32, i32>  {
    let mut cards_instances: hash_map::HashMap<i32, i32> = hash_map::HashMap::new();

    for card in cards {
        if cards_instances.contains_key(&card.id) {
            cards_instances.insert(card.id, cards_instances.get(&card.id).unwrap() + 1);
        } else {
            cards_instances.insert(card.id, 1);
        }

        let mut num_of_instances_my_card = *cards_instances.get(&card.id).unwrap();
        let mut num_of_instances = calculate_instances(card.clone());


        for card_instance in num_of_instances {

            if cards_instances.contains_key(&card_instance.id) {
                println!("Card: {} - Instances: {:}", card_instance.id, cards_instances.get(&card_instance.id).unwrap() + card_instance.instances * num_of_instances_my_card);
                cards_instances.insert(card_instance.id,  cards_instances.get(&card_instance.id).unwrap() + card_instance.instances * num_of_instances_my_card);
            } else {
                println!("Card: {} - Instances: {:}", card_instance.id, card_instance.instances * num_of_instances_my_card);
                cards_instances.insert(card_instance.id,  card_instance.instances * num_of_instances_my_card);
            }
        }
    }

    return cards_instances;
}

fn calculate_total_points_part2(cards: Vec<Card>) -> i32 {
    let mut instances_cards = calculate_number_of_instances(cards);
    let mut total_points = 0;

    for card in instances_cards {
        total_points += card.1;
    }

    return total_points;
}

fn main() {
    let contents = read_file_to_string();
    let parsed_data = parse_data(contents);
    let total_points = calculate_total_points(parsed_data.clone());
    let total_points_part2 = calculate_total_points_part2(parsed_data);

    println!("Part 1 - Total Points: {}", total_points);
    println!("Part 2 - Total Points: {}", total_points_part2 )

}