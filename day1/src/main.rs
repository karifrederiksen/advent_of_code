use std::fs;

mod part1;
mod part2;

fn main() {
    let input = read_input();
    let fuel_required1 = part1::calc(&input);
    let fuel_required2 = part2::calc(&input);

    println!("Part1:\n{}\n\nPart2:\n{}\n", fuel_required1, fuel_required2);
}

fn read_input() -> String {
    return fs::read_to_string("input.txt").expect("Failed to read input file");
}

#[test]
fn regression_test_part1() {
    assert!(part1::calc(&read_input()) == 3_457_281);
}

#[test]
fn regression_test_part2() {
    assert!(part2::calc(&read_input()) == 5_183_030);
}
