advent_of_code::solution!(12);

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    required: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Region> {
    let lines: Vec<&str> = input.lines().collect();
    let mut regions = Vec::new();

    for line in lines {
        if line.contains('x') {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let dims: Vec<&str> = parts[0].split('x').collect();
            let width = dims[0].parse().unwrap();
            let height = dims[1].trim_end_matches(':').parse().unwrap();
            let required: Vec<usize> = parts[1..].iter().map(|s| s.parse().unwrap()).collect();
            regions.push(Region { width, height, required });
        }
    }

    regions
}

pub fn part_one(input: &str) -> Option<u64> {
    let regions = parse_input(input);

    let count = regions.iter()
        .filter(|region| {
            let total_presents: usize = region.required.iter().sum();
            total_presents * 8 < region.width * region.height
        })
        .count();

    Some(count as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
