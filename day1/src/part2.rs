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
    assert_eq!(fuel_required(14), 2);
    assert_eq!(fuel_required(1969), 966);
    assert_eq!(fuel_required(100756), 50346);
}

fn fuel_required(mass: u64) -> u64 {
    let mut remaining_mass = mass;
    let mut total_fuel_required = 0;
    loop {
        match u64::checked_sub(((remaining_mass as f64) / 3.0).floor() as u64, 2) {
            None => {
                break;
            }
            Some(fuel_required) => {
                total_fuel_required += fuel_required;
                remaining_mass = fuel_required;
            }
        };
    }
    return total_fuel_required;
}
