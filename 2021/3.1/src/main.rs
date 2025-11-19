use std::{env, fs};
use std::io::prelude::*;
use std::io::BufReader;

type Val = usize;
type Data = Vec<Val>;
struct DiagReport {
    width: usize,
    data: Data,
}


fn read_file(path: &str) -> anyhow::Result<DiagReport> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(&file);
    let mut data = Vec::new();
    let mut width = 0;
    for line in reader.lines() {
        let str = line?.trim().to_string();
        let val = Val::from_str_radix(&str, 2)?;
        width = str.len();
        data.push(val);
    }
    Ok(DiagReport { data, width })
}


fn calc_gamma_epsilon(report: &DiagReport) -> (Val, Val) {
    let gamma_threshold = report.data.len() / 2;
    let mut accum = vec![0; report.width];
    for report_val in &report.data {
        let mut val = *report_val;
        for e in (0..report.width).rev() {
            //let pow = (2 as Val).pow(e);
            let pow = 2usize.pow(e as u32);
            let pos_val = (val / pow) as usize;
            accum[e] += pos_val;
            val -= pos_val * pow;
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for e in 0..report.width {
        if accum[e] > gamma_threshold {
            gamma += 2usize.pow(e as u32);
        } else {
            epsilon += 2usize.pow(e as u32);
        }
    }
    (gamma, epsilon)
}


fn main() {
    let path = env::args().nth(1).expect("Input file argument is required");
    let data = read_file(&path).expect("Could not read data file");
    let (gamma, epsilon) = calc_gamma_epsilon(&data);
    dbg!(gamma, epsilon);
    println!("gamma: {:b} epsilon: {:b}", gamma, epsilon);
    println!("power: {}", gamma * epsilon)
}
