advent_of_code::solution!(2);

pub fn check_safety(level: &Vec<u32>) -> bool {
    let rule1 = level.windows(2).all(|w| w[0] > w[1]) || level.windows(2).all(|w| w[0] < w[1]);
    let rule2 = level
        .windows(2)
        .all(|w| w[0].abs_diff(w[1]) <= 3 && w[0].abs_diff(w[1]) >= 1);
    rule1 && rule2
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let mut levels: Vec<Vec<u32>> = vec![];

    lines.into_iter().filter(|&x| !x.is_empty()).for_each(|f: &str| {
        levels.push(
            f.split_whitespace()
                .map(|i| i.parse::<u32>().unwrap())
                .collect(),
        );
    });
    let mut safe_levels: u32 = 0;
    for level in &levels {
        if check_safety(level) {
            safe_levels += 1;
        }
    }
    Some(safe_levels)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let mut levels: Vec<Vec<u32>> = vec![];

    lines.into_iter().filter(|&x| !x.is_empty()).for_each(|f: &str| {
        levels.push(
            f.split_whitespace()
                .map(|i| i.parse::<u32>().unwrap())
                .collect(),
        );
    });
    let mut safe_levels: u32 = 0;
    for level in &levels {
        if check_safety(level)
        {
            safe_levels += 1;
        }
        else {
            for index in 0..level.len() {
                let mut duplicated_level = level.clone();
                duplicated_level.remove(index);
                if check_safety(&duplicated_level) {
                    safe_levels += 1;
                    break;
                }
            }
        }
    }
    Some(safe_levels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
