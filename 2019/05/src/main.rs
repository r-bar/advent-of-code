use std::convert::TryFrom;
use std::error::Error;
use std::io::prelude::*;
use std::{env, fs, io};

type BoxResult<T> = Result<T, Box<dyn Error>>;
type Program<'a> = &'a mut [i32];

#[derive(Debug)]
struct Intcomp<'a, R, W>
where
    R: BufRead + std::fmt::Debug,
    W: Write + std::fmt::Debug,
{
    memory: Program<'a>,
    pointer: usize,
    stdin: R,
    stdout: W,
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add {
        a: i32,
        b: i32,
        save: usize,
        a_mode: Mode,
        b_mode: Mode,
    },
    Mul {
        a: i32,
        b: i32,
        save: usize,
        a_mode: Mode,
        b_mode: Mode,
    },
    In {
        save: usize,
    },
    Out {
        read: usize,
        read_mode: Mode,
    },
    JumpIfTrue {
        check: i32,
        check_mode: Mode,
        address: usize,
        address_mode: Mode,
    },
    JumpIfFalse {
        check: i32,
        address: usize,
        check_mode: Mode,
        address_mode: Mode,
    },
    LessThan {
        a: i32,
        b: i32,
        save: usize,
        a_mode: Mode,
        b_mode: Mode,
    },
    Eql {
        a: i32,
        b: i32,
        save: usize,
        a_mode: Mode,
        b_mode: Mode,
    },
    Exit,
}

#[derive(Debug, PartialEq)]
enum Mode {
    Positional,
    Immeadiate,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Positional
    }
}

impl From<i32> for Mode {
    fn from(value: i32) -> Self {
        match value {
            1 => Mode::Immeadiate,
            _ => Mode::default(),
        }
    }
}

impl From<&i32> for Mode {
    fn from(value: &i32) -> Self {
        Mode::from(*value)
    }
}

//impl <T: Into<Mode>> From<Option<T>> for Mode {
//    fn from(value: Option<T>) -> Self {
//        value.map(|v| v.into()).unwrap_or(Mode::default())
//    }
//}

fn digits(number: i32) -> Vec<i32> {
    let mut divisor = 1;
    while number >= divisor * 10 {
        divisor *= 10;
    }

    let mut digits = Vec::new();
    let mut n = number;
    while divisor > 0 {
        digits.push(n / divisor);
        n %= divisor;
        divisor /= 10;
    }
    digits
}

impl TryFrom<&[i32]> for Operation {
    type Error = String;

    fn try_from(opcode: &[i32]) -> Result<Self, Self::Error> {
        let operation_code: i32 = *opcode.get(0).ok_or("empty opcode")?;
        let (operation, modes) = Operation::parse_operation_modes(operation_code);
        match (operation, opcode) {
            (1, [_raw_op, a, b, save]) => Ok(Operation::Add {
                a: *a,
                b: *b,
                a_mode: modes.get(0).map(Mode::from).unwrap_or_default(),
                b_mode: modes.get(1).map(Mode::from).unwrap_or_default(),
                save: *save as usize,
            }),
            (2, [_raw_op, a, b, save]) => Ok(Operation::Mul {
                a: *a,
                b: *b,
                a_mode: modes.get(0).map(Mode::from).unwrap_or_default(),
                b_mode: modes.get(1).map(Mode::from).unwrap_or_default(),
                save: *save as usize,
            }),
            (3, [_raw_op, save]) => Ok(Operation::In {
                save: *save as usize,
            }),
            (4, [_raw_op, read]) => Ok(Operation::Out {
                read: *read as usize,
                read_mode: modes.get(0).map(Mode::from).unwrap_or_default(),
            }),
            (5, [_raw_op, check, address]) => Ok(Operation::JumpIfTrue {
                check: *check,
                address: *address as usize,
                check_mode: modes.get(0).map(Mode::from).unwrap_or_default(),
                address_mode: modes.get(1).map(Mode::from).unwrap_or_default(),
            }),
            (6, [_raw_op, check, address]) => Ok(Operation::JumpIfFalse {
                check: *check,
                address: *address as usize,
                check_mode: modes.get(0).map(Mode::from).unwrap_or_default(),
                address_mode: modes.get(1).map(Mode::from).unwrap_or_default(),
            }),
            (7, [_raw_op, a, b, save]) => Ok(Operation::LessThan {
                a: *a,
                b: *b,
                a_mode: modes.get(0).map(Mode::from).unwrap_or_default(),
                b_mode: modes.get(1).map(Mode::from).unwrap_or_default(),
                save: *save as usize,
            }),
            (8, [_raw_op, a, b, save]) => Ok(Operation::Eql {
                a: *a,
                b: *b,
                a_mode: modes.get(0).map(Mode::from).unwrap_or_default(),
                b_mode: modes.get(1).map(Mode::from).unwrap_or_default(),
                save: *save as usize,
            }),
            (99, _) => Ok(Operation::Exit),
            _ => Err("invalid opcode".into()),
        }
    }
}

