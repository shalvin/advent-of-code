advent_of_code::solution!(7);

type Terms = Vec<u32>;

struct Equation {
    answer: u64,
    terms: Terms,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (result_str, terms_str) = line.split_once(':').unwrap();
            let result = result_str.parse::<u64>().unwrap();
            let terms: Terms = terms_str
                .split_whitespace()
                .map(|t| t.parse::<u32>().unwrap())
                .collect();

            Equation {
                answer: result,
                terms,
            }
        })
        .collect()
}

fn check_equation(equation: &Equation) -> bool {
    for i in 0..2u64.pow(equation.terms.len() as u32) {
        let calculated_result = equation
            .terms
            .iter()
            .enumerate()
            .fold(0u64, |acc, (j, term)| match i & 1 << j {
                0 => acc.saturating_add(*term as u64),
                _ => acc.saturating_mul(*term as u64),
            });

        if calculated_result == equation.answer {
            return true;
        }
    }

    false
}

fn check_equation_3(equation: &Equation) -> bool {
    for i in 0..=3u64.pow(equation.terms.len() as u32 - 1) {
        let mut terms_iter = equation.terms.iter().enumerate();
        let (_, inital_val) = terms_iter.next().unwrap();

        let calculated_result = terms_iter.fold(*inital_val as u64, |acc, (j, term)| {
            let j = j as u32;
            let term = *term as u64;

            match (i / 3u64.pow(j - 1)) % 3 {
                0 => acc.saturating_add(term),
                1 => acc.saturating_mul(term),
                _ => acc
                    .saturating_mul(10u64.pow(term.ilog10() + 1))
                    .saturating_add(term),
            }
        });

        if calculated_result == equation.answer {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    let mut total = 0;

    for eq in equations {
        if check_equation(&eq) {
            total += eq.answer;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let mut total = 0;

    for eq in equations {
        if check_equation_3(&eq) {
            total += eq.answer;
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
