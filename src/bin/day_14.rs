use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Element {
    atom: char,
    next: Option<Rc<RefCell<Element>>>,
}

#[derive(Clone, Debug)]
struct Rule {
    first: char,
    second: char,
    insertion: char,
}

impl Rule {
    fn parse(s: &str) -> Rule {
        let chars: Vec<char> = s.chars().collect();
        assert!(chars.len() == 7);
        Rule {
            first: chars[0],
            second: chars[1],
            insertion: chars[6],
        }
    }
}

fn parse_input() -> (String, Vec<Rule>) {
    let lines = adventofcode2021::input_lines(14);
    let starting_pattern = lines[0].clone();
    assert!(lines[1].is_empty());
    let rules = lines.iter().skip(2).map(|it| Rule::parse(it)).collect();
    (starting_pattern, rules)
}

fn build_start(pattern: &str) -> Rc<RefCell<Element>> {
    let mut last: Option<Rc<RefCell<Element>>> = None;
    for c in pattern.chars().rev() {
        let curr = Element {
            atom: c,
            next: last.clone(),
        };
        last = Some(Rc::new(RefCell::new(curr)));
    }
    last.unwrap()
}

type RuleLookup = HashMap<char, HashMap<char, Rule>>;

fn index_rules(rules: Vec<Rule>) -> RuleLookup {
    let mut result: RuleLookup = HashMap::new();
    for rule in rules {
        result.entry(rule.first).or_insert_with(HashMap::new);
        result
            .get_mut(&rule.first)
            .unwrap()
            .insert(rule.second, rule.clone());
    }
    result
}

fn apply_rules_once(chain_head: Rc<RefCell<Element>>, rules: &RuleLookup) {
    let mut first: Rc<RefCell<Element>> = chain_head.clone();
    let mut second: Rc<RefCell<Element>> = first.borrow_mut().next.clone().unwrap();

    loop {
        let fc = first.borrow().atom;
        let sc = second.borrow().atom;
        let insertion = rules[&fc][&sc].insertion;
        first.borrow_mut().next = Some(Rc::new(RefCell::new(Element {
            atom: insertion,
            next: Some(second.clone()),
        })));
        first = second;
        if let Some(next) = &first.borrow().next {
            second = next.clone();
        } else {
            break;
        }
    }
}

struct ChainCharIter {
    curr_node: Option<Rc<RefCell<Element>>>,
}

impl ChainCharIter {
    fn new(node: &Rc<RefCell<Element>>) -> ChainCharIter {
        ChainCharIter {
            curr_node: Some(node.clone()),
        }
    }
}

impl Iterator for ChainCharIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr_node.take().map(|node| {
            self.curr_node = node.borrow().next.clone();
            node.borrow().atom
        })
    }
}

fn build_pairs(starting: &str) -> (HashMap<String, u64>, HashMap<char, u64>) {
    let mut prev = starting.chars().next().unwrap();
    let mut result = HashMap::new();
    let mut counts = HashMap::new();
    for c in starting.chars() {
        let curr = *counts.get(&c).unwrap_or(&0);
        counts.insert(c, curr + 1);
    }
    for c in starting.chars().skip(1) {
        let pair = format!("{}{}", prev, c);
        let curr = *result.get(&pair).unwrap_or(&0);
        result.insert(pair, curr + 1);
        prev = c;
    }
    (result, counts)
}

