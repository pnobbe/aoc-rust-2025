use std::collections::{HashMap, HashSet};

advent_of_code::solution!(9);

fn parse_tiles(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<u64> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            (parts.len() == 2).then(|| (parts[0], parts[1]))
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

    Some(max_area as u64)
}

#[inline]
fn is_point_in_or_on_polygon(
    point: (i64, i64),
    polygon: &[(i64, i64)],
    vertex_set: &HashSet<(i64, i64)>,
    cache: &mut HashMap<(i64, i64), bool>,
) -> bool {
    if let Some(&result) = cache.get(&point) {
        return result;
    }

    // check if point is a vertex
    if vertex_set.contains(&point) {
        cache.insert(point, true);
        return true;
    }

    let (px, py) = point;
    let n = polygon.len();

    // check if point is on an edge
    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        if x1 == x2 && px == x1 {
            if py >= y1.min(y2) && py <= y1.max(y2) {
                cache.insert(point, true);
                return true;
            }
        } else if y1 == y2 && py == y1 {
            if px >= x1.min(x2) && px <= x1.max(x2) {
                cache.insert(point, true);
                return true;
            }
        }
    }

    // ray cast to check if point is inside polygon
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }

    cache.insert(point, inside);
    inside
}

#[inline]
fn is_rectangle_valid(
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    polygon: &[(i64, i64)],
    vertex_set: &HashSet<(i64, i64)>,
    cache: &mut HashMap<(i64, i64), bool>,
) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    // check the four corners
    if !is_point_in_or_on_polygon((min_x, min_y), polygon, vertex_set, cache)
        || !is_point_in_or_on_polygon((min_x, max_y), polygon, vertex_set, cache)
        || !is_point_in_or_on_polygon((max_x, min_y), polygon, vertex_set, cache)
        || !is_point_in_or_on_polygon((max_x, max_y), polygon, vertex_set, cache)
    {
        return false;
    }

    let width = max_x - min_x;
    let height = max_y - min_y;

    // check top and bottom edges (excluding corners)
    for x in (min_x + 1)..max_x {
        if !is_point_in_or_on_polygon((x, min_y), polygon, vertex_set, cache) {
            return false;
        }
        if height > 0 && !is_point_in_or_on_polygon((x, max_y), polygon, vertex_set, cache) {
            return false;
        }
    }

    // check left and right edges (excluding corners)
    for y in (min_y + 1)..max_y {
        if !is_point_in_or_on_polygon((min_x, y), polygon, vertex_set, cache) {
            return false;
        }
        if width > 0 && !is_point_in_or_on_polygon((max_x, y), polygon, vertex_set, cache) {
            return false;
        }
    }

    // check interior points
    for y in (min_y + 1)..max_y {
        for x in (min_x + 1)..max_x {
            if !is_point_in_or_on_polygon((x, y), polygon, vertex_set, cache) {
                return false;
            }
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let tiles = parse_tiles(input);
    let n = tiles.len();

    // collect unique x, y coordinates
    let x_coords: Vec<u64> = tiles.iter().map(|&(x, _)| x).collect::<HashSet<_>>().into_iter().collect();
    let y_coords: Vec<u64> = tiles.iter().map(|&(_, y)| y).collect::<HashSet<_>>().into_iter().collect();

    let mut x_sorted = x_coords;
    let mut y_sorted = y_coords;
    x_sorted.sort_unstable();
    y_sorted.sort_unstable();

    // create mapping from original to compressed coordinates
    let compress_x = |x: u64| x_sorted.binary_search(&x).unwrap() as i64;
    let compress_y = |y: u64| y_sorted.binary_search(&y).unwrap() as i64;

    // build compressed polygon
    let compressed_polygon: Vec<(i64, i64)> = tiles
        .iter()
        .map(|&(x, y)| (compress_x(x), compress_y(y)))
        .collect();

    // create vertex set for O(1) lookup
    let vertex_set: HashSet<(i64, i64)> = compressed_polygon.iter().copied().collect();

    let mut max_area = 0;

    // pre-allocate cache with reasonable capacity
    let estimated_points = (x_sorted.len() * y_sorted.len()).min(10000);
    let mut cache: HashMap<(i64, i64), bool> = HashMap::with_capacity(estimated_points);

    // sort pairs by decreasing area to find the maximum faster
    let mut pairs: Vec<(usize, usize, u64)> = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let width = tiles[i].0.abs_diff(tiles[j].0) + 1;
            let height = tiles[i].1.abs_diff(tiles[j].1) + 1;
            let area = width * height;
            pairs.push((i, j, area));
        }
    }
    pairs.sort_unstable_by_key(|&(_, _, area)| std::cmp::Reverse(area));

    for &(i, j, area) in &pairs {
        if area <= max_area {
            break; // all remaining pairs have smaller areas
        }

        let (cx1, cy1) = compressed_polygon[i];
        let (cx2, cy2) = compressed_polygon[j];

        // skip if the compressed rectangle is tiny (adjacent coordinates)
        let comp_width = (cx2 - cx1).abs();
        let comp_height = (cy2 - cy1).abs();
        if comp_width == 0 || comp_height == 0 {
            continue;
        }

        // check rectangle validity in compressed space
        if is_rectangle_valid(cx1, cy1, cx2, cy2, &compressed_polygon, &vertex_set, &mut cache) {
            max_area = area;
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
