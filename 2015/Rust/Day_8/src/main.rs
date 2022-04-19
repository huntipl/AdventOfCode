fn main() {
    println!("Hello, world!");
    // escapes:
    // \\ \" \x12

    let file = std::fs::read_to_string("input").unwrap();

    let mut character_counter = 0;
    let mut byte_counter = 0;

    let mut total_hex = 0;
    let mut total_quote = 0;
    let mut total_slash = 0;
    let mut total_surrounding_quotes = 0;

    for line in file.lines() {
        let tmp_c = line.chars().count() as u32;
        let tmp_stripped_line = &line[1..line.len()-1];

        let slash = tmp_stripped_line.matches("\\\\").count() as u32;
        let quote = tmp_stripped_line.matches("\\\"").count() as u32;
        let hex = tmp_stripped_line.matches("\\x").count() as u32;

        total_hex += hex;
        total_quote += quote;
        total_slash += slash;
        total_surrounding_quotes += 2;

        let tmp_b = tmp_c - 2 - (slash) - (quote) - (hex*3);

        println!("{tmp_stripped_line} {tmp_c} {tmp_b} | \\{slash} \\\"{quote} \\x{hex}");
        character_counter += tmp_c;
        byte_counter += tmp_b;
    }
    println!("Result= {character_counter}-{byte_counter} = {}", character_counter-byte_counter);
    println!("Debug: hex:{total_hex}/124 quote:{total_quote}/255 slash:{total_slash}/113 sur:{total_surrounding_quotes}")
}