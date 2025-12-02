advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let sum: i64 = input
        .split(',')
        .flat_map(|range_str| {
            let (left, right) = range_str.split_once('-')
                .expect("Invalid range format");
            let start = left.parse::<i64>().unwrap();
            let end = right.parse::<i64>().unwrap();
            start..=end
        })
        .filter(|&id| {
            let digit_count = id.ilog10() + 1;
            if digit_count % 2 != 0 {
                return false;
            }
            let divisor = 10_i64.pow(digit_count / 2);
            let left_half = id / divisor;
            let right_half = id % divisor;
            left_half == right_half
        })
        .sum();

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum: i64 = input
        .split(',')
        .flat_map(|range_str| {
            let (left, right) = range_str.split_once('-')
                .expect("Invalid range format");
            let start = left.parse::<i64>().unwrap();
            let end = right.parse::<i64>().unwrap();
            start..=end
        })
        .filter(|&id| {
            let s = id.to_string();
            let len = s.len();
            (1..=len / 2).any(|chunk_size| {
                if s.len() % chunk_size != 0 {
                    return false;
                }
                let first_chunk = &s[..chunk_size];
                s.as_bytes()
                    .chunks(chunk_size)
                    .all(|chunk| chunk == first_chunk.as_bytes())
            })
        })
        .sum();

    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
