fn main() {
    // escapes:
    // \\ \" \x12

    let file = std::fs::read_to_string("input").unwrap();

    let mut character_counter = 0;
    let mut byte_counter = 0;

    let mut slash = 0;
    let mut hex = 0;
    let mut quote = 0;

    for l in file.lines() {
        let line_len = l.chars().count() as u32;
        let mut real_char_len: u32 = 0;

        let mut open: u8 = 0;

        for (i, c) in l.chars().enumerate() {
            if open > 0 {
                open -= 1;
                continue;
            }
            if i == 0 || i as u32 == line_len - 1 {
                continue;
            }
            if c == '\\' {
                // slash
                if l.chars().nth(i + 1).unwrap() == '\\' {
                    slash += 1;
                    real_char_len += 1;
                    open = 1;
                    continue;
                }
                // quote
                else if l.chars().nth(i + 1).unwrap() == '\"' {
                    println!("{l} >>> {}", l.get(i..).unwrap());
                    quote += 1;
                    real_char_len += 1;
                    open = 1;
                    continue;
                }
                // hex
                else if l.chars().nth(i + 1).unwrap() == 'x' {
                    let a: u32 = l.chars().nth(i + 2).unwrap().into();
                    let b: u32 = l.chars().nth(i + 3).unwrap().into();

                    if ((48..=57).contains(&a) || (97..=102).contains(&a))
                        && ((48..=57).contains(&b) || (97..=102).contains(&b))
                    {
                        hex += 1;
                        real_char_len += 1;
                        open = 3;
                        continue;
                    }
                }
            } else {
                real_char_len += 1;
            }
        }
        character_counter += line_len;
        byte_counter += real_char_len;
    }

    println!(
        "Result= {character_counter}-{byte_counter} = {}",
        character_counter - byte_counter
    );
    println!("Debug: total:{character_counter} hex:{hex}/124 quote:{quote}/248 slash:{slash}/113")
}
