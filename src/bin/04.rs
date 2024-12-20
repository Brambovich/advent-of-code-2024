advent_of_code::solution!(4);

use diagonal::diagonal_pos_neg;
use diagonal::diagonal_pos_pos;
use grid::Grid;

pub fn check_chars(input: &[char]) -> bool {
    let xmas_bool = input[0] == 'X' && input[1] == 'M' && input[2] == 'A' && input[3] == 'S';
    let samx_bool = input[3] == 'X' && input[2] == 'M' && input[1] == 'A' && input[0] == 'S';
    xmas_bool || samx_bool
}

pub fn check_chars_vec(input: Vec<&char>) -> bool {
    let xmas_bool = *input[0] == 'X' && *input[1] == 'M' && *input[2] == 'A' && *input[3] == 'S';
    let samx_bool = *input[3] == 'X' && *input[2] == 'M' && *input[1] == 'A' && *input[0] == 'S';
    xmas_bool || samx_bool
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut levels: Vec<Vec<char>> = vec![];

    let lines = input.split('\n');
    lines
        .into_iter()
        .filter(|&x| !x.is_empty())
        .for_each(|f: &str| {
            levels.push(f.chars().collect::<Vec<_>>());
        });

    let mut total_xmas_count = 0;
    for row in &levels {
        total_xmas_count += row.windows(4).filter(|&f| check_chars(f)).count() as u32;
    }
    for index in 0..levels[0].len() {
        let b: Vec<char> = levels.iter().map(|x| x[index]).collect();
        total_xmas_count += b.windows(4).filter(|&f| check_chars(f)).count() as u32;
    }

    for row in diagonal_pos_pos(&levels) {
        total_xmas_count += row
            .windows(4)
            .filter(|&f| check_chars_vec(Vec::from(f)))
            .count() as u32;
    }
    for row in diagonal_pos_neg(&levels) {
        total_xmas_count += row
            .windows(4)
            .filter(|&f| check_chars_vec(Vec::from(f)))
            .count() as u32;
    }

    Some(total_xmas_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut levels: Vec<Vec<char>> = vec![];
    let lines = input.split('\n');
    lines
        .into_iter()
        .filter(|&x| !x.is_empty())
        .for_each(|f: &str| {
            levels.push(f.chars().collect::<Vec<_>>());
        });

    let grid_input = Grid::from(&levels);
    let a = grid_input
        .indexed_iter()
        .filter(|((x, y), &t)| {
            let mut x1 = false;
            let mut x2 = false;
            if *x > 0 && *y > 0 {
                x1 = {
                    grid_input.get(x + 1, y + 1).is_some_and(|&f| f == 'M')
                        && grid_input.get(x - 1, y - 1).is_some_and(|&f| f == 'S')
                        || grid_input.get(x + 1, y + 1).is_some_and(|&f| f == 'S')
                            && grid_input.get(x - 1, y - 1).is_some_and(|&f| f == 'M')
                };
                x2 = {
                    grid_input.get(x + 1, y - 1).is_some_and(|&f| f == 'M')
                        && grid_input.get(x - 1, y + 1).is_some_and(|&f| f == 'S')
                        || grid_input.get(x + 1, y - 1).is_some_and(|&f| f == 'S')
                            && grid_input.get(x - 1, y + 1).is_some_and(|&f| f == 'M')
                };
            }
            t == 'A' && x1 && x2
        })
        .count() as u32;
    Some(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
