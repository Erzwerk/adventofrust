advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let inputs = input.split('\n').map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
        // .map(|l| {
        //     l.chars()
        //     .collect::<Vec<_>>()
        // })
        // .collect::<Vec<Vec<_>>>();
    let mut x_pairs:Vec<(i32, i32)> = Vec::new();
    for z in 0.. inputs.len() {
        for i in 0..inputs[z].len() {
            if inputs[z][i] == 'X' {
                x_pairs.push((z as i32, i as i32));
            };
        }
    }
    let right = right_test(&inputs, &x_pairs);
    let left = left_test(&inputs, &x_pairs);
    let desc = desc_test(&inputs, &x_pairs);
    let asc = asc_test(&inputs, &x_pairs);
    println!("{}",right+left+desc+asc);

    Some((right + left + desc + asc) as u32)
}
fn right_test(inputs: &Vec<Vec<char>>,x_pairs: &Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;
    for i in 0..x_pairs.len()-1 {
        if x_pairs[i].0+1 == x_pairs[i+1].0 && x_pairs[i].1+1 == x_pairs[i+1].1 {
            continue;
        } else {
            let mut index_1 = x_pairs[i].0;
            let mut index_2 = x_pairs[i].1;
            if index_2 + 3 > inputs[0].len() as i32 -1 {
                continue;
            }
            let mut j = 1;

            if inputs[index_1 as usize][index_2 as usize + j] != 'M' {
                continue;
            }
            while  (inputs[index_1 as usize][index_2 as usize + j] == 'M') && (index_2 + (j as i32) + 2 < inputs[0].len() as i32 -1){
                j += 1;
            }
            if inputs[index_1 as usize][(index_2)as usize +j] == 'M' {
                continue;
            }
            if inputs[index_1 as usize][index_2 as usize+j] != 'A' {
                continue;
            }
            j+1;
            while  (inputs[index_1 as usize][index_2 as usize + j] == 'A') && (index_2 + (j as i32) + 1 < inputs[0].len() as i32 -1){
                j += 1;
            }
            if inputs[index_1 as usize][(index_2)as usize +j] == 'A' {
                continue;
            }
            if inputs[index_1 as usize][index_2 as usize + j] == 'S' {
                sum+=1;
            }

        }
    }
    return sum;
}
fn left_test(inputs: &Vec<Vec<char>>,x_pairs: &Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;
    for i in (1..x_pairs.len()).rev() {
        if x_pairs[i].0-1 == x_pairs[i-1].0 && x_pairs[i].1-1 == x_pairs[i-1].1 {
            continue;
        } else {
            let mut index_1 = x_pairs[i].0;
            let mut index_2 = x_pairs[i].1;
            if index_2 -3 < 0 {
                continue;
            }
            let mut j = 1;

            if inputs[index_1 as usize][index_2 as usize - j] != 'M' {
                continue;
            }
            while  (inputs[index_1 as usize][index_2 as usize - j] == 'M') && (index_2 - (j as i32) - 2 > 0){
                j += 1;
            }
            if inputs[index_1 as usize][(index_2)as usize -j] == 'M' {
                continue;
            }
            if inputs[index_1 as usize][index_2 as usize - j] != 'A' {
                continue;
            }
            j+1;
            while  (inputs[index_1 as usize][index_2 as usize - j] == 'A') && (index_2 - (j as i32) - 1 > 0){
                j += 1;
            }
            if inputs[index_1 as usize][(index_2)as usize - j] == 'A' {
                continue;
            }
            if inputs[index_1 as usize][index_2 as usize - j] == 'S' {
                sum+=1;
            }

        }
    }
    return sum;
}
fn desc_test(inputs: &Vec<Vec<char>>,x_pairs: &Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;
    for i in (1..x_pairs.len()).rev() {
        if x_pairs[i].0-1 == x_pairs[i-1].0 && x_pairs[i].1-1 == x_pairs[i-1].1 {
            continue;
        } else {
            let mut index_1 = x_pairs[i].0;
            let mut index_2 = x_pairs[i].1;
            if index_1 -3 < 0 {
                continue;
            }
            let mut j = 1;

            if inputs[index_1 as usize - j][index_2 as usize ] != 'M' {
                continue;
            }
            while  (inputs[index_1 as usize - j][index_2 as usize] == 'M') && (index_1 - (j as i32) - 2 > 0){
                j += 1;
            }
            if inputs[index_1 as usize - j][(index_2)as usize] == 'M' {
                continue;
            }
            if inputs[index_1 as usize - j][index_2 as usize] != 'A' {
                continue;
            }
            j+1;
            while  (inputs[index_1 as usize - j][index_2 as usize] == 'A') && (index_1 - (j as i32) - 1 > 0){
                j += 1;
            }
            if inputs[index_1 as usize - j][(index_2)as usize] == 'A' {
                continue;
            }
            if inputs[index_1 as usize - j][index_2 as usize] == 'S' {
                sum+=1;
            }

        }
    }
    return sum;
}
fn asc_test(inputs: &Vec<Vec<char>>,x_pairs: &Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;
    for i in 0..x_pairs.len()-1 {
        if x_pairs[i].0+1 == x_pairs[i+1].0 && x_pairs[i].1+1 == x_pairs[i+1].1 {
            continue;
        } else {
            let mut index_1 = x_pairs[i].0;
            let mut index_2 = x_pairs[i].1;
            if index_1 + 3 > inputs[0].len() as i32 -1{
                continue;
            }
            let mut j = 1;

            if inputs[index_1 as usize + j][index_2 as usize ] != 'M' {
                continue;
            }
            while  (inputs[index_1 as usize + j][index_2 as usize] == 'M') && (index_1 + (j as i32) +2 < inputs[0].len() as i32 -1){
                j += 1;
            }
            if inputs[index_1 as usize + j][(index_2)as usize] == 'M' {
                continue;
            }
            if inputs[index_1 as usize + j][index_2 as usize] != 'A' {
                continue;
            }
            j+1;
            while  (inputs[index_1 as usize + j][index_2 as usize] == 'A') && (index_1 + (j as i32)  +1 < inputs[0].len() as i32 -1){
                j += 1;
            }
            if inputs[index_1 as usize + j][(index_2)as usize] == 'A' {
                continue;
            }
            if inputs[index_1 as usize + j][index_2 as usize] == 'S' {
                sum+=1;
            }

        }
    }
    return sum;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
