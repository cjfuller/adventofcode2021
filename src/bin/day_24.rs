use rayon::prelude::*;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Reg {
    X,
    Y,
    Z,
    W,
}
use Reg::*;

#[derive(Clone, Copy, Debug)]
struct RegError;

impl FromStr for Reg {
    type Err = RegError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(X),
            "y" => Ok(Y),
            "z" => Ok(Z),
            "w" => Ok(W),
            _ => Err(RegError),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Literal(i64),
    Var(Reg),
}

use Operand::*;

impl FromStr for Operand {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<Reg>()
            .map(Var)
            .unwrap_or_else(|_| Literal(s.parse::<i64>().unwrap())))
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

use Operation::*;

impl Operation {
    fn apply(self, reg: &mut Registers, digits: &[i64], i: &mut usize) {
        match self {
            Inp(Var(r)) => {
                *i += 1;
                reg.wr(r, digits[*i - 1])
            }
            Add(Var(a), b) => reg.wr(a, reg.rd(a) + reg.val(b)),
            Mul(Var(a), b) => reg.wr(a, reg.rd(a) * reg.val(b)),
            Div(Var(a), b) => reg.wr(a, reg.rd(a) / reg.val(b)),
            Mod(Var(a), b) => reg.wr(a, reg.rd(a) % reg.val(b)),
            Eql(Var(a), b) => reg.wr(a, if reg.rd(a) == reg.val(b) { 1 } else { 0 }),
            _ => panic!("Invalid operation"),
        }
    }
}

