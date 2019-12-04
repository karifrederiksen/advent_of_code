mod part1;
mod part2;

fn main() {
    let (min, max) = get_input();

    let solution1 = part1::find_potential_passwords(min, max).len();
    let solution2 = part2::find_potential_passwords(min, max).len();

    println!("Part1:\n{}\n\nPart2:\n{}\n\n", solution1, solution2);
}

fn get_input() -> (u32, u32) {
    let range_inputs: Vec<u32> = "272091-815432"
        .split("-")
        .map(|x| x.parse::<u32>().expect("should be a number"))
        .collect();

    let min = range_inputs[0];
    let max = range_inputs[1];
    return (min, max);
}

#[test]
fn regression_test() {
    let (min, max) = get_input();
    assert_eq!(part1::find_potential_passwords(min, max).len(), 931);
    assert_eq!(part2::find_potential_passwords(min, max).len(), 609);
}
