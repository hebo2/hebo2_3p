use super::error::ParseError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Suit {
    M = 0x00,
    P = 0x01,
    S = 0x02,
    Z = 0x03,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self {
            Suit::M => 'm',
            Suit::P => 'p',
            Suit::S => 's',
            Suit::Z => 'z',
        };
        write!(f, "{suit}")
    }
}

impl std::str::FromStr for Suit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "m" => Ok(Suit::M),
            "p" => Ok(Suit::P),
            "s" => Ok(Suit::S),
            "z" => Ok(Suit::Z),
            _ => Err(ParseError { message: format!("Invalid suit: {s}") }),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Rank {
    NR = 0x00,
    N1 = 0x01,
    N2 = 0x02,
    N3 = 0x03,
    N4 = 0x04,
    N5 = 0x05,
    N6 = 0x06,
    N7 = 0x07,
    N8 = 0x08,
    N9 = 0x09,
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl std::str::FromStr for Rank {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Rank::NR),
            "1" => Ok(Rank::N1),
            "2" => Ok(Rank::N2),
            "3" => Ok(Rank::N3),
            "4" => Ok(Rank::N4),
            "5" => Ok(Rank::N5),
            "6" => Ok(Rank::N6),
            "7" => Ok(Rank::N7),
            "8" => Ok(Rank::N8),
            "9" => Ok(Rank::N9),
            _ => Err(ParseError { message: format!("Invalid rank: {s}") }),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Tile {
    MR = 0x00,
    M1 = 0x01,
    M2 = 0x02,
    M3 = 0x03,
    M4 = 0x04,
    M5 = 0x05,
    M6 = 0x06,
    M7 = 0x07,
    M8 = 0x08,
    M9 = 0x09,

    PR = 0x10,
    P1 = 0x11,
    P2 = 0x12,
    P3 = 0x13,
    P4 = 0x14,
    P5 = 0x15,
    P6 = 0x16,
    P7 = 0x17,
    P8 = 0x18,
    P9 = 0x19,

    SR = 0x20,
    S1 = 0x21,
    S2 = 0x22,
    S3 = 0x23,
    S4 = 0x24,
    S5 = 0x25,
    S6 = 0x26,
    S7 = 0x27,
    S8 = 0x28,
    S9 = 0x29,

    Z1 = 0x31,
    Z2 = 0x32,
    Z3 = 0x33,
    Z4 = 0x34,
    Z5 = 0x35,
    Z6 = 0x36,
    Z7 = 0x37,
}

impl Tile {
    pub fn new(suit: Suit, rank: Rank) -> Option<Tile> {
        use Rank::*;
        let tile = (suit as u8) << 4 | rank as u8;
        match suit {
            Suit::Z => (N1 <= rank && rank <= N7).then(|| unsafe { std::mem::transmute(tile) }),
            _ => Some(unsafe { std::mem::transmute(tile) }),
        }
    }

    pub fn suit(self) -> Suit {
        unsafe { std::mem::transmute((self as u8) >> 4) }
    }

    pub fn rank(self) -> Rank {
        unsafe { std::mem::transmute((self as u8) & 0x0F) }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit(), self.rank())
    }
}

impl std::str::FromStr for Tile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let invalid = || ParseError { message: format!("Invalid tile: {s}") };
        if s.len() != 2 || !s.is_ascii() {
            return Err(invalid());
        }

        let suit = s[0..1].parse::<Suit>().map_err(|_| invalid())?;
        let rank = s[1..2].parse::<Rank>().map_err(|_| invalid())?;
        Tile::new(suit, rank).ok_or_else(|| invalid())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Seat {
    Rgt = 0x01,
    Acr = 0x02,
    Lft = 0x03,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Drop {
    pub target: Tile,
    pub is_imm: bool,
    pub is_rot: bool,
    pub caller: Option<Seat>,
}

impl std::fmt::Display for Drop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_imm = if self.is_imm { "_" } else { "" };
        let is_rot = if self.is_rot { "*" } else { "" };
        let caller = match self.caller {
            None => "",
            Some(Seat::Rgt) => "-",
            Some(Seat::Acr) => "=",
            Some(Seat::Lft) => "+",
        };
        write!(f, "{}{}{}{}", self.target, is_imm, is_rot, caller)
    }
}

impl std::str::FromStr for Drop {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let invalid = || ParseError { message: format!("Invalid drop: {s}") };
        if !(2 <= s.len() && s.len() <= 5) || !s.is_ascii() {
            return Err(invalid());
        }

        let tile = s[..2].parse::<Tile>().map_err(|_| invalid())?;
        let prop = &s[2..];
        let (is_imm, prop) = prop.strip_prefix("_").map_or((false, prop), |s| (true, s));
        let (is_rot, prop) = prop.strip_prefix("*").map_or((false, prop), |s| (true, s));
        let caller = match prop {
            "" => None,
            "-" => Some(Seat::Rgt),
            "=" => Some(Seat::Acr),
            "+" => Some(Seat::Lft),
            _ => return Err(invalid()),
        };

        Ok(Drop { target: tile, is_imm, is_rot, caller })
    }
}

#[cfg(test)]
mod tests {
    use super::{Drop, Rank, Suit, Tile};
    use super::{Rank::*, Seat::*, Suit::*, Tile::*};

    const RANKS: [Rank; 10] = [NR, N1, N2, N3, N4, N5, N6, N7, N8, N9];
    const RANK_STRS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    #[rustfmt::skip]
    const TILES: [Tile; 37] = [
        MR, M1, M2, M3, M4, M5, M6, M7, M8, M9,
        PR, P1, P2, P3, P4, P5, P6, P7, P8, P9,
        SR, S1, S2, S3, S4, S5, S6, S7, S8, S9,
            Z1, Z2, Z3, Z4, Z5, Z6, Z7,
    ];

    #[test]
    fn suit_round_trip() {
        for (suit, text) in [(M, "m"), (P, "p"), (S, "s"), (Z, "z")] {
            assert_eq!(suit.to_string(), text);
            assert_eq!(text.parse::<Suit>().ok(), Some(suit));
        }
    }

    #[test]
    fn rank_round_trip() {
        for (rank, text) in RANKS.into_iter().zip(RANK_STRS) {
            assert_eq!(rank.to_string(), text);
            assert_eq!(text.parse::<Rank>().ok(), Some(rank));
        }
    }

    #[test]
    fn tile_round_trip() {
        for tile in TILES {
            assert_eq!(Tile::new(tile.suit(), tile.rank()), Some(tile));
            assert_eq!(tile.to_string(), format!("{}{}", tile.suit(), tile.rank()));
            assert_eq!(tile.to_string().parse::<Tile>().ok(), Some(tile));
        }
    }

    #[test]
    fn drop_round_trip() {
        let callers = [(None, ""), (Some(Rgt), "-"), (Some(Acr), "="), (Some(Lft), "+")];

        for target in TILES {
            for is_imm in [false, true] {
                for is_rot in [false, true] {
                    for (caller, caller_suffix) in callers {
                        let drop = Drop { target, is_imm, is_rot, caller };
                        let expected = format!(
                            "{}{}{}{}",
                            target,
                            if is_imm { "_" } else { "" },
                            if is_rot { "*" } else { "" },
                            caller_suffix
                        );

                        assert_eq!(drop.to_string(), expected);
                        assert_eq!(expected.parse::<Drop>().ok(), Some(drop));
                    }
                }
            }
        }
    }
}
