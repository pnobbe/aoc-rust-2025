advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    let mut position: i32 = 50;
    for line in input.lines() {
        let (direction, magnitude_str) = line.split_at(1);
        let magnitude: i32 = magnitude_str.parse().ok()?;

        if direction == "R" {
            position += magnitude;
        } else {
            position -= magnitude;
        }

        position = position.rem_euclid(100);

        if position == 0 {
            count += 1;
        }
    }
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count = 0u64;
    let mut position: i32 = 50;

    for line in input.lines() {
        let (direction, magnitude_str) = line.split_at(1);
        let magnitude: i32 = magnitude_str.parse().ok()?;

        let target = if direction == "R" {
            position + magnitude
        } else {
            position - magnitude
        };

        if direction == "R" {
            // For 'R', integer division naturally counts multiples of 100 reached
            count += (target / 100) as u64;
        } else {
            // For 'L', we count full 100s in the magnitude, plus 1 if we
            // crossed/hit 0 (unless we started exactly at 0)
            let crossed_zero = (position > 0 && target <= 0) as i32;
            count += (magnitude / 100 + crossed_zero) as u64;
        }

        position = target.rem_euclid(100);
    }
    Some(count)
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
        assert_eq!(result, Some(6));
    }
}
