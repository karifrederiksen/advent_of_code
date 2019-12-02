use super::part1::execute;

pub fn find_the_inputs(initial_memory: &[u32]) -> u32 {
    for verb in 0..99 {
        for noun in 0..99 {
            let mut memory: Vec<u32> = initial_memory.to_vec();
            memory[1] = verb;
            memory[2] = noun;
            let output = execute(memory);
            if output[0] == 1969_07_20 {
                return verb * 100 + noun;
            }
        }
    }

    panic!("things didn't work out");
}
