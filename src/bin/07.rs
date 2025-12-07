use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let start_pos = lines.first()?.find('S')?;
    let width = lines.first()?.len();

    let split_count = lines
        .iter()
        .skip(1)
        .fold((HashMap::from([(start_pos, 1)]), 0u64), |(positions, count), line| {
            let line_bytes = line.as_bytes();
            let mut next_positions = HashMap::new();
            let mut splits = 0u64;

            for pos in positions.keys() {
                if line_bytes[*pos] == b'^' {
                    splits += 1;
                    if *pos > 0 {
                        next_positions.insert(pos - 1, 1);
                    }
                    if pos + 1 < width {
                        next_positions.insert(pos + 1, 1);
                    }
                } else {
                    next_positions.insert(*pos, 1);
                }
            }

            (next_positions, count + splits)
        })
        .1;

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let start_pos = lines.first()?.find('S')?;
    let width = lines.first()?.len();

    let timeline_count = lines
        .iter()
        .skip(1)
        .fold(HashMap::from([(start_pos, 1u64)]), |positions, line| {
            let line_bytes = line.as_bytes();
            let mut next_positions = HashMap::new();

            for (pos, count) in positions {
                if line_bytes[pos] == b'^' {
                    if pos > 0 {
                        *next_positions.entry(pos - 1).or_insert(0) += count;
                    }
                    if pos + 1 < width {
                        *next_positions.entry(pos + 1).or_insert(0) += count;
                    }
                } else {
                    *next_positions.entry(pos).or_insert(0) += count;
                }
            }

            next_positions
        })
        .values()
        .sum();

    Some(timeline_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
