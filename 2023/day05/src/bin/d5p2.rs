use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use rayon::prelude::*;
use clap::Parser;

use day05::Almanac;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Version {
    #[default] V1,
    V2,
}

impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v1" => Ok(Version::V1),
            "v2" => Ok(Version::V2),
            _ => Err(anyhow::anyhow!("Invalid version")),
        }
    }
}

#[derive(Debug, Parser)]
struct Args {
    filename: String,
    #[arg(long)]
    version: Option<Version>,
}

fn to_tuple<T>(slice: &[T]) -> (&T, &T) {
    match slice {
        [a, b] => (a, b),
        _ => panic!("Invalid chunk"),
    }
}

fn v1(almanac: &Almanac) -> anyhow::Result<usize> {
    let ranges: Vec<_> = almanac.seeds.chunks(2).map(to_tuple).collect();
    let loc = ranges.into_iter().map(|(&start, length)| {
        (start..start+length).into_par_iter()
            .map(|seed| almanac.seed_to_location(seed)).min().unwrap()
    }).min().unwrap();
    Ok(loc)
}

fn v2(almanac: &Almanac) -> anyhow::Result<usize> {
    let ranges: Vec<_> = almanac.seeds.chunks(2).map(to_tuple).collect();
    let loc = ranges.into_par_iter().map(|(&start, length)| {
        (start..start+length).into_par_iter()
            .map(|seed| almanac.seed_to_location(seed)).min().unwrap()
    }).min().unwrap();
    Ok(loc)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let f = File::open(args.filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let almanac = Almanac::from_str(&buffer)?;
    match args.version.unwrap_or_default() {
        Version::V1 => println!("{}", v1(&almanac)?),
        Version::V2 => println!("{}", v2(&almanac)?),
    }
    Ok(())
}
