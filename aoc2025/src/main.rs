use std::cmp::Ordering;
use std::time::Instant;
mod helpers;

fn main() {
    let start = Instant::now();
    let location_strings = helpers::get_file_separated_or_panic("aoc2025/src/day5.txt", "\n");
    let ranges = get_ranges_from_lines(location_strings);
    let combined_ranges = combine_ranges(ranges);
    let total_count: u64 = combined_ranges.iter().map(|r| r.size()).sum();
    println!("valid ids: {total_count}, found in {:?}", start.elapsed());
}

#[derive(Debug, PartialEq)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Range {
    fn new(lower: u64, upper: u64) -> Range {
        Range { lower, upper }
    }

    fn size(&self) -> u64 {
        self.upper - self.lower + 1
    }
}

#[derive(PartialEq, Eq, Debug)]
enum RangePoint {
    Start(u64),
    End(u64),
}

impl PartialOrd<Self> for RangePoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RangePoint {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = value_in_range_point(self);
        let right = value_in_range_point(other);
        match left.cmp(&right) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match (self, other) {
                (RangePoint::Start(l_val), RangePoint::Start(r_val))
                | (RangePoint::End(l_val), RangePoint::End(r_val)) => l_val.cmp(r_val),
                (RangePoint::Start(_), RangePoint::End(_)) => Ordering::Less,
                (RangePoint::End(_), RangePoint::Start(_)) => Ordering::Greater,
            },
        }
    }
}

fn get_ranges_from_lines(lines: Vec<String>) -> Vec<Range> {
    lines
        .iter()
        .take_while(|s| *s != "")
        .map(|s| {
            let mut split = s.split("-");
            let lower = split.next().unwrap().parse().unwrap();
            let upper = split.next().unwrap().parse().unwrap();
            Range::new(lower, upper)
        })
        .collect()
}

fn value_in_range_point(point: &RangePoint) -> u64 {
    match point {
        RangePoint::Start(val) => *val,
        RangePoint::End(val) => *val,
    }
}

fn ranges_to_sorted_points(ranges: Vec<Range>) -> Vec<RangePoint> {
    let mut list: Vec<RangePoint> = ranges
        .iter()
        .map(|r| [RangePoint::Start(r.lower), RangePoint::End(r.upper)])
        .flatten()
        .collect();
    list.sort();
    list
}

fn combine_ranges(range: Vec<Range>) -> Vec<Range> {
    let sorted_points = ranges_to_sorted_points(range);
    let mut open_count = 0;
    let mut current_start: Option<u64> = None;
    let mut found_ranges: Vec<Range> = Vec::new();

    for point in sorted_points {
        match point {
            RangePoint::Start(start_val) => {
                open_count += 1;
                if current_start == None {
                    current_start = Some(start_val)
                }
            }
            RangePoint::End(end_val) => {
                open_count -= 1;
                if open_count == 0 {
                    let start_val =
                        current_start.expect("Should never have an End without a start");
                    found_ranges.push(Range::new(start_val, end_val));
                    current_start = None;
                }
            }
        }
    }
    found_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ranges() -> Vec<Range> {
        vec![
            Range::new(3, 5),
            Range::new(4, 4),
            Range::new(10, 14),
            Range::new(16, 20),
            Range::new(12, 18),
            Range::new(20, 20),
        ]
    }

    fn sorted_range_points() -> Vec<RangePoint> {
        vec![
            RangePoint::Start(3),
            RangePoint::Start(4),
            RangePoint::End(4),
            RangePoint::End(5),
            RangePoint::Start(10),
            RangePoint::Start(12),
            RangePoint::End(14),
            RangePoint::Start(16),
            RangePoint::End(18),
            RangePoint::Start(20),
            RangePoint::End(20),
            RangePoint::End(20),
        ]
    }

    #[test]
    fn ranges_get_turned_to_range_points_than_sort_correctly() {
        let sorted = ranges_to_sorted_points(test_ranges());
        assert_eq!(sorted, sorted_range_points())
    }

    #[test]
    fn combine_ranges_test() {
        assert_eq!(
            combine_ranges(test_ranges()),
            vec![Range::new(3, 5), Range::new(10, 20)]
        )
    }
}
