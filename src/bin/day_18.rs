use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq)]
enum Number {
    Literal(Rc<RefCell<i64>>),
    Pair {
        left: Rc<RefCell<Number>>,
        right: Rc<RefCell<Number>>,
    },
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Literal(num) => write!(f, "{}", *num.borrow()),
            Self::Pair { left, right } => write!(f, "[{},{}]", &*left.borrow(), &*right.borrow()),
        }
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(self, f)
    }
}

#[derive(Clone, Debug)]
struct ReduceContext {
    last_left_num: Option<Rc<RefCell<i64>>>,
    depth: usize,
    stack: Vec<Rc<RefCell<Number>>>,
}

impl ReduceContext {
    fn new() -> ReduceContext {
        ReduceContext {
            last_left_num: None,
            depth: 0,
            stack: vec![],
        }
    }
}

fn find_rhs_num(curr: Option<Rc<RefCell<Number>>>, ctx: ReduceContext) -> Option<Rc<RefCell<i64>>> {
    match curr {
        None => {
            if ctx.stack.is_empty() {
                None
            } else {
                find_rhs_num(
                    ctx.stack.last().cloned(),
                    ReduceContext {
                        stack: ctx.stack.split_at(ctx.stack.len() - 1).1.to_vec(),
                        ..ctx
                    },
                )
            }
        }
        Some(num) => match &*num.borrow() {
            Number::Literal(n) => Some(n.clone()),
            Number::Pair { left, right } => {
                let mut stack = ctx.stack.clone();
                stack.push(right.clone());
                let lhs_result = find_rhs_num(
                    Some(left.clone()),
                    ReduceContext {
                        stack,
                        ..ctx.clone()
                    },
                );
                if lhs_result.is_some() {
                    lhs_result
                } else {
                    find_rhs_num(Some(right.clone()), ctx)
                }
            }
        },
    }
}

impl std::ops::Add<Number> for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Number {
        let mut sum = Number::Pair {
            left: Rc::new(RefCell::new(self)),
            right: Rc::new(RefCell::new(rhs)),
        };
        sum.reduce();
        sum
    }
}