impl Operation {
    fn parse_operation_modes(code: i32) -> (i32, Vec<i32>) {
        let digits = digits(code);
        match digits.len() {
            1 => (digits[0], vec![]),
            2 => (digits[0] * 10 + digits[1], vec![]),
            n => (
                digits[n - 2] * 10 + digits[n - 1],
                // digits are reversed to preserve flag indexes when the operation has multiple
                // parameters
                digits[..n - 2].iter().rev().map(|i| *i).collect(),
            ),
        }
    }

    fn size_by_code(code: i32) -> Result<usize, String> {
        let (op, _modes) = Operation::parse_operation_modes(code);
        match op {
            1 => Ok(4),
            2 => Ok(4),
            3 => Ok(2),
            4 => Ok(2),
            5 => Ok(3),
            6 => Ok(3),
            7 => Ok(4),
            8 => Ok(4),
            99 => Ok(1),
            _ => Err(format!("invalid code: {}", code)),
        }
    }

    fn size(&self) -> usize {
        #[allow(unused_variables)]
        match self {
            Self::Add {
                a,
                b,
                a_mode,
                b_mode,
                save,
            } => 4,
            Self::Mul {
                a,
                b,
                a_mode,
                b_mode,
                save,
            } => 4,
            Self::In { save } => 2,
            Self::Out { read, read_mode } => 2,
            Self::JumpIfTrue {
                check,
                address,
                check_mode,
                address_mode,
            } => 3,
            Self::JumpIfFalse {
                check,
                address,
                check_mode,
                address_mode,
            } => 3,
            Self::LessThan {
                a,
                b,
                a_mode,
                b_mode,
                save,
            } => 4,
            Self::Eql {
                a,
                b,
                a_mode,
                b_mode,
                save,
            } => 4,
            Self::Exit => 1,
        }
    }
}

