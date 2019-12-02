pub fn execute(mut memory: Vec<u32>) -> Vec<u32> {
    let mut intruction_pointer = 0;
    loop {
        match memory[intruction_pointer] {
            1 => {
                //add
                let noun = memory[intruction_pointer + 1] as usize;
                let verb = memory[intruction_pointer + 2] as usize;
                let address = memory[intruction_pointer + 3] as usize;
                memory[address] = memory[noun] + memory[verb];
                intruction_pointer += 4;
            }
            2 => {
                //multiply
                let noun = memory[intruction_pointer + 1] as usize;
                let verb = memory[intruction_pointer + 2] as usize;
                let address = memory[intruction_pointer + 3] as usize;
                memory[address] = memory[noun] * memory[verb];
                intruction_pointer += 4;
            }
            99 => {
                //halt
                break;
            }
            _ => {
                panic!("undefined operator: {}", memory[intruction_pointer]);
            }
        }
    }
    return memory;
}

#[test]
fn provided_examples() {
    assert_eq!(execute(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    assert_eq!(execute(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    assert_eq!(execute(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    assert_eq!(
        execute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
}
