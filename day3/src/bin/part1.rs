const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

enum Direction { UP, DOWN, RIGHT, LEFT }
struct Action { pub direction: Direction, pub steps: u32 }
struct Grid { hit_counts: std::collections::HashMap<(i32, i32), u32> }

impl Grid {
    fn new() -> Grid {
        Grid{ hit_counts: std::collections::HashMap::new() }
    }

    fn lay_segment(&mut self, (start_x, start_y): (i32, i32), action: Action) -> (i32, i32) {
        match action {
            Action { direction: Direction::UP, steps} => {
                let end_y = start_y + steps as i32;
                for y in start_y..end_y {
                    *self.hit_counts.entry((start_x, y + 1)).or_insert(0) += 1;
                }
                (start_x, end_y)
            },
            Action { direction: Direction::DOWN, steps} => {
                let end_y = start_y - steps as i32;
                for y in end_y..start_y {
                    *self.hit_counts.entry((start_x, y - 1)).or_insert(0) += 1;
                }
                (start_x, end_y)
            },
            Action { direction: Direction::RIGHT, steps} => {
                let end_x = start_x + steps as i32;
                for x in start_x..end_x {
                    *self.hit_counts.entry((x + 1, start_y)).or_insert(0) += 1;
                }
                (end_x, start_y)
            },
            Action { direction: Direction::LEFT, steps} => {
                let end_x = start_x - steps as i32;
                for x in end_x..start_x {
                    *self.hit_counts.entry((x - 1, start_y)).or_insert(0) += 1;
                }
                (end_x, start_y)
            },
        }
    }

    fn find_closest_intersection(&self) -> u32 {
        self
            .hit_counts
            .iter()
            .filter(|(_, &value)| value > 1)
            .map(|(&key, _)| key)
            .map(|(x, y)| (x.abs() + y.abs()) as u32)
            .min()
            .expect("expected there to be an intersection")
    }
}

fn process(input: &str) -> impl ToString {
    let wire_actions: Vec<Vec<Action>> = input
        .lines()
        .map(|line| line.trim().split(',').map(|raw| {
            let direction = match raw.chars().nth(0).expect("expected a non-empty string") {
                'u' | 'U' => Direction::UP,
                'd' | 'D' => Direction::DOWN,
                'r' | 'R' => Direction::RIGHT,
                'l' | 'L' => Direction::LEFT,
                _ => panic!("unexpected direction")
            };
            let steps = raw.chars().skip(1).collect::<String>().parse::<u32>().expect("expected a valid positive integer");

            Action { direction, steps }
        }).collect())
        .collect();

    let mut grid = Grid::new();
    for actions in wire_actions {
        let mut cursor = (0, 0);
        for action in actions {
            cursor = grid.lay_segment(cursor, action);
        }
    }

    grid.find_closest_intersection()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(process("R8,U5,L5,D3\nU7,R6,D4,L4").to_string(), "6");
        assert_eq!(process("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").to_string(), "159");
        assert_eq!(process("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7").to_string(), "135");
    }
}
