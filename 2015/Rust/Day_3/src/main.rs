use std::collections::HashMap;

enum Turn {
    Santa,
    Robo,
}

struct WorldMap {
    santa_x: i32,
    santa_y: i32,
    robo_x: i32,
    robo_y: i32,
    visited: HashMap<(i32, i32), i32>,
    turn: Turn,
    robo_santa_enabled: bool,
}

impl WorldMap {
    fn pos(&self) -> (i32, i32) {
        match self.turn {
            Turn::Santa => (self.santa_x, self.santa_y),
            Turn::Robo => (self.robo_x, self.robo_y),
        }
    }
    fn next_turn(&mut self) {
        if self.robo_santa_enabled{
            self.turn = match self.turn {
                Turn::Santa => Turn::Robo,
                Turn::Robo => Turn::Santa,
            }
        }
    }
    fn up(&mut self) {
        match self.turn {
            Turn::Santa => {
                self.santa_y += 1;
            }
            Turn::Robo => {
                self.robo_y += 1;
            }
        }
        *self.visited.entry(self.pos()).or_insert(1) += 1;
        self.next_turn();
    }
    fn down(&mut self) {
        match self.turn {
            Turn::Santa => {
                self.santa_y -= 1;
            }
            Turn::Robo => {
                self.robo_y -= 1;
            }
        }
        *self.visited.entry(self.pos()).or_insert(1) += 1;
        self.next_turn();
    }
    fn left(&mut self) {
        match self.turn {
            Turn::Santa => {
                self.santa_x -= 1;
            }
            Turn::Robo => {
                self.robo_x -= 1;
            }
        }
        *self.visited.entry(self.pos()).or_insert(1) += 1;
        self.next_turn();
    }
    fn right(&mut self) {
        match self.turn {
            Turn::Santa => {
                self.santa_x += 1;
            }
            Turn::Robo => {
                self.robo_x += 1;
            }
        }
        *self.visited.entry(self.pos()).or_insert(1) += 1;
        self.next_turn();
    }
    fn count(&self) -> i32 {
        self.visited.keys().count() as i32
    }
    fn new() -> WorldMap {
        WorldMap {
            santa_x: 0,
            santa_y: 0,
            robo_x: 0,
            robo_y: 0,
            visited: HashMap::from([((0, 0), 1)]),
            turn: Turn::Santa,
            robo_santa_enabled: false,
        }
    }
    pub fn enable_robo_santa(&mut self) {
        self.robo_santa_enabled = true;
    }
}

fn main() {
    let data = std::fs::read_to_string("input").unwrap();

    let mut world_map = WorldMap::new();
    // Task2
    // world_map.enable_robo_santa();

    data.chars().for_each(|c| match c {
        '^' => world_map.up(),
        '>' => world_map.right(),
        'v' => world_map.down(),
        '<' => world_map.left(),
        _ => panic!("Invalid input"),
    });
    
    println!("{:#?}", world_map.count());
}
