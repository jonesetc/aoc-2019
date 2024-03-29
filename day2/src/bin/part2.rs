const INPUT: &str = include_str!("../../etc/input-part2.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn run_program(mut program: Vec<u32>) -> u32 {
    for ip in (0..=program.len()).step_by(4) {
        let op: u32 = program[ip];

        match op {
            1 => {
                let (input1, input2, output) = (program[ip + 1], program[ip + 2], program[ip + 3]);
                program[output as usize] = program[input1 as usize] + program[input2 as usize];
            }
            2 => {
                let (input1, input2, output) = (program[ip + 1], program[ip + 2], program[ip + 3]);
                program[output as usize] = program[input1 as usize] * program[input2 as usize];
            }
            99 => break,
            _ => panic!("unsupported operation"),
        }
    }

    program[0]
}

fn process(input: &str) -> impl ToString {
    let original_program: Vec<u32> = input
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = original_program.clone();
            program[1] = noun;
            program[2] = verb;

            match run_program(program) {
                19_690_720 => return 100 * noun + verb,
                _ => continue,
            }
        }
    }

    panic!("no answer was found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(run_program(vec![1, 0, 0, 0, 99]), 2);
        assert_eq!(run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
        assert_eq!(
            run_program(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            3_500
        );
    }
}