impl<'a, R, W> Intcomp<'a, R, W>
where
    R: BufRead + std::fmt::Debug,
    W: Write + std::fmt::Debug,
{
    ///// Initializes a new integer computer with stdin and stdout tied to the system stdin and
    ///// stdout.
    //fn new<A: BufRead, B: Write>(program: Program) -> Self {
    //    let stdin: Box<dyn BufRead> = Box::new(io::stdin().lock());
    //    let stdout: Box<dyn Write> = Box::new(io::stdout());
    //    Intcomp {
    //        memory: program,
    //        pointer: 0,
    //        //stdin: stdin,
    //        //stdout: stdout,
    //        stdin: io::stdin().lock(),
    //        stdout: io::stdout(),
    //    }
    //}

    /// Initializes a new integer computer with specific stdin and stdout. Useful for testing.
    fn new_with_io(stdin: R, stdout: W, program: Program<'a>) -> Self {
        Intcomp {
            memory: program,
            pointer: 0,
            stdin: stdin,
            stdout: stdout,
        }
    }

    fn set(&mut self, address: usize, val: i32) -> Result<(), String> {
        if self.memory.len() <= address {
            return Err("value out of bounds".into());
        }
        self.memory[address] = val;
        Ok(())
    }

    fn get(&self, val: i32, mode: Mode) -> Result<i32, String> {
        match mode {
            Mode::Positional => self
                .memory
                .get(val as usize)
                .map(|i| *i)
                .ok_or(format!("address out of bounds: {}", val)),
            Mode::Immeadiate => Ok(val),
        }
    }

    fn get_operation(&self, address: usize) -> Result<Operation, String> {
        self.memory
            .get(address)
            .ok_or("address out of bounds".into())
            .and_then(|i| Operation::size_by_code(*i))
            .and_then(|size| {
                self.memory
                    .get(self.pointer..self.pointer + size)
                    .ok_or("arguments out of bounds".into())
            })
            .and_then(Operation::try_from)
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

    fn run_debug(&mut self) -> BoxResult<()> {
        let result = self.run();
        result.unwrap_or_else(|err| {
            println!("ERROR {}", err);
            println!("memory dump: pointer={} {:?}", self.pointer, self.memory);
            panic!();
        });
        Ok(())
    }

    /// Performs the operation on the current computer state. Returns a result with a boolean value
    /// indicating whether the program should exit after the operation.
    fn perform(&mut self, operation: Operation) -> BoxResult<bool> {
        let op_size = operation.size();
        match operation {
            Operation::Add {
                a,
                b,
                a_mode,
                b_mode,
                save,
            } => {
                let aval = self.get(a, a_mode)?;
                let bval = self.get(b, b_mode)?;
                self.memory[save] = aval + bval;
                self.pointer += op_size;
                Ok(false)
            }
            Operation::Mul {
                a,
                b,
                a_mode,
                b_mode,
                save,
            } => {
                let aval = self.get(a, a_mode)?;
                let bval = self.get(b, b_mode)?;
                self.memory[save] = aval * bval;
                self.pointer += op_size;
                Ok(false)
            }
            Operation::In { save } => {
                self.stdout.write("Input: ".as_bytes())?;
                self.stdout.flush()?;
                let mut input = String::new();
                self.stdin.read_line(&mut input)?;
                let val: i32 = input.trim().parse().or(Err("invalid input"))?;
                self.set(save, val)?;
                self.pointer += op_size;
                Ok(false)
            }
            Operation::Out { read, read_mode } => {
                let val = self.get(read as i32, read_mode)?;
                self.stdout.write(format!("{}\n", val).as_bytes())?;
                self.pointer += op_size;
                Ok(false)
            }
            Operation::JumpIfTrue {
                check,
                address,
                check_mode,
                address_mode,
            } => {
                let check_val = self.get(check, check_mode)?;
                if check_val != 0 {
                    let address_val = self.get(address as i32, address_mode)?;
                    self.pointer = address_val as usize;
                } else {
                    self.pointer += op_size;
                }
                Ok(false)
            }
            Operation::JumpIfFalse {
                check,
                address,
                check_mode,
                address_mode,
            } => {
                let check_val = self.get(check, check_mode)?;
                if check_val == 0 {
                    let address_val = self.get(address as i32, address_mode)?;
                    self.pointer = address_val as usize;
                } else {
                    self.pointer += op_size;
                }
                Ok(false)
            }
            Operation::LessThan {
                a,
                b,
                save,
                a_mode,
                b_mode,
            } => {
                let a_val = self.get(a, a_mode)?;
                let b_val = self.get(b, b_mode)?;
                let result = if a_val < b_val { 1 } else { 0 };
                self.memory[save as usize] = result;
                self.pointer += op_size;
                Ok(false)
            }
            Operation::Eql {
                a,
                b,
                save,
                a_mode,
                b_mode,
            } => {
                let a_val = self.get(a, a_mode)?;
                let b_val = self.get(b, b_mode)?;
                let result = if a_val == b_val { 1 } else { 0 };
                self.memory[save as usize] = result;
                self.pointer += op_size;
                Ok(false)
            }
            Operation::Exit => Ok(true),
        }
    }
}

fn load_program_file<'a>(path: &str) -> Program<'a> {
    let f = fs::File::open(path).expect("unable to open program file");
    let reader = io::BufReader::new(f);
    let program = reader
        .split(b',')
        .filter_map(Result::ok)
        .map(String::from_utf8)
        .filter_map(Result::ok)
        .map(|s| s.trim().parse())
        .filter_map(Result::ok)
        .collect();
    Box::leak(program)
}

fn main() {
    println!("loading program...");
    let path = env::args().nth(1).expect("program file required");
    let program = load_program_file(&path);
    println!("program: {:?}\n\n", program);
    let stdin = io::stdin(); // ensure borrowed value not dropped
    let stdin = stdin.lock();
    let stdout = io::stdout();
    let mut comp = Intcomp::new_with_io(stdin, stdout, program);
    comp.run().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_program_and_check_result(program: Program, expected: Program) {
        let stdin = b"";
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(&stdin[..], &mut stdout, program);
        comp.run_debug().unwrap();
        println!("result   = {:?}", &comp.memory);
        println!("expected = {:?}", &expected);
        assert_eq!(comp.memory, expected);
    }

    #[test]
    fn intcomp_add() {
        run_program_and_check_result(&mut vec![1, 0, 0, 0, 99], &mut vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn intcomp_mul_1() {
        run_program_and_check_result(&mut vec![2, 3, 0, 3, 99], &mut vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn intcomp_mul_2() {
        run_program_and_check_result(
            &mut vec![2, 4, 4, 5, 99, 0],
            &mut vec![2, 4, 4, 5, 99, 9801],
        );
    }

    #[test]
    fn intcomp_combined_1() {
        run_program_and_check_result(
            &mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            &mut vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    #[test]
    fn intcomp_combined_2() {
        run_program_and_check_result(
            &mut vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            &mut vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
    }

    #[test]
    fn stdin_stdout() {
        let stdin = b"94";
        let mut stdout = Vec::new();
        let mut program: Vec<i32> = vec![3, 0, 4, 0, 99];
        let mut comp = Intcomp::new_with_io(&stdin[..], &mut stdout, &mut program);
        comp.run_debug().unwrap();
        assert_eq!(&comp.memory[0], &94i32);
        let output = String::from_utf8(stdout).unwrap();
        assert!(output.contains("94"));
    }

    #[test]
    fn test_digits() {
        assert_eq!(digits(1002), vec![1, 0, 0, 2]);
    }

    #[test]
    fn operation_mode_parsing() {
        let (operation, modes) = Operation::parse_operation_modes(1002);
        assert_eq!(operation, 2);
        assert_eq!(modes, vec![0, 1]);

        let (operation, modes) = Operation::parse_operation_modes(2);
        assert_eq!(operation, 2);
        assert_eq!(modes, vec![]);

        let (operation, modes) = Operation::parse_operation_modes(1097);
        assert_eq!(operation, 97);
        assert_eq!(modes, vec![0, 1]);
    }

    #[test]
    fn position_mode_eql() {
        let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let stdin = &b"8"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        assert!(stdout.contains(&b'1'));

        let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let stdin = &b"9"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        assert!(stdout.contains(&b'0'));
    }

    #[test]
    fn position_mode_less_than() {
        let mut program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let stdin = &b"5"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        assert!(stdout.contains(&b'1'));

        let mut program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let stdin = &b"9"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        assert!(stdout.contains(&b'0'));
    }

    #[test]
    fn immeadiate_mode_eql() {
        let mut program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let stdin = &b"8"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        assert!(stdout.contains(&b'1'));

        let mut program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let stdin = &b"9"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        assert!(stdout.contains(&b'0'));
    }

    #[test]
    fn immeadiate_mode_less_than() {
        let mut program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let stdin = &b"5"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        assert!(stdout.contains(&b'1'));

        let mut program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let stdin = &b"10"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        assert!(stdout.contains(&b'0'));
    }

    #[test]
    fn immeadiate_mode_jump() {
        let mut program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let stdin = &b"10"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        let out_str = String::from_utf8(stdout).unwrap();
        println!("{}", out_str);
        assert!(out_str.contains('1'));

        let mut program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let stdin = &b"0"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        assert!(stdout.contains(&b'0'));
    }

    #[test]
    fn intcomp_combined_3() {
        let mut program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let stdin = &b"5"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        let out_str = String::from_utf8(stdout).unwrap();
        println!("{}", out_str);
        assert!(out_str.contains("999"));

        let mut program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let stdin = &b"8"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        let out_str = String::from_utf8(stdout).unwrap();
        println!("{}", out_str);
        assert!(out_str.contains("1000"));

        let mut program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let stdin = &b"300"[..];
        let mut stdout = Vec::new();
        let mut comp = Intcomp::new_with_io(stdin, &mut stdout, &mut program);
        comp.run_debug().unwrap();
        let out_str = String::from_utf8(stdout).unwrap();
        println!("{}", out_str);
        assert!(out_str.contains("1001"));
    }
}