fn apply_rules_once_to_pairs(
    curr_pairs: &HashMap<String, u64>,
    counts: &mut HashMap<char, u64>,
    rules: &[Rule],
) -> HashMap<String, u64> {
    let mut pairs = curr_pairs.clone();
    for rule in rules {
        let pair = format!("{}{}", rule.first, rule.second);
        let curr_count = *curr_pairs.get(&pair).unwrap_or(&0);
        if curr_count > 0 {
            let new_pair_0 = format!("{}{}", rule.first, rule.insertion);
            let new_pair_1 = format!("{}{}", rule.insertion, rule.second);
            let curr_0 = *pairs.get(&new_pair_0).unwrap_or(&0);
            let curr_1 = *pairs.get(&new_pair_1).unwrap_or(&0);
            pairs.insert(new_pair_0, curr_count + curr_0);
            pairs.insert(new_pair_1, curr_count + curr_1);
            let curr_output_count = *pairs.get(&pair).unwrap_or(&0);
            pairs.insert(pair, curr_output_count - curr_count);
            let insertion_count = *counts.get(&rule.insertion).unwrap_or(&0);
            counts.insert(rule.insertion, curr_count + insertion_count);
        }
    }
    pairs
}

fn main() {
    let (starting_pattern, rules) = parse_input();
    let chain_head = build_start(&starting_pattern);
    let rule_index = index_rules(rules.clone());

    for _ in 0..10 {
        apply_rules_once(chain_head.clone(), &rule_index)
    }

    let mut counts: HashMap<char, u32> = HashMap::new();
    for c in ChainCharIter::new(&chain_head) {
        counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);
    }

    let mut pairs: Vec<(char, u32)> = counts.into_iter().collect();
    pairs.sort_unstable_by_key(|(_, count)| *count);
    let most_common_count = pairs.last().unwrap().1;
    let least_common_count = pairs.first().unwrap().1;

    println!("Part 1: {}", most_common_count - least_common_count);

    let (mut atom_pairs, mut atom_counts) = build_pairs(&starting_pattern);

    for _ in 0..10 {
        atom_pairs = apply_rules_once_to_pairs(&atom_pairs, &mut atom_counts, &rules);
    }
    let mut pairs2: Vec<(char, u64)> = atom_counts.into_iter().collect();
    pairs2.sort_unstable_by_key(|(_, count)| *count);
    let most_common_count = pairs2.last().unwrap().1;
    let least_common_count = pairs2.first().unwrap().1;

    println!(
        "Part 1 (alt impl): {}",
        most_common_count - least_common_count
    );

    let (mut atom_pairs, mut atom_counts) = build_pairs(&starting_pattern);

    for _ in 0..40 {
        atom_pairs = apply_rules_once_to_pairs(&atom_pairs, &mut atom_counts, &rules);
    }
    let mut pairs2: Vec<(char, u64)> = atom_counts.into_iter().collect();
    pairs2.sort_unstable_by_key(|(_, count)| *count);
    let most_common_count = pairs2.last().unwrap().1;
    let least_common_count = pairs2.first().unwrap().1;

    println!("Part 2: {}", most_common_count - least_common_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_grow_once() {
        let start = build_start("NNCB");
        let rules = vec![
            Rule {
                first: 'N',
                second: 'N',
                insertion: 'C',
            },
            Rule {
                first: 'N',
                second: 'C',
                insertion: 'B',
            },
            Rule {
                first: 'C',
                second: 'B',
                insertion: 'H',
            },
        ];
        let i_rules = index_rules(rules);
        apply_rules_once(start.clone(), &i_rules);
        let result: String = ChainCharIter::new(&start).collect();
        assert_eq!(result, "NCNBCHB".to_string())
    }

    #[test]
    fn test_by_pairs() {
        let (start, mut counts) = build_pairs("NNCB");
        let rules = vec![
            Rule {
                first: 'N',
                second: 'N',
                insertion: 'C',
            },
            Rule {
                first: 'N',
                second: 'C',
                insertion: 'B',
            },
            Rule {
                first: 'C',
                second: 'B',
                insertion: 'H',
            },
        ];

        let result = apply_rules_once_to_pairs(&start, &mut counts, &rules);
        assert_eq!(counts[&'N'], 2);
        assert_eq!(counts[&'C'], 2);
        assert_eq!(counts[&'B'], 2);
        assert_eq!(counts[&'H'], 1);
        assert_eq!(result["BC"], 1);
        assert_eq!(result["NN"], 0);
    }
}
