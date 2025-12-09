



fn operation_rows_from_lines(lines: Vec<String>) -> (Vec<char>, Vec<String>) {
    let mut iter = lines.iter().rev();
    let operation_strings = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();
    let num_lines = iter.map(|e| e.clone()).rev().collect();
    (operation_strings, num_lines)
}

fn num_rows_to_column_nums(rows: Vec<String>) -> Vec<Option<u64>> {
    let max_digits = rows.iter().map(|s| s.len()).max().unwrap();
    let mut nums = Vec::new();
    let column_chars: Vec<Vec<char>> = rows
        .iter()
        .map(|s| s.chars().rev().collect())
        .rev()
        .collect();
    for i in 0..max_digits {
        let mut power: u64 = 1;
        let mut total = 0;
        let mut any_found = false;
        for chars in &column_chars {
            if i >= chars.len() {
                continue;
            }

            let digit: u64 = match chars[i].to_string().parse() {
                Ok(digit) => digit,
                Err(_) => continue,
            };
            any_found = true;
            total += power * digit;
            power *= 10;
        }
        if any_found {
            nums.push(Some(total))
        } else {
            nums.push(None)
        }
    }
    nums.reverse();
    nums
}

fn do_operation_on_columns(operations: Vec<char>, num_lists: Vec<Option<u64>>) -> Vec<u64> {
    let lengths = operations.len();
    let nums_lists: Vec<Vec<u64>> = num_lists
        .split(|e| e.is_none())
        .map(|list| list.iter().map(|e| e.unwrap()).collect())
        .collect();
    let mut answers = Vec::new();
    for i in 0..lengths {
        let nums = &nums_lists[i];
        match operations[i] {
            '*' => answers.push(nums.iter().product()),
            '+' => answers.push(nums.iter().sum()),
            _ => panic!("Invalid operation"),
        }
    }
    answers
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lines() -> Vec<String> {
        vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn confirm_split_on_whitespace_is_a_thing() {
        assert_eq!(
            "*   +   *   +".split_whitespace().collect::<Vec<&str>>(),
            vec!["*", "+", "*", "+"]
        )
    }

    #[test]
    fn operation_rows_test() {
        let actual = operation_rows_from_lines(test_lines());

        let answer = (
            vec!['*', '+', '*', '+'],
            vec![
                vec!["123 328  51 64 "],
                vec![" 45 64  387 23 "],
                vec!["  6 98  215 314"],
            ]
                .iter()
                .map(|v| v.iter().map(|e| e.to_string()).collect())
                .collect(),
        );
        assert_eq!(actual, answer)
    }

    #[test]
    fn day2_example_test() {
        let (operations, num_lists) = operation_rows_from_lines(test_lines());
        let column_nums = num_rows_to_column_nums(num_lists);
        let grand_total: u64 = do_operation_on_columns(operations, column_nums)
            .iter()
            .sum();
        assert_eq!(grand_total, 3263827);
    }

    #[test]
    fn num_rows_to_column_nums_test() {
        let rows: Vec<String> = vec![
            "123 328".parse().unwrap(),
            " 45 64 ".parse().unwrap(),
            "  6 98 ".parse().unwrap(),
        ];
        let column_nums = num_rows_to_column_nums(rows);
        assert_eq!(
            column_nums,
            vec![
                Some(1),
                Some(24),
                Some(356),
                None,
                Some(369),
                Some(248),
                Some(8)
            ]
        )
    }
}
