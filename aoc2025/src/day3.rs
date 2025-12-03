pub fn sum_power_banks(bank_strings: Vec<String>) -> u128 {
    let sum: u128 = bank_strings
        .iter()
        .map(|str| max_joltage_of_bank(str))
        .sum();
    sum
}

fn max_joltage_of_bank(bank_string: &str) -> u128 {
    let powers: Vec<u128> = bank_string
        .chars()
        .map(|c| c.to_string().parse::<u128>().unwrap())
        .collect();

    let mut digits: Vec<u128> = Vec::new();
    let mut front_index = 0;
    let digits_to_use = 12;
    for i in 1..=digits_to_use {
        let (index, digit) =
            find_max_with_index_without_start_and_end(&powers, front_index, digits_to_use - i);
        digits.push(digit);
        front_index = index + 1;
    }
    values_from_digits(digits)
}

fn find_max_with_index_without_start_and_end(
    powers: &Vec<u128>,
    skipped_in_front: usize,
    left_on_end: usize,
) -> (usize, u128) {
    let (index, first_digit) = powers
        .iter()
        .take(powers.len() - left_on_end)
        .skip(skipped_in_front)
        .enumerate()
        .fold(
            (0, 1),
            |acc, (index, val)| {
                if val > &acc.1 { (index, *val) } else { acc }
            },
        );
    (index + skipped_in_front, first_digit)
}

fn values_from_digits(digits: Vec<u128>) -> u128 {
    let mut power = 1;
    digits.iter().rev().fold(0, |acc, e| {
        let digit = acc + power * e;
        power *= 10;
        digit
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bank_joltage_examples() {
        assert_eq!(max_joltage_of_bank("987654321111111"), 987654321111);
        assert_eq!(max_joltage_of_bank("811111111111119"), 811111111119);
        assert_eq!(max_joltage_of_bank("234234234234278"), 434234234278);
        assert_eq!(max_joltage_of_bank("818181911112111"), 888911112111);
    }

    #[test]
    fn total_joltage_of_all_banks_from_example() {
        let banks = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(sum_power_banks(banks), 3121910778619);
    }

    #[test]
    fn value_from_digits_vec_test() {
        let digits = vec![1, 2, 3, 4];
        assert_eq!(values_from_digits(digits), 1234)
    }
}
