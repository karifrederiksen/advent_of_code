use std::collections::hash_map::HashMap;

use super::part1;

pub fn find_potential_passwords(min: u32, max: u32) -> Vec<u32> {
    let mut eligible: Vec<u32> = Vec::new();

    for x in min..=max {
        if is_eligible(x) {
            eligible.push(x);
        }
    }

    return eligible;
}

fn is_eligible(n: u32) -> bool {
    if !part1::is_six_digits(n) {
        return false;
    }
    let digits: Vec<char> = n.to_string().chars().collect();
    if !part1::is_sorted_asc(&digits) {
        return false;
    }

    // since the input is sorted, we can just count how many intances of each character appears.
    let mut char_count: HashMap<char, u8> = HashMap::new();
    for n in digits {
        let next_count = match char_count.get(&n) {
            None => 1,
            Some(x) => x + 1,
        };
        char_count.insert(n, next_count);
    }

    let has_exactly_two_adjacent = char_count
        .into_iter()
        .find(|&(_, count)| count == 2)
        .is_some();

    return has_exactly_two_adjacent;
}

#[test]
fn examples() {
    assert_eq!(is_eligible(112233), true);
    assert_eq!(is_eligible(123444), false);
    assert_eq!(is_eligible(111122), true);
}
