fn main() {
    let data = std::fs::read_to_string("input").unwrap();
    let mut result = 0;
    let mut basement = 0;

    // Using enumerate just cause I need position of the iterator for part 2
    data.chars().enumerate().for_each(|(i, c)| {
        match c {
            '(' => result += 1,
            ')' => result -= 1,
            _ => panic!("Unexpected character!"),
        }
        // Part 2
        if basement == 0 && result < 0 {
            basement = i + 1
        }
    });

    // Part 1
    println!("Part1: Result: {result}");
    // Part 2
    println!("Part2: Basement at: {basement}");
}
