use std::{io::BufRead, str::FromStr};

use crate::pkg::utils::get_file_reader;

use super::{packet::Packet, packet_pair::PacketPair};

pub fn print_solutions() {
    let filename = "./src/pkg/day_13/input.txt";

    let pairs = get_packet_pairs(filename);
    let sum_of_in_order_pair_indices =
        get_sum_of_in_order_pair_indices(&pairs);

    let decorder_key = get_decorder_key(&pairs);

    println!("Day 13:");
    println!(
        "The sum of indices of packet pairs who are in order is {}.",
        sum_of_in_order_pair_indices
    );
    println!("The decoder key is {}.", decorder_key);
}

fn get_packet_pairs(filename: &str) -> Vec<PacketPair> {
    let mut p1: Option<Packet> = None;
    let mut p2: Option<Packet> = None;

    let mut pairs: Vec<PacketPair> = vec![];

    let reader = get_file_reader(filename);

    for line in reader.lines() {
        let line = &line.unwrap();

        if line.trim() == "" {
            pairs.push(PacketPair::new(p1.unwrap(), p2.unwrap()));

            p1 = None;
            p2 = None;
        } else if p1.is_none() {
            p1 = Some(Packet::from_str(line).unwrap());
        } else {
            p2 = Some(Packet::from_str(line).unwrap());
        }
    }

    pairs.push(PacketPair::new(p1.unwrap(), p2.unwrap()));

    return pairs;
}

fn get_sum_of_in_order_pair_indices(pairs: &Vec<PacketPair>) -> usize {
    let mut sum = 0;

    for (i, pair) in pairs.iter().enumerate() {
        let idx = i + 1; // pairs are 1 indexed.

        if pair.is_in_order() {
            sum = sum + idx;
        }
    }

    return sum;
}

// couldn't think of a better way to do this.
fn make_div_pack_1() -> Packet {
    Packet::Sub(Box::new(vec![Packet::Integer(2)]))
}

fn make_div_pack_2() -> Packet {
    Packet::Sub(Box::new(vec![Packet::Integer(6)]))
}

fn get_decorder_key(pairs: &Vec<PacketPair>) -> usize {
    let ordered = order_packets(pairs);

    let first_idx = ordered
        .iter()
        .position(|p| p == &make_div_pack_1())
        .unwrap()
        + 1;
    let second_idx = ordered
        .iter()
        .position(|p| p == &make_div_pack_2())
        .unwrap()
        + 1;

    return first_idx * second_idx;
}

fn order_packets(pairs: &Vec<PacketPair>) -> Vec<Packet> {
    let mut packets = get_all_packets(pairs);

    packets.push(make_div_pack_1());
    packets.push(make_div_pack_2());

    packets.sort(); // sort smallest to largest.

    return packets;
}

fn get_all_packets(pairs: &Vec<PacketPair>) -> Vec<Packet> {
    let mut packets: Vec<Packet> = vec![];

    for p in pairs {
        packets.push(p.0.clone());
        packets.push(p.1.clone());
    }

    return packets;
}
