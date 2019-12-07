const MIN: u32 = 137_683;
const MAX: u32 = 596_253;

fn main() {
    println!("answer is: {}", process(MIN, MAX).to_string())
}

fn is_valid_password(password: u32) -> bool {
    let (is_ordered, has_consecutive) = password
        .to_string()
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .fold((true, false), |(is_ordered, has_consecutive), curr| {
            (
                is_ordered && (curr[0] <= curr[1]),
                has_consecutive || curr[0] == curr[1],
            )
        });
    is_ordered && has_consecutive
}

fn process(min: u32, max: u32) -> impl ToString {
    (min..=max)
        .filter(|&password| is_valid_password(password))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(is_valid_password(111_111), true);
        assert_eq!(is_valid_password(223_450), false);
        assert_eq!(is_valid_password(123_789), false);
    }
}
