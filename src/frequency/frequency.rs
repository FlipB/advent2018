use std;
use std::io::BufRead;
use std::vec::Vec;

use super::error::Error;

/// read_frequency_drift_offsets reads supplied reader for lines of frequency drift offsets
/// frequency drift offset lines are converted to vector of integers if no error occurs
/// Input is impl BufRead in order to enable unit testing because thats something i wanted to try out
/// # Arguments
/// impl BufRead containing frequency offsets separated by line breaks
/// # Returns
/// int vector representation of frequency offsets or an error
pub fn read_frequency_drift_offsets(reader: impl BufRead) -> Result<Vec<i64>, Error> {
    let mut freq_offsets: Vec<i64> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => match line.parse::<i64>() {
                Ok(frequency_offset) => freq_offsets.push(frequency_offset),
                Err(error) => {
                    return Err(Error::ParseErr(
                        error,
                        format!("unable to parse line '{}' for frequency offset", line).to_owned(),
                    ))
                }
            },
            Err(error) => {
                return Err(Error::IOErr(
                    error,
                    "error reading lines from file".to_owned(),
                ))
            }
        }
    }
    Ok(freq_offsets)
}

/// calc_frequency_drift takes an offset of integers and calculates the sum which is returned
pub fn calc_frequency_drift(freq_offsets: Vec<i64>) -> i64 {
    freq_offsets
        .iter()
        .fold(0i64, |val: i64, sum: &i64| sum + val)
}

#[test]
fn read_frequency_drift_offsets_test() {
    let v = read_frequency_drift_offsets("+1\n-13\n+4\n0\n".as_bytes());
    match v {
        Ok(v) => match v.as_slice() {
            [1, -13, 4, 0] => assert!(true),
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    // This also works:
    //assert_eq!(v, vec![1, -13, 4, 0])
    // But pattern matching and vector destructuring is much cooler...
}

#[test]
fn calc_frequency_drift_test() {
    let v = vec![1, -5, 15, 2, 3, 5, -6];
    assert!(calc_frequency_drift(v) == 15);
    assert!(calc_frequency_drift(vec![]) == 0);
    assert!(calc_frequency_drift(vec![2]) == 2);
    assert!(calc_frequency_drift(vec![-5]) == -5);
}
