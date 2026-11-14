use super::error::ParseError;
use super::tile::{Rank, Seat, Suit};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Chow {
    M123 = 0x01,
    M234 = 0x02,
    M345 = 0x03,
    M456 = 0x04,
    M567 = 0x05,
    M678 = 0x06,
    M789 = 0x07,
    M34R = 0x08,
    M4R6 = 0x09,
    MR67 = 0x0A,

    P123 = 0x11,
    P234 = 0x12,
    P345 = 0x13,
    P456 = 0x14,
    P567 = 0x15,
    P678 = 0x16,
    P789 = 0x17,
    P34R = 0x18,
    P4R6 = 0x19,
    PR67 = 0x1A,

    S123 = 0x21,
    S234 = 0x22,
    S345 = 0x23,
    S456 = 0x24,
    S567 = 0x25,
    S678 = 0x26,
    S789 = 0x27,
    S34R = 0x28,
    S4R6 = 0x29,
    SR67 = 0x2A,

    M132 = 0x41,
    M243 = 0x42,
    M354 = 0x43,
    M465 = 0x44,
    M576 = 0x45,
    M687 = 0x46,
    M798 = 0x47,
    M3R4 = 0x48,
    M46R = 0x49,
    MR76 = 0x4A,

    P132 = 0x51,
    P243 = 0x52,
    P354 = 0x53,
    P465 = 0x54,
    P576 = 0x55,
    P687 = 0x56,
    P798 = 0x57,
    P3R4 = 0x58,
    P46R = 0x59,
    PR76 = 0x5A,

    S132 = 0x61,
    S243 = 0x62,
    S354 = 0x63,
    S465 = 0x64,
    S576 = 0x65,
    S687 = 0x66,
    S798 = 0x67,
    S3R4 = 0x68,
    S46R = 0x69,
    SR76 = 0x6A,

    M231 = 0x81,
    M342 = 0x82,
    M453 = 0x83,
    M564 = 0x84,
    M675 = 0x85,
    M786 = 0x86,
    M897 = 0x87,
    M4R3 = 0x88,
    MR64 = 0x89,
    M67R = 0x8A,

    P231 = 0x91,
    P342 = 0x92,
    P453 = 0x93,
    P564 = 0x94,
    P675 = 0x95,
    P786 = 0x96,
    P897 = 0x97,
    P4R3 = 0x98,
    PR64 = 0x99,
    P67R = 0x9A,

    S231 = 0xA1,
    S342 = 0xA2,
    S453 = 0xA3,
    S564 = 0xA4,
    S675 = 0xA5,
    S786 = 0xA6,
    S897 = 0xA7,
    S4R3 = 0xA8,
    SR64 = 0xA9,
    S67R = 0xAA,
}

impl Chow {
    pub fn new(suit: Suit, ranks: [Rank; 3]) -> Option<Self> {
        let base = (suit as u8) << 4;
        let offset = match ranks {
            [Rank::N1, Rank::N2, Rank::N3] => 0x01,
            [Rank::N2, Rank::N3, Rank::N4] => 0x02,
            [Rank::N3, Rank::N4, Rank::N5] => 0x03,
            [Rank::N4, Rank::N5, Rank::N6] => 0x04,
            [Rank::N5, Rank::N6, Rank::N7] => 0x05,
            [Rank::N6, Rank::N7, Rank::N8] => 0x06,
            [Rank::N7, Rank::N8, Rank::N9] => 0x07,
            [Rank::N3, Rank::N4, Rank::NR] => 0x08,
            [Rank::N4, Rank::NR, Rank::N6] => 0x09,
            [Rank::NR, Rank::N6, Rank::N7] => 0x0A,

            [Rank::N1, Rank::N3, Rank::N2] => 0x41,
            [Rank::N2, Rank::N4, Rank::N3] => 0x42,
            [Rank::N3, Rank::N5, Rank::N4] => 0x43,
            [Rank::N4, Rank::N6, Rank::N5] => 0x44,
            [Rank::N5, Rank::N7, Rank::N6] => 0x45,
            [Rank::N6, Rank::N8, Rank::N7] => 0x46,
            [Rank::N7, Rank::N9, Rank::N8] => 0x47,
            [Rank::N3, Rank::NR, Rank::N4] => 0x48,
            [Rank::N4, Rank::N6, Rank::NR] => 0x49,
            [Rank::NR, Rank::N7, Rank::N6] => 0x4A,

            [Rank::N2, Rank::N3, Rank::N1] => 0x81,
            [Rank::N3, Rank::N4, Rank::N2] => 0x82,
            [Rank::N4, Rank::N5, Rank::N3] => 0x83,
            [Rank::N5, Rank::N6, Rank::N4] => 0x84,
            [Rank::N6, Rank::N7, Rank::N5] => 0x85,
            [Rank::N7, Rank::N8, Rank::N6] => 0x86,
            [Rank::N8, Rank::N9, Rank::N7] => 0x87,
            [Rank::N4, Rank::NR, Rank::N3] => 0x88,
            [Rank::NR, Rank::N6, Rank::N4] => 0x89,
            [Rank::N6, Rank::N7, Rank::NR] => 0x8A,

            _ => return None,
        };
        let chow = base | offset;
        match suit {
            Suit::Z => None,
            _ => Some(unsafe { std::mem::transmute(chow) }),
        }
    }

