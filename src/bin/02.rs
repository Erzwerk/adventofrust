advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    println!("{}", input);
    // input.split gibt nur den iterator wieder
    // |s| ist eine argument fuer eine lambada function\
    // map(|d| d.parse::<i32>() konvertiert den integer
    // <Vec<&str>>() erste dimension und <Vec<Vec<&str>>>() zweite
    let input = input.trim()
        .split("\n")
        .map(|l| {
            l.split(" ")
                .map(|d| d.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    let mut sum:u32 = 0;
    for z in 0..input.len() {
        let mut safe = true;
        let mut decr= None;
        for i in 1..input[z].len() {
            let check = input[z][i] < input[z][i - 1];
            if (*decr.get_or_insert(check) == check) {
                if(input[z][i] - input[z][i - 1]).abs() > 3 || (input[z][i] - input[z][i - 1]).abs() < 1{
                    safe = false;
                    break;
                }
            } else {
                safe = false;
                break;
            }
        }
        if safe {
            sum += 1;
        }
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.trim()
        .split("\n")
        .map(|l| {
            l.split(" ")
                .map(|d| d.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    let mut sum:u32 = 0;
    for z in 0..input.len() {
        let mut safe = true;
        safe = check_safe(&input[z]);
        if !safe {
            for i in 0..input[z].len() {
                let mut new_vec = input[z].clone();
                new_vec.remove(i);
                if  check_safe(&new_vec){
                    safe = true;
                    break;
                }

            }
        }
        if safe {
            sum += 1;
        }
    }
    return Some(sum);
}

fn check_safe(input:&Vec<i32>) -> bool {
    let mut decr= None;
    for i in 1..input.len() {
        let check = input[i] < input[i - 1];
        if (*decr.get_or_insert(check) == check) {
            if(input[i] - input[i - 1]).abs() > 3 || (input[i] - input[i - 1]).abs() < 1{
                return false;
            }
        } else {
            return false;
        }
    }
    return true;
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
