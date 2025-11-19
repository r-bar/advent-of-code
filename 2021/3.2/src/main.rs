use std::{env, fs};
use std::io::prelude::*;
use std::io::BufReader;

type Val = Vec<bool>;
type Data = Vec<Val>;
struct DiagReport {
    width: usize,
    data: Data,
}


impl DiagReport {


    fn from_file(path: &str) -> anyhow::Result<Self> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(&file);
        let mut data = Vec::new();
        let mut width = 0;
        for line in reader.lines() {
            let str = line?.trim().to_string();
            let val = str_to_val(&str);
            width = str.len();
            data.push(val);
        }
        Ok(DiagReport { data, width })
    }

    fn calc_gamma_epsilon(&self) -> (Val, Val) {
        let gamma_threshold = self.data.len() / 2;
        let mut accum = vec![0; self.width];
        for report_val in &self.data {
            for (e, v) in report_val.iter().enumerate() {
                accum[e] += *v as usize;
            }
        }
        // ensures that we perform a binary NOT operation using only the applicable bits
        let mut gamma = vec![false; self.width];
        let mut epsilon = vec![false; self.width];
        for e in 0..self.width {
            if accum[e] > gamma_threshold {
                gamma[e] = true;
            } else {
                epsilon[e] = true;
            }
        }
        (gamma, epsilon)
    }

    fn o2_scrubber_rating(&self) -> &Val {
        let mut haystack: Vec<&Val> = self.data.iter().collect();
        for pointer in 0..self.width {
            let col_count: usize = haystack.iter().map(|v| v[pointer] as usize).sum();
            let mode = col_count >= (div_ciel(haystack.len(), 2));
            drain_filter(&mut haystack, |v| v[pointer] != mode);
            if haystack.len() == 1 {
                return haystack[0]
            }
        }
        haystack[0]
    }

    fn co2_scrubber_rating(&self) -> &Val {
        let mut haystack: Vec<&Val> = self.data.iter().collect();
        for pointer in 0..self.width {
            let col_count: usize = haystack.iter().map(|v| v[pointer] as usize).sum();
            let mode = col_count >= (div_ciel(haystack.len(), 2));
            drain_filter(&mut haystack, |v| v[pointer] == mode);
            if haystack.len() == 1 {
                return haystack[0]
            }
        }
        haystack[0]
    }

    fn life_support_rating(&self) -> anyhow::Result<usize> {
        let o2_rating = val_to_usize(self.o2_scrubber_rating());
        let co2_rating = val_to_usize(self.co2_scrubber_rating());
        Ok(o2_rating * co2_rating)
    }

}

/// Removes elements from the vector where the filter evaluates to true
fn drain_filter<T, F>(vec: &mut Vec<T>, mut filter: F)
    where F: FnMut(&mut T) -> bool
{
    let mut i = 0;
    while i < vec.len() {
        if filter(&mut vec[i]) {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}

fn binstr(i: usize, width: usize) -> String {
    let mut i = i;
    let mut str = String::new();
    for e in (0..width).rev() {
        let pow = 2usize.pow(e as u32);
        if i >= pow {
            str.push('1');
            i -= pow;
        } else {
            str.push('0');
        }
    }
    str
}

fn usize_to_val(i: usize, width: usize) -> Val {
    let mut out = vec![false; width];
    let mut i = i;
    for e in (0..width).rev() {
        let pow = 2usize.pow(e as u32);
        if i >= pow {
            out[e] = true;
            i -= pow;
        }
    }
    out
}

fn val_to_usize(val: &Val) -> usize {
    val.iter().rev().enumerate().map(|(e, i)| 2usize.pow(e as u32) * (*i as usize)).sum()
}

fn val_to_str(val: &Val) -> String {
    val.iter().map(|v| if *v { '1' } else { '0' }).collect()
}

fn str_to_val(str: &str) -> Val {
    str.chars().map(|c| c == '1').collect()
}

fn div_ciel(a: usize, b: usize) -> usize {
    if a % b == 0 {
        a / b
    } else {
        (a / b) + 1
    }
}

fn main() {
    let path = env::args().nth(1).expect("Input file argument is required");
    let report = DiagReport::from_file(&path).expect("Could not read data file");
    let (gamma, epsilon) = report.calc_gamma_epsilon();
    //dbg!(&gamma, &epsilon);
    println!("gamma: {} epsilon: {}", val_to_str(&gamma), val_to_str(&epsilon));
    println!("gamma: {} epsilon: {}", val_to_usize(&gamma), val_to_usize(&epsilon));
    let power = val_to_usize(&gamma) * val_to_usize(&epsilon);
    println!("power: {}", power);
    let o2_rating = report.o2_scrubber_rating();
    let co2_rating = report.co2_scrubber_rating();
    println!("o2 rating: {} ({})", val_to_str(&o2_rating), val_to_usize(&o2_rating));
    println!("co2 rating: {} ({})", val_to_str(&co2_rating), val_to_usize(&co2_rating));
    println!("life support rating: {}", report.life_support_rating().unwrap());
}
