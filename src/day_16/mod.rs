use bitvec::prelude::*;
use bitvec::ptr::Const;

fn bits_to_usize<'a, T: Clone + IntoIterator<Item = BitRef<'a, Const>>>(bits: &T) -> usize {
    bits.clone()
        .into_iter()
        .fold(0, |sum, bit| (sum << 1) + (*bit as usize))
}

#[derive(Debug)]
pub enum PacketContent {
    Literal(usize),
    Operator { subpackets: Vec<Packet> },
}

#[derive(Debug)]
pub struct Packet {
    version: u8,
    type_id: u8,
    content: PacketContent,
}

impl Packet {
    fn sum_versions(&self) -> usize {
        match &self.content {
            PacketContent::Literal(_) => self.version as usize,
            PacketContent::Operator { subpackets } => subpackets
                .iter()
                .fold(self.version as usize, |acc, packet| {
                    acc + (packet.sum_versions() as usize)
                }),
        }
    }

    fn value(&self) -> usize {
        match &self.content {
            PacketContent::Literal(value) => *value,
            #[rustfmt::skip]
            PacketContent::Operator { subpackets } => match self.type_id {
                0 => subpackets.iter().map(|packet| packet.value()).sum(),
                1 => subpackets.iter().map(|packet| packet.value()).product(),
                2 => subpackets.iter().map(|packet| packet.value()).min().unwrap(),
                3 => subpackets.iter().map(|packet| packet.value()).max().unwrap(),
                5 => if subpackets[0].value() > subpackets[1].value() { 1 } else { 0 },
                6 => if subpackets[0].value() < subpackets[1].value() { 1 } else { 0 },
                7 => if subpackets[0].value() == subpackets[1].value() { 1 } else { 0 },
                _ => 0,
            },
        }
    }
}

fn str_to_bitvec(s: &str) -> Result<BitVec, String> {
    Ok(s.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| Some(c.to_digit(16)?))
        .collect::<Option<Vec<u32>>>()
        .ok_or("parse error")?
        .iter()
        .map(|n| [1 & n >> 3, 1 & n >> 2, 1 & n >> 1, 1 & n])
        .flatten()
        .map(|n| n == 1)
        .collect())
}

fn packets_from(bits: BitVec) -> Result<(Packet, BitVec), String> {
    let (version_bits, rest) = bits.split_at(3);
    let version = bits_to_usize(&version_bits) as u8;
    let (type_id_bits, rest) = rest.split_at(3);
    let type_id = bits_to_usize(&type_id_bits) as u8;
    let mut rest = rest;
    let content = if type_id == 4 {
        let literal_chunks: Vec<usize> = rest
            .chunks(5)
            .scan(false, |done, chunk| {
                if *done {
                    return None;
                }
                if !chunk[0] {
                    *done = true;
                }
                Some(chunk)
            })
            .map(|chunk| {
                bits_to_usize(
                    &chunk
                        .into_iter()
                        .collect::<Vec<_>>()
                        .drain(1..)
                        .collect::<Vec<_>>()
                        .into_iter(),
                )
            })
            .collect();
        let literal = literal_chunks.iter().fold(0, |acc, n| (acc << 4) + n);
        rest = &rest[literal_chunks.len() * 5..];
        PacketContent::Literal(literal)
    } else {
        let split = rest.split_at(1);
        let length_type_bit = split.0;
        rest = split.1;
        let mut subpackets = Vec::new();
        if length_type_bit == bits![1] {
            let split = rest.split_at(11);
            rest = split.1;
            let n_subpackets = bits_to_usize(&split.0);
            for _ in 0..n_subpackets {
                let step = packets_from(BitVec::from_bitslice(rest).into())?;
                subpackets.push(step.0);
                rest = &rest[rest.len() - step.1.len()..];
            }
        } else {
            let split = rest.split_at(15);
            rest = split.1;
            let total_length = bits_to_usize(&split.0);
            let mut bits: BitVec = rest.iter().take(total_length).collect();
            while !bits.is_empty() {
                let step = packets_from(bits)?;
                subpackets.push(step.0);
                bits = step.1;
            }
            rest = &rest[total_length..];
        };
        PacketContent::Operator {
            subpackets: subpackets,
        }
    };
    Ok((
        Packet {
            version: version,
            type_id: type_id,
            content: content,
        },
        rest.into(),
    ))
}

#[aoc_generator(day16)]
pub fn get_input(input: &str) -> Packet {
    packets_from(str_to_bitvec(input).unwrap()).unwrap().0
}

#[aoc(day16, part1)]
pub fn part_1(packet: &Packet) -> usize {
    packet.sum_versions()
}

#[aoc(day16, part2)]
pub fn part_2(packet: &Packet) -> usize {
    packet.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input("8A004A801A8002F478")), 16);
        assert_eq!(part_1(&get_input("620080001611562C8802118E34")), 12);
        assert_eq!(part_1(&get_input("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part_1(&get_input("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input("C200B40A82")), 3);
        assert_eq!(part_2(&get_input("04005AC33890")), 54);
        assert_eq!(part_2(&get_input("880086C3E88112")), 7);
        assert_eq!(part_2(&get_input("CE00C43D881120")), 9);
        assert_eq!(part_2(&get_input("D8005AC2A8F0")), 1);
        assert_eq!(part_2(&get_input("F600BC2D8F")), 0);
        assert_eq!(part_2(&get_input("9C005AC2F8F0")), 0);
        assert_eq!(part_2(&get_input("9C0141080250320F1802104A08")), 1);
    }
}
