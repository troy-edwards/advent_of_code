mod helpers;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::Instant;



#[derive(Debug, PartialEq, PartialOrd)]
#[derive(Eq)]
#[derive(Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    id: usize
}

impl Point {
    fn new(x: i64, y: i64, z: i64, id: usize) -> Point {
        Point { x, y, z, id}
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct PointPair<'a> {
    first: &'a Point,
    second: &'a Point,
    square_dist: i64,
}

impl<'a> PointPair<'a> {
    fn new(p1: &'a Point, p2: &'a Point) -> PointPair<'a> {
        let dist = (p2.x - p1.x).pow(2) + (p2.y - p1.y).pow(2) + (p2.z - p1.z).pow(2);
        if p1 < p2 {
            PointPair {
                first: p1,
                second: &p2,
                square_dist: dist,
            }
        } else {
            PointPair {
                first: &p2,
                second: &p1,
                square_dist: dist,
            }
        }
    }
}

impl Eq for PointPair<'_> {}

impl PartialOrd<Self> for PointPair<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(self.cmp(other))
    }
}

impl Ord for PointPair<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.square_dist.cmp(&other.square_dist)
    }
}

impl PartialEq for PointPair<'_> {
    fn eq(&self, other: &Self) -> bool {
        (self.first == other.first && self.second == other.second)
            || (self.second == other.first && self.first == other.second)
    }
}

fn points_from_lines(lines: Vec<String>) -> Vec<Point> {
    lines
        .iter().enumerate()
        .map(|(index, line)| {
            let elements: Vec<i64> = line.split(",").map(|e| e.parse().unwrap()).collect();
            Point::new(elements[0], elements[1], elements[2], index)
        })
        .collect()
}

fn all_point_pairs(points: &'_ Vec<Point>) -> Vec<PointPair<'_>> {
    let mut pairs = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            pairs.push(PointPair::new(&points[i], &points[j]))
        }
    }
    pairs
}

#[allow(dead_code)]
fn find_n_closest_pairs<'a>(pairs: &'a Vec<PointPair<'_>>, n: usize) -> Vec<&'a PointPair<'a>> {
    let mut pairs_heap = BinaryHeap::new();
    // put n smallest in a heap
    for pair in pairs {
        if pairs_heap.len() < n {
            pairs_heap.push(pair);
            continue;
        }
        let top = pairs_heap.pop().unwrap();
        if pair < top {
            pairs_heap.push(pair);
        } else {
            pairs_heap.push(top);
        }
    }
    let mut closest_pairs = Vec::new();
    for pair in pairs_heap {
        closest_pairs.push(pair);
    }
    closest_pairs.reverse();
    //extract them from heap, and return lowest to highest
    closest_pairs
}

fn find_all_pairs_in_order<'a>(pairs: &'a Vec<PointPair<'_>>) -> Vec<PointPair<'a>> {
    let mut new_list = pairs.clone();
    new_list.sort();
    new_list
}

#[allow(dead_code)]
fn combine_circuits(points: &Vec<Point>, closest_pairs: &Vec<&PointPair>) -> u64 {
    let mut circuit_to_points: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut point_to_circuit: HashMap<usize, usize> = HashMap::new();

    for point in points {
        circuit_to_points.insert(point.id, HashSet::from([point.id]));
        point_to_circuit.insert(point.id, point.id);
    }

    for pair in closest_pairs {
        let first_circuit = point_to_circuit.get(&pair.first.id).unwrap().clone();
        let second_circuit = point_to_circuit.get(&pair.second.id).unwrap().clone();
        if first_circuit == second_circuit { continue }

        let (dest_circuit, source_circuit) = if first_circuit < second_circuit {
            (first_circuit, second_circuit)
        } else {
            (second_circuit, first_circuit)
        };

        let source_hash = circuit_to_points.remove(&source_circuit).unwrap();
        let dest_hash: &mut HashSet<usize> = circuit_to_points.get_mut(&dest_circuit).unwrap();
        for point in source_hash {
            dest_hash.insert(point);
            point_to_circuit.insert(point, dest_circuit);
        }
    }
    let mut circuit_sizes: Vec<usize> = circuit_to_points.values().map(|hash| hash.len()).collect();
    circuit_sizes.sort();
    let product: usize = circuit_sizes.iter().rev().take(3).product();
    product as u64
}

