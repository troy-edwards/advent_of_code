mod helpers;



use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let lines = helpers::get_file_separated_or_panic("aoc2025/src/day9.txt", "\n");
    let answer = find_max_area(lines);
    println!(
        "answer: {answer}, found in {:?}",
        start.elapsed()
    );
}

fn lines_to_points(lines: Vec<String>) -> Vec<(u64, u64)> {
    lines.iter().map(|s| {
        let elems: Vec<u64> = s.split(",").map(|e| e.parse().unwrap()).collect();
        (elems[0], elems[1])
    }).collect()
}

fn area_with_corners(first: (u64, u64), second: (u64, u64)) -> u64 {
    (first.0.abs_diff(second.0) + 1) * (first.1.abs_diff(second.1) + 1)
}

fn find_max_area(lines: Vec<String>) -> u64 {
    let points = lines_to_points(lines);
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len(){
            let area = area_with_corners(points[i], points[j]);
            if area > max_area {
                max_area = area;
            }
        } 
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_lines() -> Vec<String> {
"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3".split_whitespace().map(|e| e.to_string()).collect()
    }

    #[test]
    fn lines_to_points_test() {
        let points = lines_to_points(example_lines());
        assert_eq!(points, vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ])
    }


    #[test]
    fn area_with_corners_test() {
        assert_eq!(area_with_corners((2, 5), (9, 7)), 24);
        assert_eq!(area_with_corners((7, 1), (11, 7)), 35);
        assert_eq!(area_with_corners((7, 3), (2, 3)), 6);
    }

    #[test]
    fn part_1_example_test() {
        assert_eq!(find_max_area(example_lines()), 50);
    }
}