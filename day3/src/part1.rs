use super::direction::{Direction, Pos};
use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let wires: Vec<HashMap<Pos, u32>> = input
        .lines()
        .filter(|line| line.trim() != "")
        .map(|line| {
            let directions = line.split(",").map(|loc| Direction::from_str(loc));
            Direction::fill_wire(directions)
        })
        .collect();

    let wire_a = &wires[0];
    let wire_b = &wires[1];
    let mut intersections: Vec<Pos> = Vec::new();
    for (&pos, _) in wire_a {
        if wire_b.contains_key(&pos) {
            intersections.push(pos);
        }
    }
    let mut distances: Vec<u32> = intersections
        .into_iter()
        .filter(|&x| x != Pos { x: 0, y: 0 })
        .map(|p| Pos::manhattan_distance(&p))
        .collect();
    distances.sort();
    return distances[0];
}

#[test]
fn examples() {
    let input1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
                  U62,R66,U55,R34,D71,R55,D58,R83\n\
                  ";
    assert_eq!(solve(input1), 159);

    let input2 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
                  U98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n\
                  ";
    assert_eq!(solve(input2), 135);
}
