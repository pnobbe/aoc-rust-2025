advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            let digits: Vec<u64> = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u64)
                .collect();

            let (max_left_idx, &max_left) = digits[..digits.len() - 1]
                .iter()
                .enumerate()
                .max_by(|&(i1, d1), &(i2, d2)| d1.cmp(d2).then(i2.cmp(&i1)))
                .unwrap();

            let max_right = digits[max_left_idx + 1..].iter().max().unwrap();

            max_left * 10 + max_right
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            let digits: Vec<u64> = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u64)
                .collect();

            let mut sequence = vec![];
            let batteries_to_find = 12;
            let mut start_index = 0;
            for batteries_remaining in (1..=batteries_to_find).rev() {
                let end_index = digits.len() - batteries_remaining + 1;
                let (max_left_idx, &max_left) = digits[start_index..end_index]
                    .iter()
                    .enumerate()
                    .max_by(|&(i1, d1), &(i2, d2)| d1.cmp(d2).then(i2.cmp(&i1)))
                    .unwrap();

                sequence.push(max_left);
                start_index += max_left_idx + 1;
            }

            sequence.iter().fold(0, |acc, &x| acc * 10 + x)
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
