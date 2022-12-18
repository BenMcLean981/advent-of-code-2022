use std::{io::BufRead, str::FromStr};

use crate::pkg::utils::get_file_reader;

use super::{packet::Packet, packet_pair::PacketPair};

pub fn print_solutions() {
    let filename = "./src/pkg/day_13/input.txt";

    let pairs = get_packet_pairs(filename);
    let sum_of_in_order_pair_indices =
        get_sum_of_in_order_pair_indices(&pairs);

    println!("Day 13:");
    println!(
        "The sum of indices of packet pairs who are in order is {}.",
        sum_of_in_order_pair_indices
    );
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
