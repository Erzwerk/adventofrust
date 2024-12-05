use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = reg.captures_iter(input).map(|caps| {
        caps[1].parse::<u32>().unwrap() * caps[2].parse::<u32>().unwrap()
    }).sum();
    return Some(sum);

}

pub fn part_two(input: &str) -> Option<u32> {
    let reg = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut is_legal = true;
    let mut sum:u32 = 0;
    for i in reg.captures_iter(input) {
        match &i[0] {
            "do()" => is_legal = true,
            "don't()" => is_legal = false,
            _ => if is_legal { sum += i[1].parse::<u32>().unwrap() * i[2].parse::<u32>().unwrap() },
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
