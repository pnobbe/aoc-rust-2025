advent_of_code::solution!(11);

use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        if let Some((device, outputs)) = line.split_once(": ") {
            let connections: Vec<&str> = outputs.split_whitespace().collect();
            graph.insert(device, connections);
        }
    }

    graph
}

fn count_paths_internal<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    target: &'a str,
    required: Option<&[&'a str]>,
    visited_required: Vec<bool>,
    memo: &mut HashMap<(&'a str, Vec<bool>), u64>,
) -> u64 {
    if current == target {
        return if required.is_none() || visited_required.iter().all(|&v| v) {
            1
        } else {
            0
        };
    }

    // check memoization
    let key = (current, visited_required.clone());
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    // if this node has no outputs, no paths
    let Some(neighbors) = graph.get(current) else {
        memo.insert(key, 0);
        return 0;
    };

    let mut total_paths = 0;
    for &neighbor in neighbors {
        // update required nodes state if applicable
        let new_visited = if let Some(req_nodes) = required {
            let mut new_visited = visited_required.clone();
            for (i, &req) in req_nodes.iter().enumerate() {
                if neighbor == req {
                    new_visited[i] = true;
                }
            }
            new_visited
        } else {
            visited_required.clone()
        };

        total_paths += count_paths_internal(graph, neighbor, target, required, new_visited, memo);
    }

    memo.insert(key, total_paths);
    total_paths
}

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    target: &'a str,
    required: Option<&[&'a str]>,
) -> u64 {
    let visited_required = required.map(|r| vec![false; r.len()]).unwrap_or_default();
    let mut memo = HashMap::new();
    count_paths_internal(graph, start, target, required, visited_required, &mut memo)
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_input(input);
    let paths = count_paths(&graph, "you", "out", None);
    Some(paths)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse_input(input);
    let required = vec!["dac", "fft"];
    let paths = count_paths(&graph, "svr", "out", Some(&required));
    Some(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        // part 1 uses a different example than part 2, so this test is skipped
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
