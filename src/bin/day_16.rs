use bitvec::prelude::*;

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}

impl Op {
    fn from_code(code: u8) -> Op {
        match code {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::Greater,
            6 => Self::Less,
            7 => Self::Equal,
            _ => panic!("Unknown op code {}", code),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        num: BitVec<Msb0, u8>,
    },
    Operator {
        version: u8,
        op: Op,
        subpackets: Vec<Packet>,
    },
}

impl Packet {
    fn eval(&self) -> u64 {
        match self {
            Self::Literal { num, .. } => num.load_be(),
            Self::Operator { op, subpackets, .. } => {
                let subeval = subpackets.iter().map(|it| it.eval());
                match op {
                    Op::Sum => subeval.sum(),
                    Op::Product => subeval.product(),
                    Op::Minimum => subeval.min().unwrap(),
                    Op::Maximum => subeval.max().unwrap(),
                    Op::Greater => {
                        let as_vec: Vec<u64> = subeval.collect();
                        assert!(as_vec.len() == 2);
                        if as_vec[0] > as_vec[1] {
                            1
                        } else {
                            0
                        }
                    }
                    Op::Less => {
                        let as_vec: Vec<u64> = subeval.collect();
                        assert!(as_vec.len() == 2);
                        if as_vec[0] < as_vec[1] {
                            1
                        } else {
                            0
                        }
                    }
                    Op::Equal => {
                        let as_vec: Vec<u64> = subeval.collect();
                        assert!(as_vec.len() == 2);
                        if as_vec[0] == as_vec[1] {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
}

fn read_input() -> BitVec<Msb0, u8> {
    let bytes: Vec<u8> = hex::decode(adventofcode2021::load_input(16)).unwrap();
    BitVec::from_vec(bytes)
}

fn parse_literal_num(input: &BitSlice<Msb0, u8>) -> (BitVec<Msb0, u8>, &BitSlice<Msb0, u8>) {
    let mut building = input[1..5].to_bitvec();
    let remainder = if input[0] {
        let (mut parsed, rest) = parse_literal_num(&input[5..]);
        building.append(&mut parsed);
        rest
    } else {
        &input[5..]
    };
    (building, remainder)
}

fn parse_packet(input: &BitSlice<Msb0, u8>) -> (Packet, &BitSlice<Msb0, u8>) {
    let version: u8 = input[0..3].load_be();
    let packet_type: u8 = input[3..6].load_be();
    if packet_type == 4u8 {
        let (num, rest) = parse_literal_num(&input[6..]);
        (Packet::Literal { version, num }, rest)
    } else {
        let length_type_id = input[6];
        if length_type_id {
            let num_sub_packets: u16 = input[7..18].load_be();
            let mut remaining = &input[18..];
            let mut packets = vec![];
            for _ in 0..num_sub_packets {
                let (packet, rest) = parse_packet(remaining);
                remaining = rest;
                packets.push(packet);
            }
            (
                Packet::Operator {
                    version,
                    op: Op::from_code(packet_type),
                    subpackets: packets,
                },
                remaining,
            )
        } else {
            let num_sub_packet_bits: usize = input[7..22].load_be();
            let mut remaining = &input[22..(22 + num_sub_packet_bits)];
            let mut packets = vec![];
            while !remaining.is_empty() && !remaining.not_any() {
                let (packet, rest) = parse_packet(remaining);
                remaining = rest;
                packets.push(packet)
            }
            (
                Packet::Operator {
                    version,
                    op: Op::from_code(packet_type),
                    subpackets: packets,
                },
                &input[(22 + num_sub_packet_bits)..],
            )
        }
    }
}

fn sum_versions(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version, .. } => *version as u64,
        Packet::Operator {
            version,
            subpackets,
            ..
        } => subpackets.iter().map(sum_versions).sum::<u64>() + (*version as u64),
    }
}

fn main() {
    let input = read_input();
    let (packet, _) = parse_packet(&input);
    println!("Part 1: {}", sum_versions(&packet));
    let value = packet.eval();
    println!("Part 2: {}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        let vec = BitVec::from_vec(hex::decode("D2FE28").unwrap());
        let (packet, _) = parse_packet(&vec);
        assert_eq!(
            packet,
            Packet::Literal {
                version: 6,
                num: bitvec![Msb0, u8; 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1]
            }
        )
    }

    #[test]
    fn test_operator() {
        let vec = BitVec::from_vec(hex::decode("38006F45291200").unwrap());
        let (packet, _) = parse_packet(&vec);
        assert_eq!(
            packet,
            Packet::Operator {
                version: 1,
                subpackets: vec![
                    Packet::Literal {
                        version: 6,
                        num: bitvec![Msb0, u8; 1, 0, 1, 0]
                    },
                    Packet::Literal {
                        version: 2,
                        num: bitvec![Msb0, u8; 0, 0, 0, 1, 0, 1, 0, 0]
                    }
                ],
                op: Op::from_code(6)
            }
        )
    }

    #[test]
    fn test_operator_2() {
        let vec = BitVec::from_vec(hex::decode("EE00D40C823060").unwrap());
        let (packet, _) = parse_packet(&vec);
        assert_eq!(
            packet,
            Packet::Operator {
                version: 7,
                op: Op::from_code(3),
                subpackets: vec![
                    Packet::Literal {
                        version: 2,
                        num: bitvec![Msb0, u8; 0, 0, 0, 1],
                    },
                    Packet::Literal {
                        version: 4,
                        num: bitvec![Msb0, u8; 0, 0, 1, 0],
                    },
                    Packet::Literal {
                        version: 1,
                        num: bitvec![Msb0, u8; 0, 0, 1, 1],
                    }
                ]
            }
        )
    }
}