impl Number {
    fn deep_clone(&self) -> Number {
        match self {
            Number::Literal(num) => Number::of_num(*num.borrow()),
            Number::Pair { left, right } => Number::Pair {
                left: Rc::new(RefCell::new(left.borrow().deep_clone())),
                right: Rc::new(RefCell::new(right.borrow().deep_clone())),
            },
        }
    }
    fn from_str(s: &str) -> Number {
        parse(&tokenize(s)).0
    }
    fn of_num(num: i64) -> Number {
        Number::Literal(Rc::new(RefCell::new(num)))
    }
    fn magnitude(&self) -> i64 {
        match self {
            Number::Literal(num) => *num.borrow(),
            Number::Pair { left, right } => {
                3 * left.borrow().magnitude() + 2 * right.borrow().magnitude()
            }
        }
    }
    fn reduce(&mut self) {
        while self.reduce_once() {}
    }
    fn reduce_once(&mut self) -> bool {
        if !self._reduce_explode(&mut ReduceContext::new()) {
            self._reduce_split()
        } else {
            true
        }
    }
    fn _reduce_split(&mut self) -> bool {
        match self {
            Self::Literal(num) => {
                assert!(*num.borrow() < 10);
                return false;
            }
            Self::Pair { left, right } => {
                let mut left_mut = left.borrow_mut();
                match &*left_mut {
                    Self::Literal(num) => {
                        let val = *num.borrow();
                        if val >= 10 {
                            *left_mut = Number::Pair {
                                left: Rc::new(RefCell::new(Number::of_num(val / 2))),
                                right: Rc::new(RefCell::new(Number::of_num(val / 2 + (val % 2)))),
                            };
                            return true;
                        }
                    }
                    Self::Pair { .. } => {
                        let l_result = left_mut._reduce_split();
                        if l_result {
                            return l_result;
                        }
                    }
                };
                let mut right_mut = right.borrow_mut();
                match &*right_mut {
                    Self::Literal(num) => {
                        let val = *num.borrow();
                        if val >= 10 {
                            *right_mut = Number::Pair {
                                left: Rc::new(RefCell::new(Number::of_num(val / 2))),
                                right: Rc::new(RefCell::new(Number::of_num(val / 2 + (val % 2)))),
                            };
                            return true;
                        }
                    }
                    Self::Pair { .. } => {
                        let r_result = right_mut._reduce_split();
                        if r_result {
                            return r_result;
                        }
                    }
                };
            }
        }
        false
    }
    fn _reduce_explode(&mut self, ctx: &mut ReduceContext) -> bool {
        match self {
            Self::Literal(num) => {
                ctx.last_left_num = Some(num.clone());
                return false;
            }
            Self::Pair { left, right } => {
                if ctx.depth == 3 {
                    let mut borrowed_left = left.borrow_mut();
                    match &*borrowed_left {
                        Self::Pair {
                            left: ll,
                            right: lr,
                        } => {
                            // Explode!
                            if let Some(r) = find_rhs_num(Some(right.clone()), ctx.clone()) {
                                match &*lr.borrow() {
                                    Number::Literal(num) => {
                                        *r.borrow_mut() += *num.borrow();
                                    }
                                    _ => {
                                        panic!("Found pair at depth 4");
                                    }
                                }
                            }
                            if let Some(l) = &ctx.last_left_num {
                                match &*ll.borrow() {
                                    Number::Literal(num) => {
                                        *l.borrow_mut() += *num.borrow();
                                    }
                                    _ => {
                                        panic!("Found pair at depth 4");
                                    }
                                }
                            }
                            *borrowed_left = Number::of_num(0);
                            return true;
                        }
                        Self::Literal(num) => ctx.last_left_num = Some(num.clone()),
                    };
                    let mut borrowed_right = right.borrow_mut();
                    match &*borrowed_right {
                        Self::Pair {
                            left: rl,
                            right: rr,
                        } => {
                            // Explode!
                            if let Some(r) = find_rhs_num(None, ctx.clone()) {
                                match &*rr.borrow() {
                                    Number::Literal(num) => {
                                        *r.borrow_mut() += *num.borrow();
                                    }
                                    _ => {
                                        panic!("Found pair at depth 4");
                                    }
                                }
                            }
                            if let Some(l) = &ctx.last_left_num {
                                match &*rl.borrow() {
                                    Number::Literal(num) => {
                                        *l.borrow_mut() += *num.borrow();
                                    }
                                    _ => {
                                        panic!("Found pair at depth 4");
                                    }
                                }
                            }
                            *borrowed_right = Number::of_num(0);
                            return true;
                        }
                        Self::Literal(num) => ctx.last_left_num = Some(num.clone()),
                    }
                } else {
                    let old_stack = ctx.stack.clone();
                    let mut stack = old_stack.clone();
                    stack.push(right.clone());
                    ctx.depth += 1;
                    ctx.stack = stack;

                    let left_result = left.borrow_mut()._reduce_explode(ctx);
                    ctx.stack = old_stack;
                    if left_result {
                        ctx.depth -= 1;
                        return left_result;
                    }
                    let right_result = right.borrow_mut()._reduce_explode(ctx);
                    ctx.depth -= 1;
                    return right_result;
                }
            }
        };
        false
    }
}

#[derive(Clone, Copy, Debug)]
enum Token {
    Num(i64),
    Open,
    Close,
}

