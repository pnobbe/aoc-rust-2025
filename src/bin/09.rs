use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(9);

fn parse_tiles(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');
            let x = parts.next()?.parse().ok()?;
            let y = parts.next()?.parse().ok()?;
            Some((x, y))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let tiles = parse_tiles(input);
    let n = tiles.len();

    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            let width = x2.abs_diff(x1) + 1;
            let height = y2.abs_diff(y1) + 1;
            let area = width * height;

            max_area = max_area.max(area);
        }
    }

    Some(max_area)
}

// Compress coordinates to reduce grid size
fn compress_coordinates(tiles: &[(u64, u64)]) -> (HashMap<u64, usize>, HashMap<u64, usize>) {
    let mut x_coords: Vec<u64> = tiles.iter().map(|&(x, _)| x).collect();
    let mut y_coords: Vec<u64> = tiles.iter().map(|&(_, y)| y).collect();

    // Add boundary coordinates
    x_coords.push(u64::MIN);
    x_coords.push(u64::MAX);
    y_coords.push(u64::MIN);
    y_coords.push(u64::MAX);

    x_coords.sort_unstable();
    x_coords.dedup();
    y_coords.sort_unstable();
    y_coords.dedup();

    let x_map: HashMap<u64, usize> = x_coords.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let y_map: HashMap<u64, usize> = y_coords.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    (x_map, y_map)
}

// Mark polygon edges on the grid
fn mark_polygon_edges(
    compressed_tiles: &[(usize, usize)],
    width: usize,
    height: usize,
) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; width]; height];
    let n = compressed_tiles.len();

    for i in 0..n {
        let (x1, y1) = compressed_tiles[i];
        let (x2, y2) = compressed_tiles[(i + 1) % n];

        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                grid[y][x] = true;
            }
        }
    }

    grid
}

// Flood fill to mark outside cells
fn flood_fill_outside(grid: &mut Vec<Vec<i32>>, inside_grid: &[Vec<bool>]) {
    let height = grid.len();
    let width = grid[0].len();
    let mut queue = VecDeque::new();

    // Start from origin (0, 0) which is guaranteed to be outside
    queue.push_back((0, 0));
    grid[0][0] = 0; // Mark as outside

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((y, x)) = queue.pop_front() {
        for &(dy, dx) in &directions {
            let ny = (y as isize + dy) as usize;
            let nx = (x as isize + dx) as usize;

            if ny < height && nx < width && grid[ny][nx] == 2 && !inside_grid[ny][nx] {
                grid[ny][nx] = 0; // Mark as outside
                queue.push_back((ny, nx));
            }
        }
    }
}

// Build 2D prefix sum array
fn build_prefix_sum(grid: &mut Vec<Vec<i32>>) {
    let height = grid.len();
    let width = grid[0].len();

    for y in 1..height {
        for x in 1..width {
            let value = if grid[y][x] == 0 { 0 } else { 1 };
            grid[y][x] = value + grid[y - 1][x] + grid[y][x - 1] - grid[y - 1][x - 1];
        }
    }

    // Handle first row and column
    for x in 1..width {
        let value = if grid[0][x] == 0 { 0 } else { 1 };
        grid[0][x] = value + grid[0][x - 1];
    }

    for y in 1..height {
        let value = if grid[y][0] == 0 { 0 } else { 1 };
        grid[y][0] = value + grid[y - 1][0];
    }

    grid[0][0] = if grid[0][0] == 0 { 0 } else { 1 };
}

// Query rectangle sum using prefix sum array
fn query_rectangle_sum(grid: &[Vec<i32>], x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
    let result = grid[y2][x2];

    let left = if x1 > 0 { grid[y2][x1 - 1] } else { 0 };
    let top = if y1 > 0 { grid[y1 - 1][x2] } else { 0 };
    let top_left = if x1 > 0 && y1 > 0 { grid[y1 - 1][x1 - 1] } else { 0 };

    result - left - top + top_left
}

pub fn part_two(input: &str) -> Option<u64> {
    let tiles = parse_tiles(input);
    let n = tiles.len();

    // Compress coordinates
    let (x_map, y_map) = compress_coordinates(&tiles);
    let compressed_tiles: Vec<(usize, usize)> = tiles
        .iter()
        .map(|&(x, y)| (*x_map.get(&x).unwrap(), *y_map.get(&y).unwrap()))
        .collect();

    let width = x_map.len();
    let height = y_map.len();

    // Mark polygon edges
    let inside_grid = mark_polygon_edges(&compressed_tiles, width, height);

    // Create grid: 0 = outside, 1 = inside, 2 = unknown
    let mut grid = vec![vec![2; width]; height];
    for y in 0..height {
        for x in 0..width {
            if inside_grid[y][x] {
                grid[y][x] = 1;
            }
        }
    }

    // Flood fill to mark outside cells
    flood_fill_outside(&mut grid, &inside_grid);

    // All remaining cells (value 2) are inside
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 2 {
                grid[y][x] = 1;
            }
        }
    }

    // Build prefix sum array
    build_prefix_sum(&mut grid);

    // Check all tile pairs
    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (cx1, cy1) = compressed_tiles[i];
            let (cx2, cy2) = compressed_tiles[j];

            let min_x = cx1.min(cx2);
            let max_x = cx1.max(cx2);
            let min_y = cy1.min(cy2);
            let max_y = cy1.max(cy2);

            let expected = ((max_x - min_x + 1) * (max_y - min_y + 1)) as i32;
            let actual = query_rectangle_sum(&grid, min_x, min_y, max_x, max_y);

            if expected == actual {
                let (x1, y1) = tiles[i];
                let (x2, y2) = tiles[j];
                let width = x2.abs_diff(x1) + 1;
                let height = y2.abs_diff(y1) + 1;
                let area = width * height;
                max_area = max_area.max(area);
            }
        }
    }

    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
