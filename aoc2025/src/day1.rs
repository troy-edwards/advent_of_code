use std::fs::File;
use std::io::Read;

fn get_password_from_lines(lines: Vec<String>) -> i32 {
    let dial_locations = dial_locations_and_crossings_from_lines(lines);
    dial_locations
        .iter()
        .map(|(loc, crossings)| {
            if *loc < 0 || *loc >= 100 {
                panic!("loc was out of range: {loc}")
            }
            if *crossings < 0 {
                panic!("crossings was negative: {crossings}")
            }
            crossings
        })
        .sum::<i32>()
}


fn dial_locations_and_crossings_from_lines(movements: Vec<String>) -> Vec<(i32, i32)> {
    let mut dial_locations = vec![(50, 0)];
    let mut current_location = 50;
    let mut zero_crossings;
    for s in movements {
        let movement = parse_code(&s);
        (current_location, zero_crossings) = apply_movement(current_location, movement);
        dial_locations.push((current_location, zero_crossings));
    }
    dial_locations
}

fn apply_movement(start: i32, movement: i32) -> (i32, i32) {
    let new_location = (start + movement) % 100;
    let final_location = if new_location < 0 {
        new_location + 100
    } else {
        new_location
    };
    let zero_crossings = if movement > 0 {
        (movement + start) / 100
    } else if movement.abs() < start {
        0
    } else if start == 0 {
        (movement.abs() - start) / 100 
    } else {
        (movement.abs() - start) / 100 + 1
    };
    (final_location, zero_crossings)
}

fn parse_code(code: &str) -> i32 {
    match code.to_ascii_lowercase().split_at(1) {
        ("r", num) => num.parse::<i32>().expect("Not actually an i32"),
        ("l", num) => -num.parse::<i32>().expect("Not actually an i32"),
        (_, _) => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_code_gives_values() {
        assert_eq!(parse_code("R32"), 32);
        assert_eq!(parse_code("L500"), -500);
        assert_eq!(parse_code("L10"), -10);
        assert_eq!(parse_code("R0"), 0);
    }

    #[test]
    fn apply_movement_test() {
        assert_eq!(apply_movement(50, -30), (20, 0));
        assert_eq!(apply_movement(50, 30), (80, 0));

        assert_eq!(apply_movement(50, -60), (90, 1));

        // operations from example:
        assert_eq!(apply_movement(50, -68), (82, 1));
        assert_eq!(apply_movement(82, -30), (52, 0));
        assert_eq!(apply_movement(52, 48), (0, 1));
        assert_eq!(apply_movement(0, -5), (95, 0));
        assert_eq!(apply_movement(95, 60), (55, 1));
        assert_eq!(apply_movement(55, -55), (0, 1));
        assert_eq!(apply_movement(0, -1), (99, 0));
        assert_eq!(apply_movement(99, -99), (0, 1));
        assert_eq!(apply_movement(0, 14), (14, 0));
        assert_eq!(apply_movement(14, -82), (32, 1));

        // // my examples
        assert_eq!(apply_movement(90, 9), (99, 0));
        assert_eq!(apply_movement(90, 11), (1, 1));
        assert_eq!(apply_movement(90, 10), (0, 1));
        assert_eq!(apply_movement(90, 110), (0, 2));
        assert_eq!(apply_movement(0, 50), (50, 0));
        assert_eq!(apply_movement(0, -50), (50, 0));

        assert_eq!(apply_movement(50, -20), (30, 0));
        assert_eq!(apply_movement(50, -120), (30, 1));
        assert_eq!(apply_movement(60, -70), (90, 1));

        assert_eq!(apply_movement(10, -10), (0, 1));
        assert_eq!(apply_movement(10, -110), (0, 2));
        assert_eq!(apply_movement(10, -210), (0, 3));

        assert_eq!(apply_movement(50, 1000), (50, 10));
    }

    #[test]
    fn dial_location_from_lines() {
        let test_lines = vec!["R10", "L70", "R110"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let dial_locations = dial_locations_and_crossings_from_lines(test_lines);
        assert_eq!(dial_locations, vec![(50, 0), (60, 0), (90, 1), (0, 2)]);
    }

    #[test]
    fn get_password_from_lines_problem_example() {
        let lines: Vec<String> = vec!["L68","L30","R48","L5","R60","L55","L1","L99","R14","L82"].iter().map(|i| i.to_string()).collect();
        assert_eq!(get_password_from_lines(lines), 6);
    }
}
