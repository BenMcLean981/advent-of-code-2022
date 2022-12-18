use std::{collections::VecDeque, fmt, str::FromStr};

#[derive(Clone)]
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
