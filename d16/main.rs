use std::ops::{Add, Shl};

const TD_LITERAL: u8 = 4;

const OK_SUM: u8 = 0;
const OK_PROD: u8 = 1;
const OK_MIN: u8 = 2;
const OK_MAX: u8 = 3;
const OK_GT: u8 = 5;
const OK_LT: u8 = 6;
const OK_EQ: u8 = 7;

#[derive(Debug, Clone)]
enum PacketKind {
    Literal(usize),
    Operator(u8, Vec<Packet>),
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    packet: PacketKind,
}

impl Packet {
    fn parse_packet(input: &mut &[u8]) -> Packet {
        let version: u8 = consume_bits(input, 3);
        let type_id: u8 = consume_bits(input, 3);
        let packet = match type_id {
            TD_LITERAL => Packet::parse_literal(input),
            n => Packet::parse_operator(input, n),
        };
        Packet { version, packet }
    }

    fn parse_literal(input: &mut &[u8]) -> PacketKind {
        let mut acc = 0usize;
        let mut flag = false;
        while !flag {
            let leading: u8 = consume_bits(input, 1);
            if leading == 0 {
                flag = true
            }
            let group: usize = consume_bits(input, 4);
            acc = (acc << 4) + group;
        }
        PacketKind::Literal(acc)
    }

    fn parse_operator(input: &mut &[u8], code: u8) -> PacketKind {
        let length_type_id_raw: u8 = consume_bits(input, 1);
        let sub_packets = match length_type_id_raw {
            0 => {
                let mut total_len: usize = consume_bits(input, 15);
                let mut acc = vec![];
                while total_len > 0 {
                    let initial_len = input.len();
                    acc.push(Packet::parse_packet(input));
                    total_len -= initial_len - input.len();
                }
                acc
            }
            1 => {
                let count: usize = consume_bits(input, 11);
                let mut acc = vec![];
                for _ in 0..count {
                    acc.push(Packet::parse_packet(input));
                }
                acc
            }
            _ => unreachable!(),
        };
        PacketKind::Operator(code, sub_packets)
    }

    fn eval(&self) -> usize {
        match self.packet {
            PacketKind::Literal(l) => l,
            PacketKind::Operator(code, ref v) => match code {
                OK_SUM => v.iter().map(Packet::eval).sum(),
                OK_PROD => v.iter().map(Packet::eval).product(),
                OK_MIN => v.iter().map(Packet::eval).min().unwrap(),
                OK_MAX => v.iter().map(Packet::eval).max().unwrap(),
                OK_GT => {
                    let mut iter = v.iter();
                    let (p1, p2) = (iter.next().unwrap(), iter.next().unwrap());
                    if p1.eval() > p2.eval() {
                        1
                    } else {
                        0
                    }
                }
                OK_LT => {
                    let mut iter = v.iter();
                    let (p1, p2) = (iter.next().unwrap(), iter.next().unwrap());
                    if p1.eval() < p2.eval() {
                        1
                    } else {
                        0
                    }
                }
                OK_EQ => {
                    let mut iter = v.iter();
                    let (p1, p2) = (iter.next().unwrap(), iter.next().unwrap());
                    if p1.eval() == p2.eval() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?
        .chars()
        .map(|x| match x {
            c @ '0'..='9' => c as u8 - b'0',
            c @ 'A'..='F' => (c as u8 - b'A') + 10,
            _ => unreachable!(),
        })
        .map(|n| [(n & 8) >> 3, (n & 4) >> 2, (n & 2) >> 1, n & 1])
        .flatten()
        .collect::<Vec<u8>>();

    let mut stream = input.as_slice();
    let packet = Packet::parse_packet(&mut stream);
    let mut queue = vec![packet.clone()];
    let mut sum = 0usize;
    while let Some(p) = queue.pop() {
        sum += p.version as usize;
        if let PacketKind::Operator(_, mut s) = p.packet {
            queue.append(&mut s)
        }
    }
    println!("A: {}", sum);

    println!("B: {}", packet.eval());
    Ok(())
}

fn consume_bits<T>(input: &mut &[u8], len: usize) -> T
where
    T: Default + Shl<usize, Output = T> + From<u8> + Copy + Add<Output = T>,
{
    if std::mem::size_of::<T>() * 8 < len {
        panic!()
    }
    let mut acc = T::default();
    (0..len).for_each(|i| acc = (acc << 1) + input[i].into());
    *input = &input[len..];
    acc
}
