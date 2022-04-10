#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
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
    lookup: HashMap<String, u16>,
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

    fn unpack(&mut self, v: &str, tail: &mut Vec<String>) -> u16 {
        match v.parse::<u16>().ok() {
            Some(unpacked) => {
                print!("[Unpacking] Unpacked {v:?} as {unpacked}\t");
                unpacked
            }
            None => {
                print!("[Unpacking] Unable to unpack {v:?}\t");
                if tail.contains(&v.to_string()) {
                    panic!("Circle!")
                }

                tail.push(v.to_string());
                let res = self.resolve(v, tail);
                tail.pop();

                self.lookup.insert(v.to_string(), res);
                res
            }
        }
    }

    fn resolve(&mut self, variable: &str, tail: &mut Vec<String>) -> u16 {
        // print!("\n{tail:?} > Resolving var:{variable}");
        print!("\ntail:{}\t|[Resolving] '{variable}'\t", tail.len());

        let result;

        match variable.parse::<u16>().ok() {
            Some(v) => {
                print!("[Resolved] v:{variable} as {v}\t");
                self.lookup.insert(variable.to_string(), v);
                return v;
            }
            _ if self.lookup.contains_key(variable) => {
                let v = *self.lookup.get(variable).unwrap();
                print!("[Found] in the lookup! {variable}:{v}\t");
                return v;
            }
            _ => {
                result = self.circuit.get(variable).unwrap().clone();
                print!("[Unresolve] unpacking: {result:?}\t");
            }
        }

        match &result {
            Operation::AND(x, y) => self.unpack(x, tail) & self.unpack(y, tail),
            Operation::OR(x, y) => self.unpack(x, tail) | self.unpack(y, tail),
            Operation::LSHIFT(x, n) => self.unpack(x, tail) << self.unpack(n, tail),
            Operation::RSHIFT(x, n) => self.unpack(x, tail) >> self.unpack(n, tail),
            Operation::NOT(x) => !self.unpack(x, tail),
            Operation::ASSIGN(x) => {
                let x = self.unpack(x, tail);
                self.lookup.insert(variable.to_string(), x);
                x
            }
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
        CircuitSolver {
            circuit: HashMap::new(),
            lookup: HashMap::new(),
        }
    }
}

fn main() {
    let mut solver = CircuitSolver::new();

    let input = std::fs::read_to_string("input").unwrap();

    for line in input.lines() {
        solver.process_input(line);
    }

    let mut tail: Vec<String> = vec!["a".to_string()];

    let solution: u16 = solver.resolve("a", &mut tail);

    tail.pop();

    println!("\n\nSolution:: {solution}, tail: {tail:?}");
}
