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
    let mut rule_2 = false;

    for i in 0..line.chars().count() {
        
        if !rule_1 && i < line.len()-3 {
            let pair = &line[i..i+2];
            if line[i+2..].contains(pair) {
                rule_1 = true;
            }
        }
        if !rule_2 && i < line.len()-2 && line.chars().nth(i).unwrap() == line.chars().nth(i+2).unwrap() {
            rule_2 = true;
        }

        if rule_1 && rule_2 {
            return false;
        }
    }
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
    println!("{counter} of {}", data.lines().count());
}
