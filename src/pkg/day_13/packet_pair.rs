use std::fmt::{self};

use super::packet::Packet;

pub(crate) struct PacketPair(Packet, Packet);

impl PacketPair {
    pub fn new(p1: Packet, p2: Packet) -> Self {
        return PacketPair(p1, p2);
    }

    pub fn is_in_order(&self) -> bool {
        let order = is_in_order(&self.0, &self.1);

        return order == Order::InOrder;
    }
}

#[derive(PartialEq)]
enum Order {
    InOrder,
    NotInOrder,
    Unclear,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Order::InOrder => write!(f, "{}", "InOrder"),
            Order::NotInOrder => write!(f, "{}", "NotInOrder"),
            Order::Unclear => write!(f, "{}", "Unclear"),
        }
    }
}

fn is_in_order(p1: &Packet, p2: &Packet) -> Order {
    // figure out the combination we have,
    // 1: int int
    // 2: int sub -> convert to sub[int] sub (clone ok because int is only a few bits)
    // 3: sub int -> convert to sub sub[int] (clone okbecause int is only a few bits).
    // 4: sub sub

    match p1 {
        Packet::Integer(u1) => match p2 {
            Packet::Integer(u2) => are_ints_in_order(u1, u2),
            Packet::Sub(_) => {
                is_in_order(&Packet::Sub(Box::new(vec![p1.clone()])), p2)
            }
        },
        Packet::Sub(s1) => match p2 {
            Packet::Integer(_) => {
                is_in_order(p1, &Packet::Sub(Box::new(vec![p2.clone()])))
            }
            Packet::Sub(s2) => are_subs_in_order(s1, s2),
        },
    }
}

fn are_subs_in_order(s1: &Box<Vec<Packet>>, s2: &Box<Vec<Packet>>) -> Order {
    for (i, p1) in s1.iter().enumerate() {
        if i == s2.len() {
            // right side out of items first.
            return Order::NotInOrder;
        }

        let p2 = &s2[i];
        let order = is_in_order(p1, p2);

        if order != Order::Unclear {
            return order;
        }
    }

    if s2.len() > s1.len() {
        // left side ran out of items first.

        return Order::InOrder;
    }

    return Order::Unclear;
}

fn are_ints_in_order(u1: &usize, u2: &usize) -> Order {
    if u1 < u2 {
        return Order::InOrder;
    } else if u1 > u2 {
        return Order::NotInOrder;
    } else {
        return Order::Unclear;
    }
}

impl fmt::Display for PacketPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.0.to_string(), self.1.to_string())
    }
}
