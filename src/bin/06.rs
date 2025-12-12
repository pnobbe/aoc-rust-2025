advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let operators: Vec<&str> = lines.last()?.split_whitespace().collect();

    // calculate sum/product for each column
    let mut sums = vec![0u64; operators.len()];
    let mut products = vec![1u64; operators.len()];

    lines
        .iter()
        .take_while(|line| !line.contains('+'))
        .for_each(|line| {
            line.split_whitespace().enumerate().for_each(|(i, num)| {
                let n = num.parse::<u64>().unwrap();
                sums[i] += n;
                products[i] *= n;
            });
        });

    // select sum or product based on operator
    let result = operators
        .iter()
        .enumerate()
        .map(|(i, &op)| match op {
            "+" => sums[i],
            "*" => products[i],
            _ => 0,
        })
        .sum::<u64>();

    Some(result)
}

struct Problem {
    numbers: Vec<u64>,
    operator: char,
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    
    if lines.is_empty() {
        return None;
    }
    
    let width = lines[0].len();
    let height = lines.len();

    // Transpose efficiently by building each column directly
    let mut transposed: Vec<String> = vec![String::with_capacity(height); width];
    
    for line in &lines {
        for (col_idx, ch) in line.chars().enumerate() {
            transposed[col_idx].push(ch);
        }
    }

    // parse problems by reading vertically (column by column)
    let (problems, _, _) = transposed
        .iter()
        .chain(std::iter::once(&String::new()))
        .fold(
            (Vec::new(), Vec::new(), '+'),
            |(mut problems, mut current_columns, mut cur_operator): (
                Vec<Problem>,
                Vec<u64>,
                char,
            ),
             line| {
                if line.trim().is_empty() {
                    // end of current problem
                    if !current_columns.is_empty() {
                        problems.push(Problem {
                            numbers: current_columns,
                            operator: cur_operator,
                        });
                        current_columns = Vec::new();
                    }
                } else {
                    // add the current line as a new column
                    let mut chars: Vec<char> = line.chars().collect();
                    let operator = chars.remove(chars.len() - 1);
                    if operator == '+' || operator == '*' {
                        cur_operator = operator;
                    }
                    let str = chars.iter().collect::<String>();
                    current_columns.push(str.trim().parse::<u64>().unwrap());
                }
                (problems, current_columns, cur_operator)
            },
        );

    let result = problems
        .into_iter()
        .map(|problem| match problem.operator {
            '+' => problem.numbers.iter().sum(),
            '*' => problem.numbers.iter().product(),
            _ => 0,
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
