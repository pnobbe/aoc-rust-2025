advent_of_code::solution!(4);

const ADJACENT_OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_valid_pos(row: isize, col: isize, height: isize, width: isize) -> bool {
    row >= 0 && row < height && col >= 0 && col < width
}

fn count_adjacent_rolls(grid: &[Vec<char>]) -> Vec<Vec<u8>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut count_grid = vec![vec![0u8; width]; height];

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '@' {
                for &(di, dj) in &ADJACENT_OFFSETS {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if is_valid_pos(ni, nj, height as isize, width as isize)
                        && grid[ni as usize][nj as usize] == '@'
                    {
                        count_grid[ni as usize][nj as usize] += 1;
                    }
                }
            }
        }
    }

    count_grid
}

fn count_accessible_rolls(grid: &[Vec<char>], count_grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .zip(count_grid.iter())
        .flat_map(|(g_row, c_row)| g_row.iter().zip(c_row.iter()))
        .filter(|&(&cell, &count)| cell == '@' && count < 4)
        .count()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let count_grid = count_adjacent_rolls(&grid);
    let count = count_accessible_rolls(&grid, &count_grid);
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_grid(input);
    let mut total_removed = 0;

    loop {
        let count_grid = count_adjacent_rolls(&grid);
        let mut removed_this_round = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '@' && count_grid[i][j] < 4 {
                    grid[i][j] = '.';
                    removed_this_round += 1;
                }
            }
        }

        if removed_this_round == 0 {
            break;
        }

        total_removed += removed_this_round;
    }

    Some(total_removed as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
