advent_of_code::solution!(5);

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let mut rules: Vec<Rule> = vec![];
    let mut updates: Vec<Vec<u32>> = vec![];

    //parsing
    for line in lines {
        if line.find('|').is_some() {
            let values: Vec<&str> = line.split("|").collect();
            if values.len() == 2 {
                rules.push(Rule {
                    before: values.get(0)?.parse::<u32>().unwrap(),
                    after: values.get(1)?.parse::<u32>().unwrap(),
                });
            }
        } else if !line.is_empty() {
            let values: Vec<u32> = line.split(",").map(|f| f.parse::<u32>().unwrap()).collect();
            updates.push(values);
        }
    }
    //solving
    let mut correct_update_total: u32 = 0;
    for update in updates {
        let mut correct_update = true;
        for rule in &rules {
            let present_index = update.iter().position(|&f| f == rule.after);
            match present_index {
                Some(index) => {
                    correct_update = update[index..].iter().all(|&f| f != rule.before);
                    if correct_update == false {
                        break;
                    }
                }
                None => continue,
            }
        }
        if correct_update {
            let index = (update.len() as f32 / 2 as f32).floor() as usize;
            correct_update_total += update[index];
        }
    }
    Some(correct_update_total)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
