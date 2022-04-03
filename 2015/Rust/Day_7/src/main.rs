#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

use std::collections::HashMap;

// #[derive(Debug, Clone, Copy)]
// enum Operation<'a> {
//     // x AND y -> z
//     AND(&'a str,&'a str),
//     // x OR y -> z
//     OR(&'a str,&'a str),
//     // x LSHIFT n -> y
//     LSHIFT(&'a str, &'a str),
//     // x RSHIFT n -> y
//     RSHIFT(&'a str, &'a str),
//     // NOT x -> f
//     NOT(&'a str),
//     // x -> y
//     ASSIGN(&'a str),
// }

// #[derive(Debug)]
// enum Operation {
//     // x AND y -> z
//     AND(str, str),
//     // x OR y -> z
//     OR(str,str),
//     // x LSHIFT n -> y
//     LSHIFT(str, str),
//     // x RSHIFT n -> y
//     RSHIFT(str, str),
//     // NOT x -> f
//     NOT(str),
//     // x -> y
//     ASSIGN(str),
// }

#[derive(Debug)]
enum Operation {
    // x AND y -> z
    AND(String, String),
    // x OR y -> z
    OR(String, String),
    // x LSHIFT n -> y
    LSHIFT(String, String),
    // x RSHIFT n -> y
    RSHIFT(String, String),
    // NOT x -> f
    NOT(String),
    // x -> y
    ASSIGN(String),
}

struct CircuitSolver {
    circuit: HashMap<String, Operation>,
    _lookup: HashMap<String, u16>,
}

impl CircuitSolver {
    fn process_input(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if line.contains("AND") {
            let x = parts[0].to_string();
            let y = parts[2].to_string();
            let z = parts[4].to_string();
            self.circuit.insert(z, Operation::AND(x, y));
        } else if line.contains("OR") {
            let x = parts[0].to_string();
            let y = parts[2].to_string();
            let z = parts[4].to_string();
            self.circuit.insert(z, Operation::OR(x, y));
        } else if line.contains("LSHIFT") {
            let x = parts[0].to_string();
            let n = parts[2].to_string();
            let z = parts[4].to_string();
            self.circuit.insert(z, Operation::LSHIFT(x, n));
        } else if line.contains("RSHIFT") {
            let x = parts[0].to_string();
            let n = parts[2].to_string();
            let z = parts[4].to_string();
            self.circuit.insert(z, Operation::RSHIFT(x, n));
        } else if line.contains("NOT") {
            let x = parts[1].to_string();
            let z = parts[3].to_string();
            self.circuit.insert(z, Operation::NOT(x));
        } else {
            let x = parts[0].to_string();
            let z = parts[2].to_string();
            self.circuit.insert(z, Operation::ASSIGN(x));
        }
    }

    fn update_lookup(&mut self, k: String, v: u16) {
        self._lookup.insert(k, v);
    }

    fn check_lookup(&mut self, k: String) -> Option<&u16> {
        self._lookup.get(&k)
    }

    fn get_circuit(&self, k: String) -> &Operation {
        self.circuit.get(&k).unwrap_or_else(|| panic!("Unable to find circuit point"))
    }

    fn solve(&mut self, start: &str) -> u16 {
        let start = self.get_circuit(start.to_string());

        if let Operation::ASSIGN(x) = start {
            return self.resolve(&x);
        };
        panic!("NO solution");
    }

    fn unpack(&mut self, v: &str) -> u16 {
        match v.parse::<u16>().ok() {
            Some(v) => v,
            None => self.resolve(v),
        }
    }
    fn resolve(&mut self, variable: &str) -> u16 {
        print!("\nResolving var:{variable}");
        let result;
        if let Ok(v) = variable.parse::<u16>() {
            println!(" >>> resolved var:{variable} as {v}");
            // self.lookup.insert(variable.to_string(), v);
            self.update_lookup(variable.to_string(), v);
            return v;
        }
        else {
            result = self.circuit.get(variable).unwrap();
            print!(" - unable to resolve, unpacking: {result:?}");
        }
        
        match result {
            Operation::AND(x, y) => self.unpack(x) & self.unpack(y),
            Operation::OR(x, y) => self.unpack(x) | self.unpack(y),
            Operation::LSHIFT(x, n) => self.unpack(x) << self.unpack(n),
            Operation::RSHIFT(x, n) => self.unpack(x) >> self.unpack(n),
            Operation::NOT(x) => !self.unpack(x),
            Operation::ASSIGN(x) => self.unpack(x), //BUG here, it's not assigning value properly
        }
    }

    fn _debug(&mut self) {
        //Debugging
        for (k, v) in &self.circuit {
            if k == "a"
                || match v {
                    Operation::AND(x, y) => x == "a" || y == "a",
                    Operation::OR(x, y) => x == "a" || y == "a",
                    Operation::LSHIFT(x, _) => x == "a",
                    Operation::RSHIFT(x, _) => x == "a",
                    Operation::NOT(x) => x == "a",
                    Operation::ASSIGN(x) => x == "a",
                }
            {
                println!("{k:#?} :: {v:?}");
            }

            if let Operation::ASSIGN(val) = v {
                let inp = val.parse::<u16>();

                if inp.is_ok() {
                    println!("{k:#?} :: {v:?}");
                }
            }
        }
    }

    pub(crate) fn new() -> Self {
        CircuitSolver { circuit: HashMap::new(), _lookup: HashMap::new() }
    }
}

fn main() {
    let mut solver = CircuitSolver::new();

    let input = std::fs::read_to_string("input").unwrap();

    for line in input.lines() {
        solver.process_input(line);
    }

    let solution: u16 = solver.solve("a");

    println!("Solution:: {solution}");
}
