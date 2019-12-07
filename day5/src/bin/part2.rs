const INPUT: &str = include_str!("../../etc/input-part2.txt");

fn main() {
    process(INPUT);
    println!("done");
}

fn get_value(param_mode: u32, param_index: usize, program: &[i32]) -> i32 {
    match param_mode {
        0 => {
            let param = program[param_index];
            program[param as usize]
        }
        1 => program[param_index],
        _ => panic!("unsupported parameter mode"),
    }
}

fn run_program(mut program: Vec<i32>) {
    let mut ip: usize = 0;
    while program[ip] != 99 {
        let instruction = program[ip] as u32;
        let op = instruction % 100;
        let param_modes = (instruction / 100 % 10, instruction / 1_000 % 10);

        match op {
            1 => {
                let (input1, input2) = (
                    get_value(param_modes.0, ip + 1, &program),
                    get_value(param_modes.1, ip + 2, &program),
                );
                let output_addr = program[ip + 3];
                program[output_addr as usize] = input1 + input2;
                ip += 4;
            }
            2 => {
                let (input1, input2) = (
                    get_value(param_modes.0, ip + 1, &program),
                    get_value(param_modes.1, ip + 2, &program),
                );
                let output_addr = program[ip + 3];
                program[output_addr as usize] = input1 * input2;
                ip += 4;
            }
            3 => {
                let output_addr = program[ip + 1];
                program[output_addr as usize] = 5;
                ip += 2;
            }
            4 => {
                let input = get_value(param_modes.0, ip + 1, &program);
                println!("{:}", input);
                ip += 2;
            }
            5 => {
                let input = get_value(param_modes.0, ip + 1, &program);
                if input != 0 {
                    ip = get_value(param_modes.1, ip + 2, &program) as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let input = get_value(param_modes.0, ip + 1, &program);
                if input == 0 {
                    ip = get_value(param_modes.1, ip + 2, &program) as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let (input1, input2) = (
                    get_value(param_modes.0, ip + 1, &program),
                    get_value(param_modes.1, ip + 2, &program),
                );
                let output_addr = program[ip + 3];
                if input1 < input2 {
                    program[output_addr as usize] = 1;
                } else {
                    program[output_addr as usize] = 0;
                }
                ip += 4;
            }
            8 => {
                let (input1, input2) = (
                    get_value(param_modes.0, ip + 1, &program),
                    get_value(param_modes.1, ip + 2, &program),
                );
                let output_addr = program[ip + 3];
                if input1 == input2 {
                    program[output_addr as usize] = 1;
                } else {
                    program[output_addr as usize] = 0;
                }
                ip += 4;
            }
            _ => panic!("unsupported operation"),
        }
    }
}

fn process(input: &str) {
    let program: Vec<i32> = input
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    run_program(program);
}
