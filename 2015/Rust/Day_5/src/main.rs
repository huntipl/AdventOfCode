use std::collections::HashMap;

fn is_naughty(line: &str) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    // Rule 3:
    for w in bad_strings {
        if line.contains(w) {
            return true
        }
    }

    let mut tmp_char: char = Default::default();
    let mut rule_2: bool = false;

    let mut v_c = 0;

    line.chars().for_each(|c| {
        if !rule_2 && tmp_char != c {
            tmp_char = c;
        }
        else if !rule_2 {
            rule_2 = true;
        }

        if vowels.contains(&c) {
            v_c += 1;
        }
    });

    if !rule_2 {return true};

    if v_c < 3 {return true};

    false
}

fn better_is_naughty(line: &str) -> bool {
    let mut rule_1 = false;

    for i in 0..line.chars().count()-3 {
        println!("{:?}: {:?}", i, line.chars().collect::<Vec<char>>()[i..i+2]);
    }
    panic!();


    true
}


fn main() {
    let data = std::fs::read_to_string("input").unwrap();

    let mut counter = 0;

    for line in data.lines() {
        if !better_is_naughty(line){
            counter += 1;
        }
    }
    println!("{counter}");
}
