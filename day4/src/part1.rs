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
    if !is_six_digits(n) {
        return false;
    }
    let digits: Vec<char> = n.to_string().chars().collect();
    if !is_sorted_asc(&digits) {
        return false;
    }
    return has_adjacent(&digits);
}

#[test]
fn examples() {
    assert_eq!(is_eligible(111111), true);
    assert_eq!(is_eligible(223450), false);
    assert_eq!(is_eligible(123789), false);
}

pub fn is_six_digits(n: u32) -> bool {
    return n > 99_999 && n < 1_000_000;
}

pub fn is_sorted_asc(digits: &[char]) -> bool {
    let mut prev = digits[0];
    for idx in 1..=5 {
        let current = digits[idx];
        if prev > current {
            return false;
        }
        prev = current;
    }
    return true;
}

pub fn has_adjacent(digits: &[char]) -> bool {
    let mut prev = digits[0];
    for idx in 1..=5 {
        let current = digits[idx];
        if current == prev {
            return true;
        }
        prev = current;
    }
    return false;
}
