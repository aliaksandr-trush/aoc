#[derive(Debug, Clone)]
enum Entity {
    Seed,
    Soil,
    Fertilizaer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Entity {
    fn from(value: &str) -> Option<Self> {
        match value {
            "seed" => Some(Entity::Seed),
            "soil" => Some(Entity::Soil),
            "fertilizaer" => Some(Entity::Fertilizaer),
            "water" => Some(Entity::Water),
            "light" => Some(Entity::Light),
            "temperature" => Some(Entity::Temperature),
            "humidity" => Some(Entity::Humidity),
            "location" => Some(Entity::Location),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    src_start: u32,
    dst_start: u32,
    lenght: u32,
}

impl Path {
    fn from(numbers: &Vec<u32>) -> Path {
        let mut numbers_iter = numbers.iter();
        let src_start = *numbers_iter.next().unwrap();
        let dst_start = *numbers_iter.next().unwrap();
        let lenght = *numbers_iter.next().unwrap();
        Path {
            src_start,
            dst_start,
            lenght,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    src: Option<Entity>,
    dst: Option<Entity>,
    pathes: Vec<Path>,
}

impl Map {
    fn new() -> Self {
        Map {
            src: None,
            dst: None,
            pathes: Vec::new(),
        }
    }
}

fn parse_input(input_str: &str) -> (Vec<&str>, Vec<Map>) {}
fn find_way(seeds_list: &Vec<u32>, maps: &Vec<Map>) -> u32 {}

pub fn proceed_data(input_data: &str) -> u32 {
    let mut seeds_list: Vec<&str> = Vec::new();
    let mut temp_map: Map = Map::new();
    let mut maps_list: Vec<Map> = Vec::new();
    for line in input_data.lines() {
        if line.is_empty() {
            if temp_map.src.is_some() {
                maps_list.push(temp_map.clone())
            }
        } else if line.starts_with("seeds:") {
            seeds_list = line.split_once(":").unwrap().1.trim().split(" ").collect();
        } else if line.ends_with("map:") {
            let mut temp = line.split_once(" ").unwrap().0.split("-");
            let src: &str = temp.next().unwrap();
            temp_map.src = Entity::from(src);
            temp.next();
            let dst: &str = temp.next().unwrap();
            temp_map.dst = Entity::from(dst);
        } else {
            let numbers: Vec<u32> = line.split(" ").map(|i| i.parse().unwrap()).collect();
            temp_map.pathes.push(Path::from(&numbers));
        }
    }
    35
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let input_data = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let res = proceed_data(input_data);
        assert_eq!(res, 35);
    }
}