    pub fn suit(self) -> Suit {
        unsafe { std::mem::transmute(((self as u8) >> 4) & 0x03) }
    }

    pub fn ranks(self) -> [Rank; 3] {
        match (self as u8) & 0xCF {
            0x01 => [Rank::N1, Rank::N2, Rank::N3],
            0x02 => [Rank::N2, Rank::N3, Rank::N4],
            0x03 => [Rank::N3, Rank::N4, Rank::N5],
            0x04 => [Rank::N4, Rank::N5, Rank::N6],
            0x05 => [Rank::N5, Rank::N6, Rank::N7],
            0x06 => [Rank::N6, Rank::N7, Rank::N8],
            0x07 => [Rank::N7, Rank::N8, Rank::N9],
            0x08 => [Rank::N3, Rank::N4, Rank::NR],
            0x09 => [Rank::N4, Rank::NR, Rank::N6],
            0x0A => [Rank::NR, Rank::N6, Rank::N7],

            0x41 => [Rank::N1, Rank::N3, Rank::N2],
            0x42 => [Rank::N2, Rank::N4, Rank::N3],
            0x43 => [Rank::N3, Rank::N5, Rank::N4],
            0x44 => [Rank::N4, Rank::N6, Rank::N5],
            0x45 => [Rank::N5, Rank::N7, Rank::N6],
            0x46 => [Rank::N6, Rank::N8, Rank::N7],
            0x47 => [Rank::N7, Rank::N9, Rank::N8],
            0x48 => [Rank::N3, Rank::NR, Rank::N4],
            0x49 => [Rank::N4, Rank::N6, Rank::NR],
            0x4A => [Rank::NR, Rank::N7, Rank::N6],

            0x81 => [Rank::N2, Rank::N3, Rank::N1],
            0x82 => [Rank::N3, Rank::N4, Rank::N2],
            0x83 => [Rank::N4, Rank::N5, Rank::N3],
            0x84 => [Rank::N5, Rank::N6, Rank::N4],
            0x85 => [Rank::N6, Rank::N7, Rank::N5],
            0x86 => [Rank::N7, Rank::N8, Rank::N6],
            0x87 => [Rank::N8, Rank::N9, Rank::N7],
            0x88 => [Rank::N4, Rank::NR, Rank::N3],
            0x89 => [Rank::NR, Rank::N6, Rank::N4],
            0x8A => [Rank::N6, Rank::N7, Rank::NR],

            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Pung {
    M111 = 0x01,
    M222 = 0x02,
    M333 = 0x03,
    M444 = 0x04,
    M555 = 0x05,
    M666 = 0x06,
    M777 = 0x07,
    M888 = 0x08,
    M999 = 0x09,
    M5R5 = 0x0A,
    M55R = 0x0B,

    P111 = 0x11,
    P222 = 0x12,
    P333 = 0x13,
    P444 = 0x14,
    P555 = 0x15,
    P666 = 0x16,
    P777 = 0x17,
    P888 = 0x18,
    P999 = 0x19,
    P5R5 = 0x1A,
    P55R = 0x1B,

    S111 = 0x21,
    S222 = 0x22,
    S333 = 0x23,
    S444 = 0x24,
    S555 = 0x25,
    S666 = 0x26,
    S777 = 0x27,
    S888 = 0x28,
    S999 = 0x29,
    S5R5 = 0x2A,
    S55R = 0x2B,

    Z111 = 0x31,
    Z222 = 0x32,
    Z333 = 0x33,
    Z444 = 0x34,
    Z555 = 0x35,
    Z666 = 0x36,
    Z777 = 0x37,
}

impl Pung {
    pub fn new(suit: Suit, ranks: [Rank; 3]) -> Option<Self> {
        let base = (suit as u8) << 4;
        let offset = match ranks {
            [Rank::N1, Rank::N1, Rank::N1] => 0x01,
            [Rank::N2, Rank::N2, Rank::N2] => 0x02,
            [Rank::N3, Rank::N3, Rank::N3] => 0x03,
            [Rank::N4, Rank::N4, Rank::N4] => 0x04,
            [Rank::N5, Rank::N5, Rank::N5] => 0x05,
            [Rank::N6, Rank::N6, Rank::N6] => 0x06,
            [Rank::N7, Rank::N7, Rank::N7] => 0x07,
            [Rank::N8, Rank::N8, Rank::N8] => 0x08,
            [Rank::N9, Rank::N9, Rank::N9] => 0x09,
            [Rank::N5, Rank::NR, Rank::N5] => 0x0A,
            [Rank::N5, Rank::N5, Rank::NR] => 0x0B,
            _ => return None,
        };
        let pung = base | offset;
        match suit {
            Suit::Z => (1 <= offset && offset <= 7).then(|| unsafe { std::mem::transmute(pung) }),
            _ => Some(unsafe { std::mem::transmute(pung) }),
        }
    }

    pub fn suit(self) -> Suit {
        unsafe { std::mem::transmute((self as u8) >> 4) }
    }

    pub fn ranks(self) -> [Rank; 3] {
        match (self as u8) & 0x0F {
            0x01 => [Rank::N1, Rank::N1, Rank::N1],
            0x02 => [Rank::N2, Rank::N2, Rank::N2],
            0x03 => [Rank::N3, Rank::N3, Rank::N3],
            0x04 => [Rank::N4, Rank::N4, Rank::N4],
            0x05 => [Rank::N5, Rank::N5, Rank::N5],
            0x06 => [Rank::N6, Rank::N6, Rank::N6],
            0x07 => [Rank::N7, Rank::N7, Rank::N7],
            0x08 => [Rank::N8, Rank::N8, Rank::N8],
            0x09 => [Rank::N9, Rank::N9, Rank::N9],
            0x0A => [Rank::N5, Rank::NR, Rank::N5],
            0x0B => [Rank::N5, Rank::N5, Rank::NR],
            _ => unreachable!(),
        }
    }

    pub fn extend(self) -> Kong {
        match self {
            Pung::M555 | Pung::P555 | Pung::S555 => unsafe {
                std::mem::transmute((self as u8) + 7)
            },
            _ => unsafe { std::mem::transmute(self as u8) },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kong {
    M1111 = 0x01,
    M2222 = 0x02,
    M3333 = 0x03,
    M4444 = 0x04,
    M5555 = 0x05,
    M6666 = 0x06,
    M7777 = 0x07,
    M8888 = 0x08,
    M9999 = 0x09,
    M5R55 = 0x0A,
    M55R5 = 0x0B,
    M555R = 0x0C,

    P1111 = 0x11,
    P2222 = 0x12,
    P3333 = 0x13,
    P4444 = 0x14,
    P5555 = 0x15,
    P6666 = 0x16,
    P7777 = 0x17,
    P8888 = 0x18,
    P9999 = 0x19,
    P5R55 = 0x1A,
    P55R5 = 0x1B,
    P555R = 0x1C,

    S1111 = 0x21,
    S2222 = 0x22,
    S3333 = 0x23,
    S4444 = 0x24,
    S5555 = 0x25,
    S6666 = 0x26,
    S7777 = 0x27,
    S8888 = 0x28,
    S9999 = 0x29,
    S5R55 = 0x2A,
    S55R5 = 0x2B,
    S555R = 0x2C,

    Z1111 = 0x31,
    Z2222 = 0x32,
    Z3333 = 0x33,
    Z4444 = 0x34,
    Z5555 = 0x35,
    Z6666 = 0x36,
    Z7777 = 0x37,
}

impl Kong {
    pub fn new(suit: Suit, ranks: [Rank; 4]) -> Option<Self> {
        let base = (suit as u8) << 4;
        let offset = match ranks {
            [Rank::N1, Rank::N1, Rank::N1, Rank::N1] => 0x01,
            [Rank::N2, Rank::N2, Rank::N2, Rank::N2] => 0x02,
            [Rank::N3, Rank::N3, Rank::N3, Rank::N3] => 0x03,
            [Rank::N4, Rank::N4, Rank::N4, Rank::N4] => 0x04,
            [Rank::N5, Rank::N5, Rank::N5, Rank::N5] => 0x05,
            [Rank::N6, Rank::N6, Rank::N6, Rank::N6] => 0x06,
            [Rank::N7, Rank::N7, Rank::N7, Rank::N7] => 0x07,
            [Rank::N8, Rank::N8, Rank::N8, Rank::N8] => 0x08,
            [Rank::N9, Rank::N9, Rank::N9, Rank::N9] => 0x09,
            [Rank::N5, Rank::NR, Rank::N5, Rank::N5] => 0x0A,
            [Rank::N5, Rank::N5, Rank::NR, Rank::N5] => 0x0B,
            [Rank::N5, Rank::N5, Rank::N5, Rank::NR] => 0x0C,
            _ => return None,
        };
        let kong = base | offset;
        match suit {
            Suit::Z => (1 <= offset && offset <= 7).then(|| unsafe { std::mem::transmute(kong) }),
            _ => Some(unsafe { std::mem::transmute(kong) }),
        }
    }

    pub fn suit(self) -> Suit {
        unsafe { std::mem::transmute((self as u8) >> 4) }
    }

    pub fn ranks(self) -> [Rank; 4] {
        match (self as u8) & 0x0F {
            0x01 => [Rank::N1, Rank::N1, Rank::N1, Rank::N1],
            0x02 => [Rank::N2, Rank::N2, Rank::N2, Rank::N2],
            0x03 => [Rank::N3, Rank::N3, Rank::N3, Rank::N3],
            0x04 => [Rank::N4, Rank::N4, Rank::N4, Rank::N4],
            0x05 => [Rank::N5, Rank::N5, Rank::N5, Rank::N5],
            0x06 => [Rank::N6, Rank::N6, Rank::N6, Rank::N6],
            0x07 => [Rank::N7, Rank::N7, Rank::N7, Rank::N7],
            0x08 => [Rank::N8, Rank::N8, Rank::N8, Rank::N8],
            0x09 => [Rank::N9, Rank::N9, Rank::N9, Rank::N9],
            0x0A => [Rank::N5, Rank::NR, Rank::N5, Rank::N5],
            0x0B => [Rank::N5, Rank::N5, Rank::NR, Rank::N5],
            0x0C => [Rank::N5, Rank::N5, Rank::N5, Rank::NR],
            _ => unreachable!("Invalid rank for Kong"),
        }
    }

    pub fn unextend(self) -> Pung {
        match self {
            Kong::M555R | Kong::P555R | Kong::S555R => unsafe {
                std::mem::transmute((self as u8) - 7)
            },
            _ => unsafe { std::mem::transmute(self as u8) },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Meld {
    OpChow { chow: Chow },
    OpPung { pung: Pung, giver: Seat },
    OpKong { kong: Kong, giver: Seat },
    ExKong { kong: Kong, giver: Seat },
    ClKong { kong: Kong },
}

impl std::fmt::Display for Meld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Meld::OpChow { chow } => {
                let (suit, ranks) = (chow.suit(), chow.ranks());
                match (*chow as u8) & 0xC0 {
                    0x00 => write!(f, "{}{}{}{}-", suit, ranks[0], ranks[1], ranks[2]),
                    0x40 => write!(f, "{}{}{}-{}", suit, ranks[0], ranks[2], ranks[1]),
                    0x80 => write!(f, "{}{}-{}{}", suit, ranks[2], ranks[0], ranks[1]),
                    _ => unreachable!(""),
                }
            }
            Meld::OpPung { pung, giver } => {
                let (suit, ranks, giver) = (pung.suit(), pung.ranks(), giver_str(*giver));
                write!(f, "{}{}{}{}{}", suit, ranks[0], ranks[1], ranks[2], giver,)
            }
            Meld::OpKong { kong, giver } => {
                let (suit, ranks, giver) = (kong.suit(), kong.ranks(), giver_str(*giver));
                write!(f, "{}{}{}{}{}{}", suit, ranks[0], ranks[1], ranks[2], ranks[3], giver)
            }
            Meld::ExKong { kong, giver } => {
                let (suit, ranks, giver) = (kong.suit(), kong.ranks(), giver_str(*giver));
                write!(f, "{}{}{}{}{}{}", suit, ranks[0], ranks[1], ranks[2], giver, ranks[3])
            }
            Meld::ClKong { kong } => {
                let (suit, ranks) = (kong.suit(), kong.ranks());
                write!(f, "{}{}{}{}{}", suit, ranks[0], ranks[1], ranks[2], ranks[3])
            }
        }
    }
}

fn giver_str(giver: Seat) -> &'static str {
    match giver {
        Seat::Rgt => "+",
        Seat::Acr => "=",
        Seat::Lft => "-",
    }
}

impl std::str::FromStr for Meld {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let invalid = || ParseError { message: format!("Invalid meld: {s}") };
        if !(s.len() == 5 || s.len() == 6) || !s.is_ascii() {
            return Err(invalid());
        }

        let bytes = s.as_bytes();
        let suit = parse_suit(bytes[0]).ok_or_else(invalid)?;
        let meld = match bytes.len() {
            5 => parse_opchow(bytes, suit)
                .or_else(|| parse_oppung(bytes, suit))
                .or_else(|| parse_clkong(bytes, suit)),
            6 => parse_opkong(bytes, suit).or_else(|| parse_exkong(bytes, suit)),
            _ => unreachable!(),
        };
        meld.ok_or_else(invalid)
    }
}

fn parse_opchow(bytes: &[u8], suit: Suit) -> Option<Meld> {
    [(0, [3, 4, 1]), (1, [1, 4, 2]), (2, [1, 2, 3])].into_iter().find_map(|(index, pos)| {
        if bytes[index as usize + 2] != b'-' {
            return None;
        }
        let ranks = parse_rank_array([bytes[pos[0]], bytes[pos[1]], bytes[pos[2]]])?;
        Some(Meld::OpChow { chow: Chow::new(suit, ranks)? })
    })
}

fn parse_oppung(bytes: &[u8], suit: Suit) -> Option<Meld> {
    let ranks = parse_rank_array([bytes[1], bytes[2], bytes[3]])?;
    Some(Meld::OpPung { pung: Pung::new(suit, ranks)?, giver: parse_giver(bytes[4])? })
}

fn parse_clkong(bytes: &[u8], suit: Suit) -> Option<Meld> {
    let ranks = parse_rank_array([bytes[1], bytes[2], bytes[3], bytes[4]])?;
    Some(Meld::ClKong { kong: Kong::new(suit, ranks)? })
}

fn parse_opkong(bytes: &[u8], suit: Suit) -> Option<Meld> {
    let ranks = parse_rank_array([bytes[1], bytes[2], bytes[3], bytes[4]])?;
    Some(Meld::OpKong { kong: Kong::new(suit, ranks)?, giver: parse_giver(bytes[5])? })
}

fn parse_exkong(bytes: &[u8], suit: Suit) -> Option<Meld> {
    let ranks = parse_rank_array([bytes[1], bytes[2], bytes[3], bytes[5]])?;
    Some(Meld::ExKong { kong: Kong::new(suit, ranks)?, giver: parse_giver(bytes[4])? })
}

fn parse_rank_array<const N: usize>(bytes: [u8; N]) -> Option<[Rank; N]> {
    let mut ranks = [Rank::NR; N];
    for (rank, byte) in ranks.iter_mut().zip(bytes) {
        *rank = parse_rank(byte)?;
    }
    Some(ranks)
}

fn parse_suit(byte: u8) -> Option<Suit> {
    match byte {
        b'm' => Some(Suit::M),
        b'p' => Some(Suit::P),
        b's' => Some(Suit::S),
        b'z' => Some(Suit::Z),
        _ => None,
    }
}

fn parse_rank(byte: u8) -> Option<Rank> {
    match byte {
        b'0' => Some(Rank::NR),
        b'1' => Some(Rank::N1),
        b'2' => Some(Rank::N2),
        b'3' => Some(Rank::N3),
        b'4' => Some(Rank::N4),
        b'5' => Some(Rank::N5),
        b'6' => Some(Rank::N6),
        b'7' => Some(Rank::N7),
        b'8' => Some(Rank::N8),
        b'9' => Some(Rank::N9),
        _ => None,
    }
}

fn parse_giver(byte: u8) -> Option<Seat> {
    match byte {
        b'+' => Some(Seat::Rgt),
        b'=' => Some(Seat::Acr),
        b'-' => Some(Seat::Lft),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{Chow, Kong, Meld, Pung};
    use super::{Chow::*, Kong::*, Pung::*};
    use crate::mahjong::tile::Rank;
    use crate::mahjong::tile::{Rank::*, Seat::*, Suit::*};

    #[rustfmt::skip]
    const CHOW_RANKS: [[Rank; 3]; 10] = [
        [N1, N2, N3], [N2, N3, N4], [N3, N4, N5], [N4, N5, N6], [N5, N6, N7],
        [N6, N7, N8], [N7, N8, N9], [N3, N4, NR], [N4, NR, N6], [NR, N6, N7],
    ];

    #[rustfmt::skip]
    const PUNG_RANKS: [[Rank; 3]; 11] = [
        [N1, N1, N1], [N2, N2, N2], [N3, N3, N3], [N4, N4, N4], [N5, N5, N5],
        [N6, N6, N6], [N7, N7, N7], [N8, N8, N8], [N9, N9, N9], [N5, N5, NR], [N5, NR, N5],
    ];

    #[rustfmt::skip]
    const KONG_RANKS: [[Rank; 4]; 12] = [
        [N1, N1, N1, N1], [N2, N2, N2, N2], [N3, N3, N3, N3], [N4, N4, N4, N4],
        [N5, N5, N5, N5], [N6, N6, N6, N6], [N7, N7, N7, N7], [N8, N8, N8, N8],
        [N9, N9, N9, N9], [N5, N5, N5, NR], [N5, N5, NR, N5], [N5, NR, N5, N5],
    ];

    fn assert_round_trip(meld: Meld) {
        let text = meld.to_string();
        assert_eq!(text.parse::<Meld>().ok(), Some(meld), "{text}");
    }

    #[test]
    fn chow_round_trip() {
        let mps_chows = [
            [M123, M234, M345, M456, M567, M678, M789, M34R, M4R6, MR67],
            [P123, P234, P345, P456, P567, P678, P789, P34R, P4R6, PR67],
            [S123, S234, S345, S456, S567, S678, S789, S34R, S4R6, SR67],
        ];

        for (suit, chows) in [M, P, S].into_iter().zip(mps_chows) {
            for (ranks, expected) in CHOW_RANKS.into_iter().zip(chows) {
                let chow = Chow::new(suit, ranks).expect("valid chow");
                assert_eq!(chow, expected);
                assert_eq!(chow.suit(), suit);
                assert_eq!(chow.ranks(), ranks);
                assert_round_trip(Meld::OpChow { chow });
            }
        }
    }

    #[test]
    fn pung_round_trip() {
        let mps_pungs = [
            [M111, M222, M333, M444, M555, M666, M777, M888, M999, M55R, M5R5],
            [P111, P222, P333, P444, P555, P666, P777, P888, P999, P55R, P5R5],
            [S111, S222, S333, S444, S555, S666, S777, S888, S999, S55R, S5R5],
        ];

        for (suit, pungs) in [M, P, S].into_iter().zip(mps_pungs) {
            for (ranks, expected) in PUNG_RANKS.into_iter().zip(pungs) {
                let pung = Pung::new(suit, ranks).expect("valid pung");
                assert_eq!(pung, expected);
                assert_eq!(pung.suit(), suit);
                assert_eq!(pung.ranks(), ranks);
                for giver in [Rgt, Acr, Lft] {
                    assert_round_trip(Meld::OpPung { pung, giver });
                }
            }
        }

        let z_pungs = [Z111, Z222, Z333, Z444, Z555, Z666, Z777];

        for (ranks, expected) in PUNG_RANKS.into_iter().zip(z_pungs) {
            let pung = Pung::new(Z, ranks).expect("valid pung");
            assert_eq!(pung, expected);
            assert_eq!(pung.suit(), Z);
            assert_eq!(pung.ranks(), ranks);
            for giver in [Rgt, Acr, Lft] {
                assert_round_trip(Meld::OpPung { pung, giver });
            }
        }
    }

    #[test]
    fn kong_round_trip() {
        let mps_kongs = [
            [M1111, M2222, M3333, M4444, M5555, M6666, M7777, M8888, M9999, M555R, M55R5, M5R55],
            [P1111, P2222, P3333, P4444, P5555, P6666, P7777, P8888, P9999, P555R, P55R5, P5R55],
            [S1111, S2222, S3333, S4444, S5555, S6666, S7777, S8888, S9999, S555R, S55R5, S5R55],
        ];

        for (suit, kongs) in [M, P, S].into_iter().zip(mps_kongs) {
            for (ranks, expected) in KONG_RANKS.into_iter().zip(kongs) {
                let kong = Kong::new(suit, ranks).expect("valid kong");
                assert_eq!(kong, expected);
                assert_eq!(kong.suit(), suit);
                assert_eq!(kong.ranks(), ranks);
                assert_round_trip(Meld::ClKong { kong });
                for giver in [Rgt, Acr, Lft] {
                    assert_round_trip(Meld::OpKong { kong, giver });
                    assert_round_trip(Meld::ExKong { kong, giver });
                }
            }
        }

        let z_kongs = [Z1111, Z2222, Z3333, Z4444, Z5555, Z6666, Z7777];

        for (ranks, expected) in KONG_RANKS.into_iter().zip(z_kongs) {
            let kong = Kong::new(Z, ranks).expect("valid kong");
            assert_eq!(kong, expected);
            assert_eq!(kong.suit(), Z);
            assert_eq!(kong.ranks(), ranks);
            assert_round_trip(Meld::ClKong { kong });
            for giver in [Rgt, Acr, Lft] {
                assert_round_trip(Meld::OpKong { kong, giver });
                assert_round_trip(Meld::ExKong { kong, giver });
            }
        }
    }

    #[test]
    fn pung_extend_and_kong_unextend_match_all_tile_combinations() {
        let mps_pungs = [
            [M111, M222, M333, M444, M555, M666, M777, M888, M999, M5R5, M55R],
            [P111, P222, P333, P444, P555, P666, P777, P888, P999, P5R5, P55R],
            [S111, S222, S333, S444, S555, S666, S777, S888, S999, S5R5, S55R],
        ];
        let mps_kongs = [
            [M1111, M2222, M3333, M4444, M555R, M6666, M7777, M8888, M9999, M5R55, M55R5],
            [P1111, P2222, P3333, P4444, P555R, P6666, P7777, P8888, P9999, P5R55, P55R5],
            [S1111, S2222, S3333, S4444, S555R, S6666, S7777, S8888, S9999, S5R55, S55R5],
        ];

        for (pungs, kongs) in mps_pungs.into_iter().zip(mps_kongs) {
            for (pung, kong) in pungs.into_iter().zip(kongs) {
                assert_eq!(pung.extend(), kong);
                assert_eq!(kong.unextend(), pung);
            }
        }

        let z_pungs = [Z111, Z222, Z333, Z444, Z555, Z666, Z777];
        let z_kongs = [Z1111, Z2222, Z3333, Z4444, Z5555, Z6666, Z7777];

        for (pung, kong) in z_pungs.into_iter().zip(z_kongs) {
            assert_eq!(pung.extend(), kong);
            assert_eq!(kong.unextend(), pung);
        }
    }

    #[test]
    fn meld_display_uses_expected_call_positions() {
        assert_eq!(Meld::OpChow { chow: Chow::M123 }.to_string(), "m123-");
        assert_eq!(Meld::OpChow { chow: Chow::M132 }.to_string(), "m12-3");
        assert_eq!(Meld::OpChow { chow: Chow::M231 }.to_string(), "m1-23");
        assert_eq!(Meld::OpPung { pung: Pung::M555, giver: Rgt }.to_string(), "m555+");
        assert_eq!(Meld::OpKong { kong: Kong::M1111, giver: Acr }.to_string(), "m1111=");
        assert_eq!(Meld::ExKong { kong: Kong::M1111, giver: Lft }.to_string(), "m111-1");
        assert_eq!(Meld::ClKong { kong: Kong::M1111 }.to_string(), "m1111");
    }
}
