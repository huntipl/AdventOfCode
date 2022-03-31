#![allow(non_snake_case)]

#[derive(Debug)]
struct Coords {
    x: u32,
    y: u32
}

impl From<&str> for Coords{
    fn from(inp: &str) -> Self {
        let tmp: Vec<u32> = inp.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
        Coords { x: tmp[0], y: tmp[1] }
    }
}

enum Operation {
    Toggle,
    On,
    Off
}


fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut map = [[0; 1000]; 1000];

    for line in input.lines() {
        let commands: Vec<&str> = line.split_whitespace().collect();
        let start:Coords;
        let end:Coords;
        let operation: Operation;

        if line.starts_with("toggle ") {
            start = Coords::from(commands[1]);
            end = Coords::from(commands[3]);
            operation = Operation::Toggle;
        }
        else if line.starts_with("turn on ") {
            start = Coords::from(commands[2]);
            end = Coords::from(commands[4]);
            operation = Operation::On;
        }
        else if line.starts_with("turn off ") {
            start = Coords::from(commands[2]);
            end = Coords::from(commands[4]);
            operation = Operation::Off;
        }
        else {
            panic!();
        }

        // Part 1
        for i in start.x..=end.x {
            for j in start.y..=end.y {
                match operation {
                    Operation::Toggle => {
                        map[i as usize][j as usize] = if map[i as usize][j as usize] == 0 {1} else {0};
                    },
                    Operation::On => {
                        map[i as usize][j as usize] = 1;
                    },
                    Operation::Off => {
                        map[i as usize][j as usize] = 0;
                    },
                }
            }
        }

        // Part 2
        for i in start.x..=end.x {
            for j in start.y..=end.y {
                match operation {
                    Operation::Toggle => {
                        map[i as usize][j as usize] += 2;
                    },
                    Operation::On => {
                        map[i as usize][j as usize] += 1;
                    },
                    Operation::Off => {
                        if map[i as usize][j as usize] > 0 {
                            map[i as usize][j as usize] -= 1;
                        }
                    },
                }
            }
        }
    }
    let result = map
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .sum::<i32>();

    println!("{result}");
}
