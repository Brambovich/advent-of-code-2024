advent_of_code::solution!(7);

use itertools::Itertools;

#[derive(Debug)]
struct TestCase {
    test_value: u64,
    inputs: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq)]
enum Operators {
    ADD = 0,
    MUL = 1,
    CONCAT = 2,
}

fn apply_operator(a: u64, b: u64, op: &Operators) -> Option<u64>
{
    if *op == Operators::MUL {
        match a.checked_mul(b) {
            Some(val) => {return Some(val);},
            None => {return None},
        }
    }
    else if *op == Operators::ADD {
        match a.checked_add(b) {
            Some(val) => {return Some(val);},
            None => {return None},
        }
    }
    else if *op == Operators::CONCAT {
        let mut a_s = a.to_string();
        let b_s = b.to_string();
        a_s.push_str(&b_s);
        match a_s.parse::<u64>() {
            Ok(val) => {return Some(val)},
            Err(_) => {return None;},
        }
    }
    else {
        return None;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.split('\n');
    let mut test_cases: Vec<TestCase> = vec![];

    //parse
    lines
        .into_iter()
        .filter(|&x| !x.is_empty())
        .for_each(|f: &str| {
            let input: Vec<&str> = f.split(':').collect::<Vec<&str>>();
            let new_test_case: TestCase = TestCase {
                test_value: input[0].parse().unwrap(),
                inputs: input[1]
                    .split_whitespace()
                    .into_iter()
                    .map(|f| f.parse().unwrap())
                    .collect(),
            };
            test_cases.push(new_test_case);
        });

    //solve
    let operators = [Operators::ADD, Operators::MUL];
    let mut total_value: u64 = 0;
    for test_case in test_cases {
        let test_case_length = test_case.inputs.len();
        for operator_options in (0..test_case_length-1).map(|_| operators.iter()).multi_cartesian_product() {
            let mut test_case_result: u64 = test_case.inputs[0];
            for (index, &value) in test_case.inputs[1..].iter().enumerate(){ 
                test_case_result = apply_operator(test_case_result, value, operator_options[index]).unwrap();
            }
            if test_case_result == test_case.test_value {
                total_value += test_case.test_value;
                break;
            }
        }
    }
    Some(total_value)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.split('\n');
    let mut test_cases: Vec<TestCase> = vec![];

    //parse
    lines
        .into_iter()
        .filter(|&x| !x.is_empty())
        .for_each(|f: &str| {
            let input: Vec<&str> = f.split(':').collect::<Vec<&str>>();
            let new_test_case: TestCase = TestCase {
                test_value: input[0].parse().unwrap(),
                inputs: input[1]
                    .split_whitespace()
                    .into_iter()
                    .map(|f| f.parse().unwrap())
                    .collect(),
            };
            test_cases.push(new_test_case);
        });

    //solve
    let operators = [Operators::ADD, Operators::MUL, Operators::CONCAT];
    let mut total_value: u64 = 0;
    for test_case in test_cases {
        let test_case_length = test_case.inputs.len();
        for operator_options in (0..test_case_length-1).map(|_| operators.iter()).multi_cartesian_product() {
            let mut test_case_result: u64 = test_case.inputs[0];
            for (index, &value) in test_case.inputs[1..].iter().enumerate(){ 
                match apply_operator(test_case_result, value, operator_options[index]) {
                    Some(val) => {test_case_result = val;},
                    None => {continue;},
                }
            }
            if test_case_result == test_case.test_value {
                total_value += test_case.test_value;
                break;
            }
        }
    }
    Some(total_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
