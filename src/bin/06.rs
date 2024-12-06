advent_of_code::solution!(6);
use grid::Grid;
use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy)]
enum Direction {
    #[default]
    North = 0,
    East = 1,
    South= 2,
    West= 3,
}
const DIRECTIONS: [Position; 4] = [
    Position {x:0, y:-1},
    Position {x:1, y:0},
    Position {x:0, y:1},
    Position {x:-1, y:0},
];

#[derive(Debug, Default)]
struct Position {
    x: isize,
    y: isize,
}


pub fn part_one(input: &str) -> Option<u32> {
    //parse
    let mut levels: Vec<Vec<char>> = vec![];
    let lines = input.split('\n');
    lines
        .into_iter()
        .filter(|&x| !x.is_empty())
        .for_each(|f: &str| {
            levels.push(f.chars().collect::<Vec<_>>());
        });

    let grid_input = Grid::from(&levels);

    //solve
    let mut current_location: Position = Position::default();
    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut current_heading: Direction = Direction::North;
    grid_input.indexed_iter().for_each(|((y_l, x_l), &t)| {
        if t == '^' {
            current_location = Position {
                x: x_l as isize,
                y: y_l as isize,
            };
        }
    });
    
    visited_positions.insert((current_location.x, current_location.y));
    loop {
        let next_y = current_location.y+DIRECTIONS[current_heading as usize].y;
        let next_x = current_location.x+DIRECTIONS[current_heading as usize].x;
        match grid_input.get(next_y, next_x) {
            Some(&c) => {
                if c == '.' || c == '^' {
                    current_location.x = next_x;
                    current_location.y = next_y;
                    visited_positions.insert((next_x, next_y));
                }
                else if c == '#' {
                    current_heading = 
                    match current_heading {
                        Direction::North => {Direction::East},
                        Direction::East => {Direction::South},
                        Direction::South => {Direction::West},
                        Direction::West => {Direction::North},
                    };
                }
            },
            None => { 
                break;
            },
        }
    }
    Some(visited_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
