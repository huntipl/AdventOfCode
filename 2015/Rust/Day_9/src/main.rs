use std::collections::{HashMap, HashSet};

fn main() {
    let file = std::fs::read_to_string("test_input").unwrap();

    let mut map: HashMap<&str, HashMap<&str,u32>> = HashMap::default();
    let mut cities: HashSet<&str> = HashSet::default();

    for line in file.lines() {
        let city_a;
        let city_b;
        let dist: u32;

        let tmp: Vec<&str> =  line.split(" = ").collect();
        dist = tmp[1].parse().unwrap();

        let mut tmp: Vec<&str> = tmp[0].split(" to ").collect();
        tmp.sort_unstable();
        city_a = tmp[0];
        city_b = tmp[1];

        map.entry(city_a).or_insert_with(|| HashMap::from([(city_b, dist)])).insert(city_b, dist);
        map.entry(city_b).or_insert_with(|| HashMap::from([(city_a, dist)])).insert(city_a, dist);

        cities.insert(city_a);
        cities.insert(city_b);

        // println!("{city_a} - {city_b} : {dist}");
    }

    // for each city, calculate path through all cities, 
    // make sure all cities are visited
    // make sure no city is visited more than once
    let mut smallest_dist = u32::MAX;
    let total_cities: u32 = cities.len() as u32;

    for city in cities {
        let mut visited: Vec<&str> = Vec::new();
        let dist: u32 = find_path(city, &map, &mut visited, total_cities);
        if dist < smallest_dist {
            smallest_dist = dist;
        }
    }
    println!("{smallest_dist}");

    // println!("{map:#?}");
    // println!("{cities:#?}");
}


fn find_path<'a>(city: &'a str, map: &'a HashMap<&'a str, HashMap<&'a str, u32>>, visited: &mut Vec<&'a str>, total_cities: u32) -> u32 {
    visited.push(city);
    if visited.len() as u32 == total_cities {
        return 0;
    }

    // let local_shortest = u32::MAX;
    for (next_city, _) in map.get(city).unwrap() {
        if visited.contains(next_city) { //doesnt work
            continue;
        }
        let x = find_path(city, map, visited, total_cities);
    }
    1
}
