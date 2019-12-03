use std::collections::HashMap;

use super::direction::{Direction, Pos};

pub fn solve(input: &str) -> u32 {
    let sparse_wire: Vec<Vec<Direction>> = input
        .lines()
        .filter(|line| line.trim() != "")
        .map(|line| {
            let directions: Vec<Direction> =
                line.split(",").map(|s| Direction::from_str(&s)).collect();
            directions
        })
        .collect();

    let wire_a: &[Direction] = &sparse_wire[0];
    let wire_b: &[Direction] = &sparse_wire[1];

    let distances_a: HashMap<Pos, u32> = Direction::fill_wire(wire_a.iter().copied());
    let distances_b: HashMap<Pos, u32> = Direction::fill_wire(wire_b.iter().copied());

    let mut best_distance = std::u32::MAX;
    for (pos, dist_from_a) in distances_a {
        if let Some(dist_from_b) = distances_b.get(&pos) {
            let distance = dist_from_a + dist_from_b;
            if distance < best_distance {
                best_distance = distance;
            }
        }
    }

    return best_distance;
}

#[test]
fn examples() {
    let input1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
                  U62,R66,U55,R34,D71,R55,D58,R83\n\
                  ";
    assert_eq!(solve(&input1), 610);
    let input1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
                  U98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n\
                  ";
    assert_eq!(solve(&input1), 410);
}
