advent_of_code::solution!(2);

const POWERS_OF_10: [i64; 19] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
];

pub fn part_one(input: &str) -> Option<u64> {
    let sum: i64 = input
        .split(',')
        .flat_map(|range_str| {
            let (left, right) = range_str.split_once('-').expect("Invalid range format");
            let start = left.parse::<i64>().unwrap();
            let end = right.parse::<i64>().unwrap();
            start..=end
        })
        .filter(|&id| {
            let digit_count = (id.ilog10() + 1) as usize;
            if digit_count % 2 != 0 {
                return false;
            }
            let divisor = POWERS_OF_10[digit_count / 2];
            let left_half = id / divisor;
            let right_half = id % divisor;
            left_half == right_half
        })
        .sum();

    Some(sum as u64)
}

fn has_repeating_pattern(n: i64) -> bool {
    let digit_count = (n.ilog10() + 1) as usize;

    for chunk_size in 1..=digit_count / 2 {
        if digit_count % chunk_size != 0 {
            continue;
        }

        let divisor = POWERS_OF_10[chunk_size];
        let num_chunks = digit_count / chunk_size;
        let first_chunk = n % divisor;

        let mut temp = n / divisor;
        let mut all_match = true;

        for _ in 1..num_chunks {
            if temp % divisor != first_chunk {
                all_match = false;
                break;
            }
            temp /= divisor;
        }

        if all_match {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum: i64 = input
        .split(',')
        .flat_map(|range_str| {
            let (left, right) = range_str.split_once('-').expect("Invalid range format");
            let start = left.parse::<i64>().unwrap();
            let end = right.parse::<i64>().unwrap();
            start..=end
        })
        .filter(|&id| has_repeating_pattern(id))
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
