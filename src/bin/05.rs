advent_of_code::solution!(5);

fn parse_ranges(lines: &mut std::str::Lines) -> Vec<(u64, u64)> {
    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            Some((start.parse().ok()?, end.parse().ok()?))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let ranges = parse_ranges(&mut lines);

    let fresh_count = lines
        .filter_map(|line| line.parse::<u64>().ok())
        .filter(|&id| ranges.iter().any(|(start, end)| id >= *start && id <= *end))
        .count();

    Some(fresh_count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut ranges = parse_ranges(&mut lines);

    ranges.sort_unstable_by_key(|&(start, _)| start);

    // Merge overlapping ranges
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged_ranges.push((start, end));
    }

    // Sum the sizes of merged ranges
    let total: u64 = merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
