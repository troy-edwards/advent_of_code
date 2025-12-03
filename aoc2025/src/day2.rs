

use std::time::Instant;

mod helpers;

fn main() {
    let start = Instant::now();
    let ids = helpers::get_file_separated_or_panic("aoc2025/src/day2.txt", ",");
    let sum = sum_invalid_ids(ids);
    let elapsed = start.elapsed();
    println!("sum of invalids: {sum}, found in {elapsed:?}");
}

fn is_invalid_id(id: &str) -> bool {
    let half_len = id.len() / 2;
    for l in 1..=half_len {
        if is_invalid_with_chunk_size(id, l) {
            return true;
        }
    }
    false
}

fn is_invalid_with_chunk_size(id: &str, size: usize) -> bool {
    let mut iter = id.as_bytes().chunks(size);
    let first = iter.next().unwrap();
    for val in iter {
        if val != first {
            return false;
        }
    }
    true
}

fn sum_invalid_ids(ids: Vec<String>) -> u128 {
    ids.iter()
        .map(|e| to_string_list_from_range_string(e) )
        .flatten()
        .map(|elem|  to_val_if_invalid(&elem) )
        .sum()
}

fn to_string_list_from_range_string(e: &String) -> Vec<String> {
    let nums: Vec<String> = e.split("-").map(|e| e.to_string()).collect();
    let first: u128 = nums[0].parse().expect("not num");
    let second: u128 = nums[1].parse().expect("not num");
    strings_from_first_to_second(first, second)
}

fn strings_from_first_to_second(first: u128, second: u128) -> Vec<String> {
    (first..=second)
        .collect::<Vec<u128>>()
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
}

fn to_val_if_invalid(elem: &String) -> u128 {
    if is_invalid_id(&elem) {
        elem.parse().expect("Not a num")
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_valid_test() {
        assert_eq!(is_invalid_id("11"), true);
        assert_eq!(is_invalid_id("22"), true);
        assert_eq!(is_invalid_id("99"), true);
        assert_eq!(is_invalid_id("998"), false);
        assert_eq!(is_invalid_id("1012"), false);
        assert_eq!(is_invalid_id("1188511880"), false);
        assert_eq!(is_invalid_id("1188511885"), true);

        assert_eq!(is_invalid_id("111"), true);
    }

    #[test]
    fn sum_invalid_ids_test() {
        let ids: Vec<String> = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ]
            .iter()
            .map(|e| e.to_string())
            .collect();
        let sum = sum_invalid_ids(ids);
        assert_eq!(sum, 4174379265);
    }
}
