advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn circuit_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = vec![0; n];
        for i in 0..n {
            let root = self.find(i);
            sizes[root] = self.size[root];
        }
        sizes.into_iter().filter(|&s| s > 0).collect()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<Point> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            Some(Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            })
        })
        .collect();

    let n = points.len();
    let mut distances = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            distances.push((points[i].distance_squared(&points[j]), i, j));
        }
    }

    distances.sort_unstable_by_key(|&(d, _, _)| d);

    let mut uf = UnionFind::new(n);
    let connections = if n == 20 { 10 } else { 1000 };

    for &(_, i, j) in distances.iter().take(connections) {
        uf.union(i, j);
    }

    let mut sizes = uf.circuit_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let result = sizes.iter().take(3).product::<usize>() as u64;
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<Point> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            if parts.len() == 3 {
                Some(Point {
                    x: parts[0],
                    y: parts[1],
                    z: parts[2],
                })
            } else {
                None
            }
        })
        .collect();

    let n = points.len();
    let mut distances = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            distances.push((points[i].distance_squared(&points[j]), i, j));
        }
    }

    distances.sort_unstable_by_key(|&(d, _, _)| d);

    let mut uf = UnionFind::new(n);
    let mut circuits = n;

    for &(_, i, j) in &distances {
        if uf.union(i, j) {
            circuits -= 1;
            if circuits == 1 {
                let result = (points[i].x as u64) * (points[j].x as u64);
                return Some(result);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