fn next_token(s: &str) -> (Token, &str) {
    if s.is_empty() {
        panic!("Malformed string");
    }
    if s.starts_with('[') {
        (Token::Open, s.split_at(1).1)
    } else if s.starts_with(']') {
        (Token::Close, s.split_at(1).1)
    } else if let Some(num) = Regex::new(r"^(\d+)")
        .unwrap()
        .captures(s)
        .and_then(|caps| caps.get(1))
        .map(|it| it.as_str())
    {
        (
            Token::Num(num.parse().unwrap()),
            s.trim_start_matches(char::is_numeric),
        )
    } else {
        next_token(s.split_at(1).1)
    }
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut curr = s;
    let mut output: Vec<Token> = vec![];
    while !curr.is_empty() {
        let (t, rest) = next_token(curr);
        curr = rest;
        output.push(t)
    }
    output
}

fn parse(tokens: &[Token]) -> (Number, &[Token]) {
    match tokens[0] {
        Token::Num(n) => (Number::of_num(n), &tokens[1..]),
        Token::Open => {
            let (left, rest) = parse(&tokens[1..]);
            let (right, rest) = parse(rest);
            if !matches!(rest[0], Token::Close) {
                panic!("Expected closing delimiter, got: {:?}", rest[0]);
            }
            (
                Number::Pair {
                    left: Rc::new(RefCell::new(left)),
                    right: Rc::new(RefCell::new(right)),
                },
                &rest[1..],
            )
        }
        Token::Close => {
            panic!("Mismatched closing delimiter.")
        }
    }
}

fn main() {
    let summed = adventofcode2021::input_lines(18)
        .iter()
        .map(|s| Number::from_str(s))
        .map(|mut it| {
            it.reduce();
            it
        })
        .reduce(|acc, el| acc + el);
    let mag = summed.unwrap().magnitude();
    println!("Part 1: {}", mag);
    let parsed: Vec<Number> = adventofcode2021::input_lines(18)
        .iter()
        .map(|s| Number::from_str(s))
        .collect();
    let mut max_mag = 0;
    for (i_idx, i) in parsed.iter().enumerate() {
        for (j_idx, j) in parsed.iter().enumerate() {
            if i_idx == j_idx {
                continue;
            }
            max_mag = std::cmp::max((i.deep_clone() + j.deep_clone()).magnitude(), max_mag);
        }
    }
    println!("Part 2: {}", max_mag);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn explode_assert(s: &str, expected: &str) {
        let mut num = Number::from_str(s);
        num._reduce_explode(&mut ReduceContext::new());
        let expected = Number::from_str(expected);
        assert_eq!(num, expected);
    }

    fn split_assert(s: &str, expected: &str) {
        let mut num = Number::from_str(s);
        num._reduce_split();
        let expected = Number::from_str(expected);
        assert_eq!(num, expected);
    }

    #[test]
    fn test_explode() {
        explode_assert("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        explode_assert("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        explode_assert("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        explode_assert(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        explode_assert(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    fn test_split() {
        split_assert("[10,7]", "[[5,5],7]");
        split_assert("[11,7]", "[[5,6],7]");
    }

    #[test]
    fn test_reduce() {
        let mut num = Number::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        num.reduce_once();
        let expected = Number::from_str("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
        assert_eq!(num, expected);
        num.reduce_once();
        let expected = Number::from_str("[[[[0,7],4],[15,[0,13]]],[1,1]]");
        assert_eq!(num, expected);
        num.reduce_once();
        let expected = Number::from_str("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        assert_eq!(num, expected);
        num.reduce_once();
        let expected = Number::from_str("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
        assert_eq!(num, expected);
        num.reduce_once();
        let expected = Number::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(num, expected);

        let mut num = Number::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        num.reduce();
        let expected = Number::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(num, expected);
    }

    #[test]
    fn test_sum_magnitude() {
        let n0 = Number::from_str("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]");
        let n1 = Number::from_str("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]");
        let sum = n0 + n1;
        let expected =
            Number::from_str("[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]");
        assert_eq!(sum, expected);
        let expected_mag: i64 = 3993;
        assert_eq!(sum.magnitude(), expected_mag);
    }
}
