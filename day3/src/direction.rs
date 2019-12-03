use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn add_x(&self, value: i32) -> Pos {
        return Pos {
            x: self.x + value,
            y: self.y,
        };
    }
    pub fn add_y(&self, value: i32) -> Pos {
        return Pos {
            x: self.x,
            y: self.y + value,
        };
    }

    pub fn manhattan_distance(&self) -> u32 {
        return (self.x.abs() + self.y.abs()) as u32;
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up(u32),
    Right(u32),
    Down(u32),
    Left(u32),
}

impl Direction {
    pub fn from_str(s: &str) -> Direction {
        let (dir, n_str) = s.split_at(1);
        let n = match n_str.parse::<u32>() {
            Ok(n) => n,
            Err(_) => panic!("Failed to parse as int: \"{}\"", n_str),
        };
        return match dir {
            "U" => Direction::Up(n),
            "R" => Direction::Right(n),
            "D" => Direction::Down(n),
            "L" => Direction::Left(n),
            _ => panic!("Unknown direction: {}", dir),
        };
    }

    // Key is the vector (0, 0) -> (x, y)
    // Value is the distance traveled thus far
    pub fn fill_wire<I>(sparse_wire: I) -> HashMap<Pos, u32>
    where
        I: Iterator<Item = Direction>,
    {
        let mut wire_points: HashMap<Pos, u32> = HashMap::new();
        let mut dist: u32 = 0;
        let mut pos = Pos { x: 0, y: 0 };
        for dir in sparse_wire {
            match dir {
                Direction::Up(n) => {
                    let next_pos = pos.add_y(n as i32);
                    for y in (pos.y + 1)..=next_pos.y {
                        dist += 1;
                        wire_points.insert(Pos { x: pos.x, y: y }, dist);
                    }
                    pos = next_pos;
                }
                Direction::Right(n) => {
                    let next_pos = pos.add_x(n as i32);
                    for x in (pos.x + 1)..=next_pos.x {
                        dist += 1;
                        wire_points.insert(Pos { x: x, y: pos.y }, dist);
                    }
                    pos = next_pos;
                }
                Direction::Down(n) => {
                    let next_pos = pos.add_y(-(n as i32));
                    for y in (next_pos.y..pos.y).rev() {
                        dist += 1;
                        wire_points.insert(Pos { x: pos.x, y: y }, dist);
                    }
                    pos = next_pos;
                }
                Direction::Left(n) => {
                    let next_pos = pos.add_x(-(n as i32));
                    for x in (next_pos.x..pos.x).rev() {
                        dist += 1;
                        wire_points.insert(Pos { x: x, y: pos.y }, dist);
                    }
                    pos = next_pos;
                }
            }
        }
        return wire_points;
    }
}
