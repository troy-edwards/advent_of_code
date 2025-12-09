use std::collections::{HashMap, HashSet};



fn get_start_and_splitter_locations(
    lines: Vec<String>,
) -> (HashMap<usize, usize>, Vec<HashSet<usize>>) {
    let mut iter = lines.iter();
    let start_location = iter.next().unwrap().chars().position(|c| c == 'S').unwrap();
    let splitter_locations = iter
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '^')
                .map(|(i, _)| i)
                .collect()
        })
        .filter(|set: &HashSet<usize>| !set.is_empty())
        .collect();
    (HashMap::from([(start_location, 1)]), splitter_locations)
}

fn split_beams_on_splitters(
    beam_locations: &HashMap<usize, usize>,
    splitter_locations: &HashSet<usize>,
) -> (HashMap<usize, usize>, usize) {
    let mut new_locations = HashMap::new();
    let mut split_count = 0;
    for (location, count) in beam_locations {
        if splitter_locations.contains(&location) {
            // dont have to bounds check (input data doesnt touch the edge =D )
            let left = new_locations.entry(location - 1).or_insert(0);
            *left += count;
            let right = new_locations.entry(location + 1).or_insert(0);
            *right += count;
            split_count += 1;
        } else {
            let un_touched = new_locations.entry(*location).or_insert(0);
            *un_touched += count;
        }
    }
    (new_locations, split_count)
}

fn split_and_timeline_count(
    start_location: HashMap<usize, usize>,
    splitter_locations: Vec<HashSet<usize>>,
) -> (usize, usize) {
    let mut current_beams = start_location;
    let mut split_count = 0;
    for locations in &splitter_locations {
        let (new_locations, splits) = split_beams_on_splitters(&current_beams, locations);
        split_count += splits;
        current_beams = new_locations;
    }
    let timelines = current_beams.values().sum();
    (split_count, timelines)
}

fn split_and_timeline_count_from_lines(lines: Vec<String>) -> (usize, usize) {
    let (start_location, splitter_locations) = get_start_and_splitter_locations(lines);
    split_and_timeline_count(start_location, splitter_locations)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<String> {
        ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .split_whitespace()
            .map(|e| e.to_string())
            .collect()
    }

    #[test]
    fn get_start_and_splitter_locations_test() {
        let first_5_lines = test_input().iter().take(5).map(|s| s.to_string()).collect();
        let (beam_locations, splitter_locations) = get_start_and_splitter_locations(first_5_lines);
        assert_eq!(beam_locations, HashMap::from([(7, 1)]));
        assert_eq!(
            splitter_locations,
            vec![HashSet::from([7]), HashSet::from([6, 8]),]
        );
    }

    #[test]
    fn split_beams_on_splitters_test() {
        let beam_locations = HashMap::from([(4, 1), (3, 1), (6, 1)]);
        let splitter_locations = HashSet::from([4, 6, 10]);

        assert_eq!(
            split_beams_on_splitters(&beam_locations, &splitter_locations),
            (HashMap::from([(3, 2), (5, 2), (7, 1)]), 2)
        )
    }

    #[test]
    fn example_data_tests() {
        let (splits, timelines) = split_and_timeline_count_from_lines(test_input());
        assert_eq!(splits, 21);
        assert_eq!(timelines, 40);
    }
    fn test_input_full() -> Vec<String> {
        ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^.^.^....
...............
...^.^.^.^.^...
...............
..^.^.^.^.^.^..
...............
.^.^.^.^.^.^.^.
..............."
            .split_whitespace()
            .map(|e| e.to_string())
            .collect()
    }

    #[test]
    fn test_data_full() {
        let (_, timelines) = split_and_timeline_count_from_lines(test_input_full());
        assert_eq!(timelines, 128);
    }
}
