use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        complete::{digit1, multispace0, multispace1},
        streaming::char,
    },
    combinator::map_res,
    error::{Error as NomError, ErrorKind},
    multi::separated_list0,
    sequence::{delimited, preceded},
    Finish, IResult,
};
use std::str::FromStr;

// will recognize the name in "Hello, name!"
fn parse_name(input: &str) -> IResult<&str, &str> {
    let (i, _) = tag("Hello, ")(input)?;
    let (i, name) = take_while(|c: char| c.is_alphabetic())(i)?;
    let (i, _) = tag("!")(i)?;

    Ok((i, name))
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// with FromStr, the result cannot be a reference to the input, it must be owned
#[derive(Debug)]
pub struct Name(pub String);

impl FromStr for Name {
    // the error must be owned as well
    type Err = NomError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_name(s).finish() {
            Ok((_remaining, name)) => Ok(Name(name.to_string())),
            Err(NomError { input, code }) => Err(NomError {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Expr, Expr),
    Mul(Expr, Expr),
}

impl Operation {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (i, left) = ws(Expr::parse)(input)?;
        let (i, symbol) = alt((char('*'), char('+')))(i)?;
        //let (i, symbol) = alt(tag("*"), tag("+"))(i)?;
        let (i, right) = ws(Expr::parse)(i)?;
        match symbol {
            '*' => Ok((i, Operation::Mul(left, right))),
            '+' => Ok((i, Operation::Add(left, right))),
            _ => Err(nom::Err::Failure(NomError::new(i, ErrorKind::Fail))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Expr {
    Num(usize),
    Old,
}

impl Expr {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (i, expr) = alt((tag("old"), digit1))(input)?;
        if expr == "old" {
            Ok((i, Self::Old))
        } else if let Ok(value) = expr.parse::<usize>() {
            Ok((i, Self::Num(value)))
        } else {
            Err(nom::Err::Failure(NomError::new(i, ErrorKind::Fail)))
        }
    }

    fn resolve(&self, old: usize) -> usize {
        match self {
            Self::Old => old,
            Self::Num(i) => *i,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    index: usize,
    items: Vec<usize>,
    operation: Operation,
    test_divisible: usize,
    true_throw_index: usize,
    false_throw_index: usize,
    inspections: usize,
}

impl Monkey {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (i, _) = multispace0(input)?;
        let (i, _) = tag("Monkey ")(i)?;
        let (i, index): (&str, usize) =
            map_res(take_while(|c: char| c.is_digit(10)), |s: &str| s.parse())(i)?;
        let (i, _) = tag(":")(i)?;
        let (i, _) = multispace1(i)?;

        let (i, items): (&str, Vec<usize>) = map_res(
            preceded(
                tag("Starting items: "),
                separated_list0(ws(tag(",")), take_while(|c: char| c.is_digit(10))),
            ),
            |v| v.into_iter().map(|s| s.parse()).collect(),
        )(i)?;
        let (i, _) = multispace1(i)?;

        let (i, operation) = preceded(tag("Operation: new = "), Operation::parse)(i)?;

        let (i, test_divisible) =
            map_res(preceded(tag("Test: divisible by "), digit1), |s: &str| {
                s.parse::<usize>()
            })(i)?;
        let (i, _) = multispace1(i)?;

        let (i, true_throw_index) = map_res(
            preceded(tag("If true: throw to monkey "), digit1),
            |s: &str| s.parse::<usize>(),
        )(i)?;
        let (i, _) = multispace1(i)?;
        let (i, false_throw_index) = map_res(
            preceded(tag("If false: throw to monkey "), digit1),
            |s: &str| s.parse::<usize>(),
        )(i)?;
        let (i, _) = multispace0(i)?;
        let monkey = Monkey {
            index,
            items,
            operation,
            test_divisible,
            true_throw_index,
            false_throw_index,
            inspections: 0,
        };
        Ok((i, monkey))
    }

    fn test(&self, item: usize) -> bool {
        item % self.test_divisible == 0
    }

    fn operate(&self, item: usize) -> usize {
        match self.operation {
            Operation::Add(left, right) => left.resolve(item) + right.resolve(item),
            Operation::Mul(left, right) => left.resolve(item) * right.resolve(item),
        }
    }

    fn catch(&mut self, item: usize) {
        self.items.push(item);
    }

    // Runs all actions on an item and returns a list of items and the index of the monkey to toss
    // them too
    fn inspect(&mut self) -> Vec<(usize, usize)> {
        let mut throws = Vec::new();
        for item in &self.items {
            let item = self.operate(*item);
            let to_monkey = if self.test(item) {
                self.true_throw_index
            } else {
                self.false_throw_index
            };
            throws.push((item, to_monkey))
        }
        self.inspections += self.items.len();
        self.items.clear();
        throws
    }

    fn reduce(&mut self, divisor_product: usize) {
        self.items = self.items.iter().map(|i| i % divisor_product).collect();
    }
}

fn run_rounds(monkeys: &mut Vec<Monkey>, rounds: usize) -> anyhow::Result<()> {
    let divisor_product = monkeys.iter().map(|m| m.test_divisible).product();
    for _round in 0..rounds {
        run_round(monkeys, divisor_product)?;
    }
    Ok(())
}

fn run_round(monkeys: &mut Vec<Monkey>, divisor_product: usize) -> anyhow::Result<()> {
    for monkey_index in 0..monkeys.len() {
        let throws = monkeys[monkey_index].inspect();
        for (item, to_monkey) in throws {
            monkeys[to_monkey].catch(item);
        }
    }
    for monkey in monkeys {
        monkey.reduce(divisor_product);
    }
    Ok(())
}

fn calculate_monkey_business(monkeys: &Vec<Monkey>) -> usize {
    let count = monkeys.len();
    let mut inspections: Vec<usize> = monkeys.iter().map(|monkey| monkey.inspections).collect();
    inspections.sort();
    inspections[(count - 2)..count].iter().product()
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = std::fs::read_to_string(&filename)?;
    let mut remaining = input.as_str();
    let mut monkeys = Vec::new();
    while !remaining.is_empty() {
        let (i, monkey) = Monkey::parse(remaining).unwrap();
        monkeys.push(monkey);
        remaining = i;
    }
    run_rounds(&mut monkeys, 10000)?;
    println!("{}", calculate_monkey_business(&monkeys));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_monkey() {
        let example = include_str!("../../example.txt");
        let monkey = Monkey::parse(example).unwrap();
        dbg!(monkey);
    }

    #[test]
    fn test_parse_monkeys() {
        let example = include_str!("../../example.txt");
        let mut remaining = example;
        let mut monkeys = Vec::new();
        while !&remaining.is_empty() {
            let (i, monkey) = Monkey::parse(remaining).unwrap();
            monkeys.push(monkey);
            remaining = i;
        }
        dbg!(&monkeys);
        assert_eq!(monkeys.len(), 4);
    }

    #[test]
    fn test_10000_rounds_of_monkey_business() {
        let example = include_str!("../../example.txt");

        let mut monkeys = Vec::new();
        let mut remaining = example;
        while !remaining.is_empty() {
            let (i, monkey) = Monkey::parse(remaining).unwrap();
            monkeys.insert(monkey.index, monkey);
            remaining = i;
        }
        run_rounds(&mut monkeys, 10000).unwrap();
        assert_eq!(monkeys[0].inspections, 52166);
        assert_eq!(monkeys[1].inspections, 47830);
        assert_eq!(monkeys[2].inspections, 1938);
        assert_eq!(monkeys[3].inspections, 52013);
        assert_eq!(calculate_monkey_business(&monkeys), 2713310158);
    }
}
