const INPUT: &str = include_str!("../../etc/input-part2.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn calculate_launch_fuel(mass: u32) -> u32 {
    std::iter::successors(Some(mass), |&last_mass| (last_mass / 3).checked_sub(2))
        .skip(1)
        .sum()
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
        assert_eq!(calculate_launch_fuel(14), 2);
        assert_eq!(calculate_launch_fuel(1_969), 966);
        assert_eq!(calculate_launch_fuel(100_756), 50_346);
    }
}
