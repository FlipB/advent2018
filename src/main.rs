extern crate lib;

use lib::frequency::*;

use std;
use std::fs;
use std::io::BufReader;

fn main() {
    day1("./data/day1_input");
}

fn day1(file: &str) {
    let res = fs::File::open(file);
    let d = match res {
        Ok(data) => data,
        Err(e) => {
            println!("error reading file {}: {}", file, e);
            return;
        }
    };

    let reader = BufReader::new(d);

    let freq_offsets = match frequency::read_frequency_drift_offsets(reader) {
        Err(e) => return println!("error reading frequency drift offsets: {:?}", e),
        Ok(fqo) => fqo,
    };

    let total_frequency_offset = frequency::calc_frequency_drift(freq_offsets);

    println!("Day1: Total Frequency drift: {}", total_frequency_offset);
}
