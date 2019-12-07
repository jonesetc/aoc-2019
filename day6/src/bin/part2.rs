const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

fn get_orbit_distance(direct_orbits: std::collections::HashMap<String, String>) -> u32 {
    let target_orbit_distances: std::collections::HashMap<&String, usize> =
        std::iter::successors(direct_orbits.get("SAN"), |&body| direct_orbits.get(body))
            .enumerate()
            .map(|(index, body)| (body, index))
            .collect();

    for (index, body) in std::iter::successors(direct_orbits.get("YOU"), |&satellite| {
        direct_orbits.get(satellite)
    })
    .enumerate()
    {
        match target_orbit_distances.get(body) {
            Some(target_index) => return (index + target_index) as u32,
            None => continue,
        }
    }

    panic!("found no common orbit path");
}

fn process(input: &str) -> impl ToString {
    let direct_orbits: std::collections::HashMap<String, String> = input
        .lines()
        .map(str::trim)
        .map(|line| line.splitn(2, ')').collect())
        .map(|v: Vec<&str>| (v[1].to_string(), v[0].to_string()))
        .collect();

    get_orbit_distance(direct_orbits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            process("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN")
                .to_string(),
            "4"
        );
    }
}
