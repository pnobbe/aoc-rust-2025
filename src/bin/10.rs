advent_of_code::solution!(10);

use good_lp::{
    default_solver, variable, variables, Expression, IntoAffineExpression, Solution, SolverModel,
    Variable,
};

fn parse_buttons(line: &str) -> Vec<Vec<usize>> {
    line.split_whitespace()
        .filter(|s| s.starts_with('('))
        .map(|s| {
            s.trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve_with_ilp(targets: &[u64], buttons: &[Vec<usize>]) -> u64 {
    let mut vars = variables!();
    let press_vars: Vec<Variable> = (0..buttons.len())
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    let objective: Expression = press_vars.iter().map(|&v| Expression::from(v)).sum();
    let mut problem = vars.minimise(objective).using(default_solver);

    // each counter must reach its target
    for (i, &target) in targets.iter().enumerate() {
        let constraint: Expression = buttons
            .iter()
            .zip(&press_vars)
            .filter(|(btn, _)| btn.contains(&i))
            .map(|(_, &var)| var.into_expression())
            .sum();
        problem.add_constraint(constraint.eq(target as i32));
    }

    match problem.solve() {
        Ok(sol) => press_vars.iter().map(|&v| sol.value(v).round() as u64).sum(),
        Err(_) => panic!("No solution found"),
    }
}

fn solve_with_bitmask(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    use std::collections::{VecDeque, HashSet};

    let n = target.len();
    let initial_state = vec![false; n];

    // Quick check if already at target
    if initial_state == target {
        return 0;
    }

    // BFS to find minimum button presses
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Convert state to u64 for faster hashing (lights are typically < 64)
    let state_to_key = |state: &[bool]| -> u64 {
        state.iter().enumerate().fold(0u64, |acc, (i, &b)| {
            acc | ((b as u64) << i)
        })
    };

    let target_key = state_to_key(target);
    queue.push_back((initial_state, 0));
    visited.insert(0u64);

    while let Some((state, presses)) = queue.pop_front() {
        // Try pressing each button
        for button in buttons {
            let mut new_state = state.clone();
            for &light in button {
                new_state[light] = !new_state[light];
            }

            let key = state_to_key(&new_state);

            if key == target_key {
                return presses + 1;
            }

            if !visited.contains(&key) {
                visited.insert(key);
                queue.push_back((new_state, presses + 1));
            }
        }
    }

    // Fallback to original brute force if BFS fails (shouldn't happen)
    (0..(1 << buttons.len()))
        .filter_map(|mask| {
            let mut state = vec![false; n];
            let mut presses = 0;

            for (i, button) in buttons.iter().enumerate() {
                if (mask >> i) & 1 == 1 {
                    presses += 1;
                    button.iter().for_each(|&light| state[light] = !state[light]);
                }
            }

            (state == target).then_some(presses)
        })
        .min()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let total: usize = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let target: Vec<bool> = line
                .split_whitespace()
                .next()
                .unwrap()
                .trim_matches(|c| c == '[' || c == ']')
                .chars()
                .map(|c| c == '#')
                .collect();
            let buttons = parse_buttons(line);
            solve_with_bitmask(&target, &buttons)
        })
        .sum();

    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total: u64 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let targets: Vec<u64> = line
                .split_whitespace()
                .find(|s| s.starts_with('{'))
                .unwrap()
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            let buttons = parse_buttons(line);
            solve_with_ilp(&targets, &buttons)
        })
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
