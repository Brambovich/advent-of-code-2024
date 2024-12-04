advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut total: u32 = 0;
    for multiply in re.captures_iter(input) {
        total += multiply.get(1).unwrap().as_str().parse::<u32>().unwrap()
            * multiply.get(2).unwrap().as_str().parse::<u32>().unwrap();
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|don't\(\)|do\(\)").unwrap();
    let mut total: u32 = 0;
    let mut enabled: bool = true;
    for item in re.captures_iter(input) {
        let operation = &item.get(0).unwrap().as_str()[..4];
        if operation == "don'"
        {
            enabled = false;
        }
        else if operation == "do()"
        {
            enabled = true;
        }
        else if operation == "mul("
        {
            if enabled
            {
                total += item.get(1).unwrap().as_str().parse::<u32>().unwrap()
                    * item.get(2).unwrap().as_str().parse::<u32>().unwrap();
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples",DAY));
        assert_eq!(result, Some(48));
    }
}
