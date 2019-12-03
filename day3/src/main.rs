mod direction;
mod part1;
mod part2;

fn main() {
    let input = get_input();
    let solution1 = part1::solve(&input);
    let solution2 = part2::solve(&input);

    println!("Part1:\n{}\n\nPart2:\n{}\n", solution1, solution2);
}

fn get_input() -> String {
    let input: String = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    return input;
}

#[test]
fn regression_test() {
    let input = get_input();
    assert_eq!(part1::solve(&input), 1264);
    assert_eq!(part2::solve(&input), 37390);
}
