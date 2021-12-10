enum ParseResult {
    Ok,
    Corrupted { illegal: char },
    Incomplete { stack: Vec<char> },
}

impl ParseResult {
    fn score(&self) -> u32 {
        match self {
            Self::Ok | Self::Incomplete { .. } => 0,
            Self::Corrupted { illegal } => match illegal {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Unexpected illegal character: {}", illegal),
            },
        }
    }
}

fn parse(s: String) -> ParseResult {
    let mut stack = vec![];
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' => {
                let prev = stack.pop();
                if prev != Some('(') {
                    return ParseResult::Corrupted { illegal: ch };
                }
            }
            ']' => {
                let prev = stack.pop();
                if prev != Some('[') {
                    return ParseResult::Corrupted { illegal: ch };
                }
            }
            '}' => {
                let prev = stack.pop();
                if prev != Some('{') {
                    return ParseResult::Corrupted { illegal: ch };
                }
            }
            '>' => {
                let prev = stack.pop();
                if prev != Some('<') {
                    return ParseResult::Corrupted { illegal: ch };
                }
            }
            _ => {
                panic!("Unexpected character {}", ch);
            }
        }
    }
    if stack.is_empty() {
        ParseResult::Ok
    } else {
        ParseResult::Incomplete { stack }
    }
}

fn complete(stack: Vec<char>) -> Vec<char> {
    let mut stack = stack;
    let mut completion = vec![];
    while !stack.is_empty() {
        let c = stack.pop().unwrap();
        match c {
            '(' => completion.push(')'),
            '[' => completion.push(']'),
            '<' => completion.push('>'),
            '{' => completion.push('}'),
            _ => {
                panic!("Unexpected character {}", c)
            }
        }
    }
    completion
}

fn score_completion(completion: &[char]) -> u64 {
    let mut score: u64 = 0;
    for c in completion {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => {
                panic!("Unexpected character {}", c)
            }
        };
    }
    score
}

fn main() {
    let input = adventofcode2021::input_lines(10);
    let score: u32 = input.clone().into_iter().map(|s| parse(s).score()).sum();
    println!("Part 1: {}", score);
    let mut p2_scores: Vec<u64> = input
        .into_iter()
        .map(parse)
        .filter_map(|r| match r {
            ParseResult::Incomplete { stack } => Some(stack),
            _ => None,
        })
        .map(complete)
        .map(|comp| score_completion(&comp))
        .collect();
    p2_scores.sort_unstable();
    let mid_index = (p2_scores.len() - 1) / 2;
    let score_p2 = p2_scores[mid_index];
    println!("Part 2: {}", score_p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scoring() {
        assert_eq!(
            score_completion(&("}}]])})]".chars().collect::<Vec<char>>())),
            288957
        );
        assert_eq!(
            score_completion(&("])}>".chars().collect::<Vec<char>>())),
            294
        );
    }
}
