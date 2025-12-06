use std::time::Instant;

mod day3;
mod helpers;


fn valid_coordinates(max_width: i32, max_height: i32, point: (i32, i32)) -> bool {
    let (x, y) = point;
    x >= 0 && x < max_width && y >= 0 && y < max_height
}

fn find_neighbors(point: (i32, i32), max_width: i32, max_height: i32) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();
    let (p_x, p_y) = point;
    for x_offset in (-1)..=(1) {
        for y_offset in (-1)..=(1) {
            let x = p_x + x_offset;
            let y = p_y + y_offset;
            if valid_coordinates(max_width, max_height, (x, y)) && !(x_offset == 0 && y_offset == 0)
            {
                neighbors.push((x, y))
            }
        }
    }
    neighbors
}

fn is_paper(locations: &Vec<Vec<bool>>, point: (i32, i32)) -> bool {
    let (x, y) = point;
    locations[y as usize][x as usize]
}

fn count_removable_points(locations: &Vec<Vec<bool>>) -> Vec<(i32, i32)> {
    let height = locations.len() as i32;
    let width = locations[0].len() as i32;

    let mut removed_points = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let point = (x, y);
            if !is_paper(&locations, point) {
                continue;
            }

            let neighbors = find_neighbors(point, width, height);
            let count = neighbors
                .iter()
                .filter(|p| is_paper(&locations, **p))
                .count();
            if count < 4 {
                removed_points.push(point);
            }
        }
    }
    removed_points
}

fn count_total_removed(locations: Vec<String>) -> i32 {
    let mut locations = to_bool_vec(locations);
    let mut total_removed = 0;
    loop {
        let points_removed = count_removable_points(&locations);
        total_removed += points_removed.len();
        if points_removed.len() == 0 { break; }
        clear_locations(&mut locations, points_removed);
    }

    total_removed as i32
}

fn clear_locations(locations: &mut Vec<Vec<bool>>, points: Vec<(i32, i32)>) {
    for (x, y) in points {
        locations[y as usize][x as usize] = false;
    }
}


    fn to_bool_vec(location_strings: Vec<String>) -> Vec<Vec<bool>> {
        let mut rows = Vec::new();
        for str in location_strings {
            let row: Vec<bool> = str.chars().map(|c| c == '@').collect();
            rows.push(row);
        }
        rows
    }

#[cfg(test)]
mod tests {
    use super::*;

    fn example_data() -> Vec<String> {
        let str = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];
        str.iter().map(|e| e.to_string()).collect()
    }

    #[test]
    fn find_neighbors_works_for_corners() {
        let zero_zero = (4, 4);
        let mut found_neighbors = find_neighbors(zero_zero, 5, 5);
        found_neighbors.sort();

        let mut actual = vec![(4, 3), (3, 3), (3, 4)];
        actual.sort();

        assert_eq!(actual, found_neighbors)
    }

    #[test]
    fn test_is_paper() {
        let paper = vec![vec![false, true], vec![true, false]];

        assert!(is_paper(&paper, (1, 0)));
        assert!(!is_paper(&paper, (0, 0)));
    }

    #[test]
    fn test_example_problem() {
        let bool_vec = to_bool_vec(example_data());
        let removed_points = count_removable_points(&bool_vec);

        assert_eq!(13, removed_points.len())
    }

    #[test]
    fn test_part_2_example() {
        let count = count_total_removed(example_data());
        assert_eq!(count, 43);
    }

    #[test]
    fn string_vec_to_bool_vec() {
        let paper = vec![".@", "@."].iter().map(|e| e.to_string()).collect();

        let bool_vec = to_bool_vec(paper);
        assert_eq!(bool_vec, vec![vec![false, true], vec![true, false]])
    }
}
