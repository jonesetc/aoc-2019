const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn count_all_orbits(direct_orbits: std::collections::HashMap<String, String>) -> u32 {
    direct_orbits
        .keys()
        .map(|satellite| {
            std::iter::successors(Some(satellite), |&body| direct_orbits.get(body))
                .skip(1)
                .count()
        })
        .sum::<usize>() as u32
}

fn process(input: &str) -> impl ToString {
    let direct_orbits: std::collections::HashMap<String, String> = input
        .lines()
        .map(str::trim)
        .map(|line| line.splitn(2, ')').collect())
        .map(|v: Vec<&str>| (v[1].to_string(), v[0].to_string()))
        .collect();

    count_all_orbits(direct_orbits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            process("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L").to_string(),
            "42"
        );
    }
}
