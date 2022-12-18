use std::{cmp::Ordering, collections::VecDeque, fmt, str::FromStr};

#[derive(Clone, PartialEq, Eq)]
pub enum Packet {
    Integer(usize),
    Sub(Box<Vec<Packet>>),
}

impl ToString for Packet {
    fn to_string(&self) -> String {
        match self {
            Packet::Integer(i) => i.to_string(),
            Packet::Sub(s) => sub_to_str(s),
        }
    }
}

fn sub_to_str(sub: &Box<Vec<Packet>>) -> String {
    let mut strings: Vec<String> = vec![];

    for s in sub.iter() {
        strings.push(s.to_string());
    }

    return format!("[{}]", strings.join(","));
}

#[derive(Debug)]
pub struct PacketParseError;

impl fmt::Display for PacketParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Could not read Packet.");
    }
}

impl FromStr for Packet {
    type Err = PacketParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut queue: VecDeque<char> = s.chars().collect();

        return Ok(parse(&mut queue)[0].clone());
    }
}

/*
 * Use a queue so that when we recur we consume the whole list
 * from '[' to ']' then return to parent after the ']'
*/
fn parse(queue: &mut VecDeque<char>) -> Vec<Packet> {
    let mut packets: Vec<Packet> = vec![];
    let mut num: Vec<char> = vec![];

    while !queue.is_empty() {
        let c = queue.pop_front().unwrap();

        match c {
            '[' => {
                let sub = parse(queue);
                let sub = Box::new(sub);
                let sub = Packet::Sub(sub);
                packets.push(sub);
            }
            ']' => {
                let packet_num = read_num(&num);
                match packet_num {
                    Some(n) => packets.push(n),
                    None => (),
                }
                return packets;
            }
            ',' => {
                let packet_num = read_num(&num);
                match packet_num {
                    Some(n) => packets.push(n),
                    None => (),
                }
                num = vec![];
            }
            n => num.push(n),
        }
    }

    return packets;
}

fn read_num(num: &Vec<char>) -> Option<Packet> {
    let num: String = num.iter().collect();
    let num = usize::from_str(&num);

    match num {
        Ok(num) => Some(Packet::Integer(num)),
        Err(_) => None,
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        let partial_ord = self.partial_cmp(other);
        match partial_ord {
            Some(o) => o,
            None => Ordering::Equal,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // figure out the combination we have,
        // 1: int int
        // 2: int sub -> convert to sub[int] sub (clone ok because int is only a few bits)
        // 3: sub int -> convert to sub sub[int] (clone okbecause int is only a few bits).
        // 4: sub sub

        match self {
            Packet::Integer(u1) => match other {
                Packet::Integer(u2) => are_ints_in_order(u1, u2),
                Packet::Sub(_) => Packet::Sub(Box::new(vec![self.clone()]))
                    .partial_cmp(other),
            },
            Packet::Sub(s1) => match other {
                Packet::Integer(_) => self
                    .partial_cmp(&Packet::Sub(Box::new(vec![other.clone()]))),
                Packet::Sub(s2) => are_subs_in_order(s1, s2),
            },
        }
    }
}

fn are_subs_in_order(
    s1: &Box<Vec<Packet>>,
    s2: &Box<Vec<Packet>>,
) -> Option<std::cmp::Ordering> {
    for (i, p1) in s1.iter().enumerate() {
        if i == s2.len() {
            // right side out of items first.
            return Some(Ordering::Greater);
        }

        let p2 = &s2[i];
        let order = p1.partial_cmp(p2);

        if order.is_some() {
            return order;
        }
    }

    if s2.len() > s1.len() {
        // left side ran out of items first.

        return Some(Ordering::Less);
    }

    return None; // can't decide.
}

fn are_ints_in_order(u1: &usize, u2: &usize) -> Option<std::cmp::Ordering> {
    if u1 < u2 {
        return Some(Ordering::Less);
    } else if u1 > u2 {
        return Some(Ordering::Greater);
    } else {
        return None;
    }
}
