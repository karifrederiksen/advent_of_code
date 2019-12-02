use std::str;

pub fn calc(input: &str) -> u64 {
    let masses: Vec<u64> = input
        .lines()
        .map(<u64 as std::str::FromStr>::from_str)
        .collect::<Result<Vec<u64>, _>>()
        .expect("Failed to decode input lines");

    let fuel_required: u64 = masses
        .into_iter()
        .map(fuel_required)
        .fold(0, |sum, next| sum + next);
    return fuel_required;
}

#[test]
fn provided_examples() {
    assert_eq!(fuel_required(12), 2);
    assert_eq!(fuel_required(14), 2);
    assert_eq!(fuel_required(1969), 654);
    assert_eq!(fuel_required(100756), 33583);
}

fn fuel_required(mass: u64) -> u64 {
    return ((mass as f64) / 3.0).floor() as u64 - 2;
}
