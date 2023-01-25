use std::{collections::VecDeque, cmp::Ordering, cmp::max};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(usize)
}

fn parse_int(tokens: &mut VecDeque<&str>) -> Packet {
    Packet::Int(tokens.pop_front().unwrap().parse::<usize>().unwrap())
}

fn parse_list(tokens: &mut VecDeque<&str>) -> Packet {
    tokens.pop_front().expect("Tried to parse list with no tokens");
    let mut packets = Vec::new();
    while let Some(packet) = parse_list_item(tokens) {
        packets.push(packet);
    }
    assert_eq!(tokens.pop_front().unwrap(), "]");
    Packet::List(packets)
}

fn parse_list_item(tokens: &mut VecDeque<&str>) -> Option<Packet> {
    match tokens[0] {
        "]" => None,
        "[" => Some(parse_list(tokens)),
        _ => Some(parse_int(tokens))
    }
}

fn parse_packet(line: &str) -> Packet {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[|\d+|\]").unwrap();
    }
    let mut tokens = RE.captures_iter(line)
        .map(|c| c.get(0).unwrap().as_str())
        .collect::<VecDeque<_>>();

    parse_list(&mut tokens)
}

fn compare(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::List(ref v_left), Packet::List(ref v_right)) => {
            for i in 0..max(v_left.len(), v_right.len()) {
                match (v_left.get(i), v_right.get(i)) {
                    (Some(_), None) => return Ordering::Greater,
                    (None, Some(_)) => return Ordering::Less,
                    (Some(l), Some(r)) => {
                        match compare( l,  r) {
                            Ordering::Equal => {},
                            order => return order 
                        } 
                    },
                    _ => unreachable!()
                }
            }
            Ordering::Equal
        },
        (Packet::List(_), Packet::Int(r)) => compare(left, &Packet::List(vec![Packet::Int(*r)])),
        (Packet::Int(l), Packet::List(_)) => compare(&Packet::List(vec![Packet::Int(*l)]), right),
        (Packet::Int(l), Packet::Int(r)) => l.cmp(r)
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let packets = input.split("\n\n")
        .map(|packets_str| packets_str.split_once('\n').unwrap())
        .map(|(l1, l2)| (parse_packet(l1), parse_packet(l2)))
        .collect_vec();

    let p1 = packets.iter().positions(|(l, r)| {
        compare(l, r) != Ordering::Greater
    }).map(|i| i + 1).sum();

    let mut packets = packets.iter().flat_map(|(l, r)| [l.clone(), r.clone()]).collect::<Vec<_>>();
    let dividers = [
        Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Int(6)])])
    ];
    packets.extend(dividers.clone());

    packets.sort_by(compare);

    let p2: usize = packets.iter().positions(|p| dividers.contains(p)).map(|i| i + 1).product();

    (p1, p2)
}