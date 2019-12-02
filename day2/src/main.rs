mod part1;
mod part2;

fn main() {
    let initial_memory = get_initial_memory();
    let output1 = part1::execute(initial_memory.to_vec());
    let output2 = part2::find_the_inputs(&initial_memory);
    println!("Part1:\n{:?}\nPart2:\n{:?}\n", output1[0], output2);
}

fn get_initial_memory() -> Vec<u32> {
    let initial_memory: Vec<u32> = std::fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .lines()
        .flat_map(|line| {
            line.split(",")
                .map(<u32 as std::str::FromStr>::from_str)
                .collect::<Result<Vec<u32>, _>>()
                .expect("Failed to parse csv")
        })
        .collect();
    return initial_memory;
}

#[test]
fn regression_test_part1() {
    let initial_memory = get_initial_memory();
    let output1 = part1::execute(initial_memory.to_vec());
    assert_eq!(output1[0], 5_305_097);
}

#[test]
fn regression_test_part2() {
    let initial_memory = get_initial_memory();
    let output2 = part2::find_the_inputs(&initial_memory);
    assert_eq!(output2, 49_25);
}
