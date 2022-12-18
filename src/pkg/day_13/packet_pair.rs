use std::{
    cmp::Ordering,
    fmt::{self},
};

use super::packet::Packet;

pub(crate) struct PacketPair(pub Packet, pub Packet);

impl PacketPair {
    pub fn new(p1: Packet, p2: Packet) -> Self {
        return PacketPair(p1, p2);
    }

    pub fn is_in_order(&self) -> bool {
        match self.0.partial_cmp(&self.1) {
            Some(order) => order == Ordering::Less,
            None => false,
        }
    }
}

impl fmt::Display for PacketPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.0.to_string(), self.1.to_string())
    }
}
