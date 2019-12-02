const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn calculate_launch_fuel(mass: u32) -> u32 {
    (mass / 3) - 2
}

fn process(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .map(calculate_launch_fuel)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(calculate_launch_fuel(12), 2);
        assert_eq!(calculate_launch_fuel(14), 2);
        assert_eq!(calculate_launch_fuel(1_969), 654);
        assert_eq!(calculate_launch_fuel(100_756), 33_583);
    }
}