fn combine_until_1_circuit(points: &Vec<Point>, closest_pairs: &Vec<PointPair>) -> u64 {
    let mut circuit_to_points: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut point_to_circuit: HashMap<usize, usize> = HashMap::new();

    for point in points {
        circuit_to_points.insert(point.id, HashSet::from([point.id]));
        point_to_circuit.insert(point.id, point.id);
    }

    for pair in closest_pairs {
        let first_circuit = point_to_circuit.get(&pair.first.id).unwrap().clone();
        let second_circuit = point_to_circuit.get(&pair.second.id).unwrap().clone();
        if first_circuit == second_circuit { continue }

        let (dest_circuit, source_circuit) = if first_circuit < second_circuit {
            (first_circuit, second_circuit)
        } else {
            (second_circuit, first_circuit)
        };

        let source_hash = circuit_to_points.remove(&source_circuit).unwrap();
        let dest_hash: &mut HashSet<usize> = circuit_to_points.get_mut(&dest_circuit).unwrap();
        for point in source_hash {
            dest_hash.insert(point);
            point_to_circuit.insert(point, dest_circuit);
        }
        //println!("first was: {:?}, second was: {:?}. dist: {}, with '{}' total circuits", pair.first, pair.second, pair.square_dist ,circuit_to_points.len());
        if circuit_to_points.len() == 1 { return (pair.first.x * pair.second.x) as u64}
    }
    panic!("somehow never hit 1 long");
}

#[allow(dead_code)]
fn size_of_n_biggest_circuits(lines: Vec<String>, n: usize) -> u64 {
    let points = points_from_lines(lines);
    let all_pairs = all_point_pairs(&points);
    let close_pairs = find_n_closest_pairs(&all_pairs, n);
    let answer = combine_circuits(&points, &close_pairs);
    answer
}

fn product_of_last_2(lines: Vec<String>) -> u64 {
    let points = points_from_lines(lines);
    let all_pairs = all_point_pairs(&points);
    let close_pairs = find_all_pairs_in_order(&all_pairs);
    let answer = combine_until_1_circuit(&points, &close_pairs);
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_data() -> Vec<String> {
        "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            .split_whitespace()
            .map(|e| e.to_string())
            .collect()
    }

    fn short_example_data() -> Vec<String> {
        "862,61,35
984,92,344
425,690,689"
            .split_whitespace()
            .map(|e| e.to_string())
            .collect()
    }

    #[test]
    fn points_from_lines_test() {
        let points = points_from_lines(short_example_data());
        assert_eq!(
            points,
            vec![
                Point::new(862, 61, 35, 0),
                Point::new(984, 92, 344, 1),
                Point::new(425, 690, 689, 2)
            ]
        )
    }

    #[test]
    fn square_dist_on_points_is_correct() {
        let p1 = Point::new(1, 2, 3, 0);
        let p2 = Point::new(3, 3, 4, 1);
        let pair = PointPair::new(&p1, &p2);
        assert_eq!(pair.square_dist, 6);
    }

    #[test]
    fn get_all_pairs_test() {
        let points = points_from_lines(example_data());
        let all_pairs = all_point_pairs(&points);
        assert_eq!(all_pairs.len(), 20 * (20 - 1) / 2)
    }

    #[test]
    fn test_closest_points() {
        let points = points_from_lines(example_data());
        let all_pairs = all_point_pairs(&points);
        let close_pairs = find_n_closest_pairs(&all_pairs, 2);

        assert_eq!(
            close_pairs[0],
            &PointPair::new(
                &Point::new(162, 817, 812, 0),
                &Point::new(425, 690, 689, 19)
            )
        );

        assert_eq!(
            close_pairs[1],
            &PointPair::new(
                &Point::new(162,817,812, 0),
                &Point::new(431,825,988, 7)
            )
        );
    }

    #[test]
    fn part1_with_example() {
        let answer = size_of_n_biggest_circuits(example_data(), 10);
        assert_eq!(answer, 40);
    }

    #[test]
    fn part2_with_example() {
        let answer = product_of_last_2(example_data());
        assert_eq!(answer, 25272);
    }
}
