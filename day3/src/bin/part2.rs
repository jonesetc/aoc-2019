const INPUT: &str = include_str!("../../etc/input-part1.txt");

fn main() {
    println!("answer is: {}", process(INPUT).to_string())
}

enum Direction { UP, DOWN, RIGHT, LEFT }
struct Action { pub direction: Direction, pub steps: u32 }
struct Grid { visitors: std::collections::HashMap<(i32, i32), std::collections::HashMap<usize, u32>> }

impl Grid {
    fn new() -> Grid {
        Grid{ visitors: std::collections::HashMap::new() }
    }

    fn lay_segment(&mut self, visitor_id: usize, start_steps: u32, (start_x, start_y): (i32, i32), action: &Action) -> (u32, (i32, i32)) {
        match action {
            Action { direction: Direction::UP, steps} => {
                for step in 1..=*steps {
                    self
                        .visitors
                        .entry((start_x, start_y + step as i32))
                        .or_insert_with(std::collections::HashMap::new)
                        .entry(visitor_id)
                        .or_insert(start_steps + step);
                }
                (start_steps + steps, (start_x, start_y + *steps as i32))
            },
            Action { direction: Direction::DOWN, steps} => {
                for step in 1..=*steps {
                    self
                        .visitors
                        .entry((start_x, start_y - step as i32))
                        .or_insert_with(std::collections::HashMap::new)
                        .entry(visitor_id)
                        .or_insert(start_steps + step);
                }
                (start_steps + steps, (start_x, start_y - *steps as i32))
            },
            Action { direction: Direction::RIGHT, steps} => {
                for step in 1..=*steps {
                    self
                        .visitors
                        .entry((start_x + step as i32, start_y))
                        .or_insert_with(std::collections::HashMap::new)
                        .entry(visitor_id)
                        .or_insert(start_steps + step);
                }
                (start_steps + steps, (start_x + *steps as i32, start_y))
            },
            Action { direction: Direction::LEFT, steps} => {
                for step in 1..=*steps {
                    self
                        .visitors
                        .entry((start_x - step as i32, start_y))
                        .or_insert_with(std::collections::HashMap::new)
                        .entry(visitor_id)
                        .or_insert(start_steps + step);
                }
                (start_steps + steps, (start_x - *steps as i32, start_y))
            },
        }
    }

    fn find_closest_intersection(&self) -> u32 {
        self
            .visitors
            .iter()
            .filter(|(_, value)| value.len() > 1)
            .map(|(_, value)| value.values().sum())
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
    for (i, actions) in wire_actions.iter().enumerate() {
        let (mut steps, mut cursor) = (0, (0, 0));
        for action in actions {
            let updated = grid.lay_segment(i, steps, cursor, action);
            steps = updated.0;
            cursor = updated.1;
        }
    }

    grid.find_closest_intersection()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(process("R8,U5,L5,D3\nU7,R6,D4,L4").to_string(), "30");
        assert_eq!(process("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").to_string(), "610");
        assert_eq!(process("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7").to_string(), "410");
    }
}