impl FromStr for Operation {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        Ok(match parts[0] {
            "inp" => Inp(parts[1].parse().unwrap()),
            "add" => Add(parts[1].parse().unwrap(), parts[2].parse().unwrap()),
            "mul" => Mul(parts[1].parse().unwrap(), parts[2].parse().unwrap()),
            "div" => Div(parts[1].parse().unwrap(), parts[2].parse().unwrap()),
            "mod" => Mod(parts[1].parse().unwrap(), parts[2].parse().unwrap()),
            "eql" => Eql(parts[1].parse().unwrap(), parts[2].parse().unwrap()),
            _ => panic!("Unknown operation"),
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Registers {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Registers {
    fn wr(&mut self, reg: Reg, value: i64) {
        match reg {
            X => {
                self.x = value;
            }
            Y => {
                self.y = value;
            }
            Z => {
                self.z = value;
            }
            W => {
                self.w = value;
            }
        }
    }
    fn rd(&self, reg: Reg) -> i64 {
        match reg {
            X => self.x,
            Y => self.y,
            Z => self.z,
            W => self.w,
        }
    }
    fn val(&self, oprd: Operand) -> i64 {
        match oprd {
            Var(r) => self.rd(r),
            Literal(n) => n,
        }
    }
}

fn is_valid(digits: &[i64], monad: &[Operation]) -> bool {
    let mut reg = Registers::default();
    let mut input_counter: usize = 0;
    for op in monad {
        op.apply(&mut reg, digits, &mut input_counter);
    }
    reg.z == 0
}

static constants: [(i64, i64, i64); 14] = [
    (14, 1, 8),
    (15, 1, 11),
    (13, 1, 2),
    (-10, 26, 11),
    (14, 1, 1),
    (-3, 26, 5),
    (-14, 26, 10),
    (12, 1, 6),
    (14, 1, 1),
    (12, 1, 11),
    (-6, 26, 9),
    (-6, 26, 14),
    (-2, 26, 11),
    (-9, 26, 2),
];

#[inline(always)]
fn simple(z: i64, inp: i64, b: i64) -> i64 {
    26 * z + inp + b
}

#[inline(always)]
fn complex(z: i64, inp: i64, a: i64, b: i64) -> i64 {
    let x = ((z % 26) + a) != inp;
    if x {
        simple(z / 26, inp, b)
    } else {
        z / 26
    }
}

fn validate_opt_opt(input: &[i64]) -> bool {
    let mut z = simple(0, input[0], 8);
    z = simple(z, input[1], 11);
    z = simple(z, input[2], 2);
    z = complex(z, input[3], -10, 11);
    z = simple(z, input[4], 1);
    z = complex(z, input[5], -3, 5);
    z = complex(z, input[6], -14, 10);
    z = simple(z, input[7], 6);
    z = simple(z, input[8], 1);
    z = simple(z, input[9], 11);
    z = complex(z, input[10], -6, 9);
    z = complex(z, input[11], -6, 14);
    z = complex(z, input[12], -2, 11);
    z % 26 == input[13] + 9
    //z = complex(z, input[13], -9, 2);
    //z == 0
}

fn validate_opt(input: &[i64]) -> bool {
    let mut z = simple(0, input[0], 8);
    z = simple(z, input[1], 11);
    z = simple(z, input[2], 2);
    z = complex(z, input[3], -10, 11);
    z = simple(z, input[4], 1);
    z = complex(z, input[5], -3, 5);
    z = complex(z, input[6], -14, 10);
    z = simple(z, input[7], 6);
    z = simple(z, input[8], 1);
    z = simple(z, input[9], 11);
    z = complex(z, input[10], -6, 9);
    z = complex(z, input[11], -6, 14);
    z = complex(z, input[12], -2, 11);
    z = complex(z, input[13], -9, 2);

    z == 0
}

#[inline(always)]
fn iter(mut z: i64, input: i64, a: i64, zdiv: i64, b: i64) -> i64 {
    let x = if ((z % 26) + a) == input { 0 } else { 1 };
    z /= zdiv;
    z *= 1 + (25 * x);
    z += x * (input + b);
    z
}

fn validate(input: &[i64]) -> bool {
    let mut z = 0;
    for (inp, (a, zdiv, b)) in input.iter().zip(constants) {
        z = iter(z, *inp, a, zdiv, b);
    }
    z == 0
}

fn main() {
    // let digits: [i64; 14] = [2, 4, 1, 9, 3, 1, 1, 1, 6, 1, 6, 1, 5, 1];
    // println!("{:?}: {:?}", &digits, validate(&digits));
    // let digits = [2, 8, 1, 9, 3, 1, 5, 1, 6, 1, 6, 1, 5, 1];
    // println!("{:?}: {:?}", &digits, validate(&digits));
    // let digits = [2, 7, 1, 9, 3, 1, 4, 1, 6, 1, 6, 1, 5, 1];
    // println!("{:?}: {:?}", &digits, validate(&digits));
    // let digits = [2, 4, 1, 9, 3, 1, 1, 1, 6, 1, 6, 1, 5, 1];
    // println!("{:?}: {:?}", &digits, validate(&digits));
    // let digits = [2, 6, 1, 9, 3, 1, 3, 1, 6, 1, 6, 1, 5, 1];
    // println!("{:?}: {:?}", &digits, validate(&digits));
    // let digits = [2, 5, 1, 9, 3, 1, 2, 1, 6, 1, 6, 1, 5, 1];
    // println!("{:?}: {:?}", &digits, validate(&digits));
    // return;
    let test = [9, 9, 9, 1, 9, 7, 6, 5, 9, 4, 9, 4, 9, 8];
    println!(
        "Part 1 verification: {:?} -> {:?}",
        &test,
        validate_opt_opt(&test),
    );
    let mut i = 0;
    let mut digits: Vec<i64> = vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
    loop {
        break;
        for di in 0..14 {
            digits[13 - di] -= 1;
            if i % 10000000 == 0 {
                println!("{:?}", &i);
            }
            i += 1;
            if digits[13 - di] == 0 {
                digits[13 - di] = 9;
            } else {
                break;
            }
        }
        if validate_opt(&digits) {
            println!("Part 1: {:?}", digits);
            break;
        }
        if digits.iter().all(|it| *it == 1) {
            panic!("Exhausted serial numbers.")
        }
    }
    let base_digits: Vec<i64> = vec![2, 4, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    const par_idx: usize = 5;
    const par_idx_2: usize = 6;
    let _: Vec<()> = (base_digits[par_idx]..10)
        .collect::<Vec<i64>>()
        .par_iter()
        .map(|d_next| {
            let mut digits_1 = base_digits.clone();
            digits_1[par_idx] = *d_next;

            // let mut x = 0;

            let _: Vec<()> = (base_digits[par_idx_2]..10)
                .collect::<Vec<i64>>()
                .par_iter()
                .map(|d_next_2| {
                    let mut digits = digits_1.clone();
                    digits[par_idx_2] = *d_next_2;
                    loop {
                        for di in 0..14 {
                            let inv_di = 13 - di;
                            if inv_di == par_idx || inv_di == par_idx_2 {
                                continue;
                            }
                            digits[inv_di] += 1;
                            if digits[inv_di] == 10 {
                                digits[inv_di] = 1;
                            } else {
                                break;
                            }
                        }
                        if validate_opt(&digits) && validate(&digits) {
                            println!("Part 2: {:?}", digits);
                            return;
                        }
                        if digits
                            .iter()
                            .enumerate()
                            .all(|(idx, it)| *it == 9 || idx == par_idx || idx == par_idx_2)
                        {
                            return;
                        }
                    }
                })
                .collect();
        })
        .collect();
}
