use core::num;
use std::collections::hash_map;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, ErrorKind};

const INPUT_FILE: &str = "input";

fn read_file_to_string() -> String {
    let mut contents = String::new();

    let file = File::open(INPUT_FILE);
    let mut file = match file {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create(INPUT_FILE){
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

#[derive(Debug)]
#[derive(Clone)]
struct Garden_Destination_Map {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}

#[derive(Clone)]
struct GardenFarm {
    seeds: Vec<usize>,
    seed_to_soil: Vec<Garden_Destination_Map>,
    soil_to_fertilizer: Vec<Garden_Destination_Map>,
    fertilizer_to_water: Vec<Garden_Destination_Map>,
    water_to_light: Vec<Garden_Destination_Map>,
    light_to_temperature: Vec<Garden_Destination_Map>,
    temperature_to_humidity: Vec<Garden_Destination_Map>,
    humidity_to_location: Vec<Garden_Destination_Map>,
}

fn parse_data(input: String) -> GardenFarm {
    let mut garden_farm = GardenFarm {
        seeds: Vec::new(),
        seed_to_soil: Vec::new(),
        soil_to_fertilizer: Vec::new(),
        fertilizer_to_water: Vec::new(),
        water_to_light: Vec::new(),
        light_to_temperature: Vec::new(),
        temperature_to_humidity: Vec::new(),
        humidity_to_location: Vec::new(),
    };

    let mut converter_pos = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if (line.contains("seeds"))
        {
            line.replace("seeds:", "").split_ascii_whitespace()
                .for_each(|x| garden_farm.seeds.push(x.parse::<usize>().unwrap()));
            continue;
        }

        if (line.contains("seed-to-soil"))
        {
            converter_pos = 0;
            continue;
        }

        if (line.contains("soil-to-fertilizer"))
        {
            converter_pos = 1;
            continue;
        }

        if (line.contains("fertilizer-to-water"))
        {
            converter_pos = 2;
            continue;
        }

        if (line.contains("water-to-light"))
        {
            converter_pos = 3;
            continue;
        }

        if (line.contains("light-to-temperature"))
        {
            converter_pos = 4;
            continue;
        }

        if (line.contains("temperature-to-humidity"))
        {
            converter_pos = 5;
            continue;
        }

        if (line.contains("humidity-to-location"))
        {
            converter_pos = 6;
            continue;
        }

        let mut line_split = line.split_ascii_whitespace();
        let mut destination_start = line_split.next().unwrap().parse::<usize>().unwrap();
        let mut source_start = line_split.next().unwrap().parse::<usize>().unwrap();
        let mut range_length = line_split.next().unwrap().parse::<usize>().unwrap();

        let garden_destination = Garden_Destination_Map {
            destination_start: destination_start,
            source_start: source_start,
            range_length: range_length,
        };

        match converter_pos {
            0 => garden_farm.seed_to_soil.push(garden_destination),
            1 => garden_farm.soil_to_fertilizer.push(garden_destination),
            2 => garden_farm.fertilizer_to_water.push(garden_destination),
            3 => garden_farm.water_to_light.push(garden_destination),
            4 => garden_farm.light_to_temperature.push(garden_destination),
            5 => garden_farm.temperature_to_humidity.push(garden_destination),
            6 => garden_farm.humidity_to_location.push(garden_destination),
            _ => println!("Error: Unknown converter position"),
        }

    }

    /*println!("Seeds: {:?}", garden_farm.seeds);
    println!("Seed to Soil: {:?}", garden_farm.seed_to_soil);
    println!("Soil to Fertilizer: {:?}", garden_farm.soil_to_fertilizer);
    println!("Fertilizer to Water: {:?}", garden_farm.fertilizer_to_water);
    println!("Water to Light: {:?}", garden_farm.water_to_light);
    println!("Light to Temperature: {:?}", garden_farm.light_to_temperature);
    println!("Temperature to Humidity: {:?}", garden_farm.temperature_to_humidity);
    println!("Humidity to Location: {:?}", garden_farm.humidity_to_location);*/

    return garden_farm;
}


fn calculate_closest_location(garden_farm: GardenFarm) -> usize {
    let mut closest_location: usize = 99999999999999;

    for seed in garden_farm.seeds {
        let mut soil: usize = seed;
        
        for converter in &garden_farm.seed_to_soil {
            if (soil >= converter.source_start && soil < converter.source_start + converter.range_length) {
                soil = converter.destination_start + (soil - converter.source_start);
                break;
            }
        }

        let mut fertilizer:usize = soil;

        //println!("Seed: {}, Soil: {}", seed, soil);


        for converter in &garden_farm.soil_to_fertilizer {
            if (fertilizer >= converter.source_start && fertilizer < converter.source_start + converter.range_length) {
                fertilizer = converter.destination_start + (fertilizer - converter.source_start);
                break;
            }
        }

        let mut water: usize = fertilizer;
        //println!("Seed: {}, Soil: {}, Fertilizer: {}", seed, soil, fertilizer);

        for converter in &garden_farm.fertilizer_to_water {
            if (water >= converter.source_start && water < converter.source_start + converter.range_length) {
                water = converter.destination_start + (water - converter.source_start);
                break;
            }
        }

        let mut light: usize = water;

        //println!("Seed: {}, Soil: {}, Fertilizer: {}, Water: {}", seed, soil, fertilizer, water);

        for converter in &garden_farm.water_to_light {
            if (light >= converter.source_start && light < converter.source_start + converter.range_length) {
                light = converter.destination_start + (light - converter.source_start);
                break;
            }
        }

        let mut temperature: usize = light;

        //println!("Seed: {}, Soil: {}, Fertilizer: {}, Water: {}, Light: {}", seed, soil, fertilizer, water, light);

        for converter in &garden_farm.light_to_temperature {
            if (temperature >= converter.source_start && temperature < converter.source_start + converter.range_length) {
                temperature = converter.destination_start + (temperature - converter.source_start);
                break;
            }
        }

        let mut humidity: usize = temperature;

        //println!("Seed: {}, Soil: {}, Fertilizer: {}, Water: {}, Light: {}, Temperature: {}", seed, soil, fertilizer, water, light, temperature);

        for converter in &garden_farm.temperature_to_humidity {
            if (humidity >= converter.source_start && humidity < converter.source_start + converter.range_length) {
                humidity = converter.destination_start + (humidity - converter.source_start);
                break;
            }
        }

        let mut closest_location_temp: usize = humidity;


        //println!("Seed: {}, Soil: {}, Fertilizer: {}, Water: {}, Light: {}, Temperature: {}, Humidity: {}", seed, soil, fertilizer, water, light, temperature, humidity);

        for converter in &garden_farm.humidity_to_location {
            if (closest_location_temp >= converter.source_start && closest_location_temp < converter.source_start + converter.range_length) {
                closest_location_temp = converter.destination_start + (closest_location_temp - converter.source_start);
                break;
            }
        }
        
        if closest_location_temp == humidity {
            //closest_location_temp = 99999999999999;
        }

        if (closest_location_temp < closest_location) {
            closest_location = closest_location_temp;
        }

        //println!("Seed: {}, Soil: {}, Fertilizer: {}, Water: {}, Light: {}, Temperature: {}, Humidity: {}, Location: {}", seed, soil, fertilizer, water, light, temperature, humidity, closest_location_temp);
        //println!("Location: {}", closest_location_temp);
    }

    return closest_location;
}

fn convert_seed_ranges(seeds: Vec<usize>) -> Vec<usize> {
    let mut converted_seeds: Vec<usize> = Vec::new();

    for i in (0..seeds.len()).step_by(2) {
        for j in (seeds[i]..seeds[i]+seeds[i+1]) {
            converted_seeds.push(j);
        }
    }

    return converted_seeds;
}


fn calculate_closest_location_part2(mut garden_farm: GardenFarm) -> usize {
    let mut closest_location: usize = 99999999999999;
    println!("Start converting seeds");
    garden_farm.seeds = convert_seed_ranges(garden_farm.seeds.clone());
    println!("Finish converting seeds");
    closest_location = calculate_closest_location(garden_farm);


    return closest_location;
}

fn main() {
    let contents = read_file_to_string();
    let garden_farm = parse_data(contents);
    let closest_location = calculate_closest_location(garden_farm.clone());

    println!("Part 1 - Closest location: {}", closest_location);

    let closest_location_part_2 = calculate_closest_location_part2(garden_farm);
    println!("Part 2 - Closest location: {}", closest_location_part_2);
}
