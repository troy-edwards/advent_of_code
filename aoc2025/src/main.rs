use std::time::Instant;
use crate::day3::sum_power_banks;

mod day3;
mod helpers;

fn main() {
    let start = Instant::now();
    let bank_strings = helpers::get_file_separated_or_panic("aoc2025/src/day3.txt", "\n");
    let sum = sum_power_banks(bank_strings);
    println!("total power: {sum}, found in {:?}", start.elapsed());
}