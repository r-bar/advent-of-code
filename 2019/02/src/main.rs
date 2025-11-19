use std::convert::TryFrom;
use std::error::Error;
use std::io::prelude::*;
use std::{env, fs, io, iter};

type BoxResult<T> = Result<T, Box<dyn Error>>;
type Program = Vec<i32>;
type OpCode = (i32, i32, i32, i32);

#[derive(Debug)]
struct Intcomp {
    memory: Program,
    pointer: usize,
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add { a: i32, b: i32, store: usize },
    Mul { a: i32, b: i32, store: usize },
    Exit,
}

impl TryFrom<OpCode> for Operation {
    type Error = &'static str;

    fn try_from(opcode: OpCode) -> Result<Self, Self::Error> {
        match opcode {
            (1, a, b, store) => Ok(Operation::Add {
                a: a,
                b: b,
                store: store as usize,
            }),
            (2, a, b, store) => Ok(Operation::Mul {
                a: a,
                b: b,
                store: store as usize,
            }),
            (99, _, _, _) => Ok(Operation::Exit),
            _ => Err("invalid opcode"),
        }
    }
}

impl Intcomp {
    fn new(program: Program) -> Self {
        Intcomp {
            memory: program,
            pointer: 0,
        }
    }

    fn set(&mut self, address: usize, val: i32) -> Result<(), &'static str> {
        if self.memory.len() <= address {
            return Err("value out of bounds");
        }
        self.memory[address] = val;
        Ok(())
    }

    fn get_operation(&self, address: usize) -> Result<Operation, &'static str> {
        let op = match self.memory.get(address) {
            None => return Err("address out of bounds"),
            // early return on exit code to avoid parameter lookup.
            // exit code may be last value in ram
            Some(99) => return Ok(Operation::Exit),
            Some(n) => n,
        };
        let a = self
            .memory
            .get(address + 1)
            .ok_or("a value out of bounds")?;
        let b = self
            .memory
            .get(address + 2)
            .ok_or("b value out of bounds")?;
        let store = self.memory.get(address + 3).ok_or("store value ")?;
        match op {
            1 => Ok(Operation::Add {
                a: *a,
                b: *b,
                store: *store as usize,
            }),
            2 => Ok(Operation::Mul {
                a: *a,
                b: *b,
                store: *store as usize,
            }),
            _ => Err("unknown op code"),
        }
    }

    fn run(&mut self) -> BoxResult<()> {
        loop {
            let operation = self.get_operation(self.pointer)?;
            let exit = self.perform(operation)?;
            if exit {
                return Ok(());
            };
        }
    }

    /// Performs the operation on the current computer state. Returns a result with a boolean value
    /// indicating whether the program should exit after the operation.
    fn perform(&mut self, operation: Operation) -> BoxResult<bool> {
        match operation {
            Operation::Add { a, b, store } => {
                let aval = self.memory.get(a as usize).ok_or("a value out of bounds")?;
                let bval = self.memory.get(b as usize).ok_or("b value out of bounds")?;
                self.memory[store] = aval + bval;
                self.pointer += 4;
                Ok(false)
            }
            Operation::Mul { a, b, store } => {
                let aval = self.memory.get(a as usize).ok_or("a value out of bounds")?;
                let bval = self.memory.get(b as usize).ok_or("b value out of bounds")?;
                self.memory[store] = aval * bval;
                self.pointer += 4;
                Ok(false)
            }
            Operation::Exit => Ok(true),
        }
    }
}

fn solver(program: &Program, target: i32) -> Option<(i32, i32)> {
    let search_range: i32 = 100;
    (0..search_range)
        .flat_map(|n| (0..search_range).zip(iter::repeat(n)))
        .find(|(noun, verb)| {
            let mut comp = Intcomp::new(program.clone());
            comp.memory[1] = *noun;
            comp.memory[2] = *verb;
            match comp.run() {
                Ok(_) => {
                    let result = comp.memory[0];
                    result == target
                }
                Err(_) => false,
            }
        })
}

fn load_program_file(path: &str) -> Program {
    let f = fs::File::open(path).expect("unable to open program file");
    let reader = io::BufReader::new(f);
    reader
        .split(b',')
        .filter_map(Result::ok)
        .map(String::from_utf8)
        .filter_map(Result::ok)
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .collect()
}

fn main() {
    println!("loading program...");
    let path = env::args().nth(1).expect("program file required");
    let program = load_program_file(&path);
    println!("program: {:?}\n\n", program);
    let mut comp = Intcomp::new(program.clone());

    println!("restoring 1202 alarm state...");
    comp.memory[1] = 12;
    comp.memory[2] = 2;

    println!("running program...");
    comp.run().unwrap_or_else(|err| {
        println!("{}", err);
        println!("runtime error{:?}", comp);
        panic!();
    });

    println!("result: {:?}", comp.memory);
    println!("0 value: {}", comp.memory[0]);

    let target = 19690720;
    println!("\n\nfinding combination for value {}...", target);

    match solver(&program, target) {
        Some((noun, verb)) => {
            let code = (100 * noun) + verb;
            println!("noun={} verb={} code={}", noun, verb, code);
        }
        None => println!("no solution found"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_program_and_check_result(program: Program, expected: Program) {
        let mut comp = Intcomp::new(program);
        comp.run().unwrap_or_else(|err| {
            println!("{}", err);
            println!("runtime error{:?}", comp);
            panic!();
        });
        println!("result   = {:?}", &comp.memory);
        println!("expected = {:?}", &expected);
        assert_eq!(comp.memory, expected);
    }

    #[test]
    fn test_intcomp_1() {
        run_program_and_check_result(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_intcomp_2() {
        run_program_and_check_result(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_intcomp_3() {
        run_program_and_check_result(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_intcomp_4() {
        run_program_and_check_result(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    #[test]
    fn test_intcomp_5() {
        run_program_and_check_result(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
    }

    #[test]
    fn test_solver() {
        let program = load_program_file("input.txt");
        let (noun, verb) = solver(&program, 9581917).unwrap();
        assert_eq!((noun, verb), (12, 2));
    }
}
