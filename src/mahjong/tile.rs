use std::collections::hash_map;
use std::hash::{Hash, Hasher};
use std::{fmt, slice::Windows, usize::MAX, iter::empty, collections::HashMap, };
use int_enum::IntEnum;


// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------
//                         Suit and SuitValue for use with Tile
// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------



#[allow(dead_code)]
#[derive(EnumIter, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Suit {
    Man,
    Pin,
    Sou,
    Honor
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Man => write!(f, "M"),
            Suit::Pin => write!(f, "P"),
            Suit::Sou => write!(f, "S"),
            Suit::Honor => write!(f, "H")
        }
    }
}

#[repr(i8)]
#[allow(dead_code)]
#[derive(EnumIter, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, IntEnum, Hash)]
pub enum SuitVal {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,

    North = 10,
    East = 11,
    South = 12,
    West = 13,

    Red = 14,
    White = 15,
    Green = 16
}

impl SuitVal {
    pub fn is_before_num(&self, other : Self) -> bool
    {
        match self {
            SuitVal::One => other == SuitVal::Two,
            SuitVal::Two => other == SuitVal::Three,
            SuitVal::Three => other == SuitVal::Four,
            SuitVal::Four => other == SuitVal::Five,
            SuitVal::Five => other == SuitVal::Six,
            SuitVal::Six => other == SuitVal::Seven,
            SuitVal::Seven => other == SuitVal::Eight,
            SuitVal::Eight => other == SuitVal::Nine,
            _ => false,
        }
    }

    pub fn is_after_num(&self, other : Self) -> bool
    {
        other.is_before_num(*self)
    }

    pub fn get_prev_num(&self) -> Option<Self>
    {
        match self {
            SuitVal::Two => Some(SuitVal::One),
            SuitVal::Three => Some(SuitVal::Two),
            SuitVal::Four => Some(SuitVal::Three),
            SuitVal::Five => Some(SuitVal::Four),
            SuitVal::Six => Some(SuitVal::Five),
            SuitVal::Seven => Some(SuitVal::Six),
            SuitVal::Eight => Some(SuitVal::Seven),
            SuitVal::Nine => Some(SuitVal::Eight),
            _ => None,
        }
    }

    pub fn get_next_num(&self) -> Option<Self>
    {
        match self {
            SuitVal::One => Some(SuitVal::Two),
            SuitVal::Two => Some(SuitVal::Three),
            SuitVal::Three => Some(SuitVal::Four),
            SuitVal::Four => Some(SuitVal::Five),
            SuitVal::Five => Some(SuitVal::Six),
            SuitVal::Six => Some(SuitVal::Seven),
            SuitVal::Seven => Some(SuitVal::Eight),
            SuitVal::Eight => Some(SuitVal::Nine),
            _ => None,
        }
    }
}

impl fmt::Display for SuitVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SuitVal::One => write!(f, "1"),
            SuitVal::Two => write!(f, "2"),
            SuitVal::Three => write!(f, "3"),
            SuitVal::Four => write!(f, "4"),
            SuitVal::Five => write!(f, "5"),
            SuitVal::Six => write!(f, "6"),
            SuitVal::Seven => write!(f, "7"),
            SuitVal::Eight => write!(f, "8"),
            SuitVal::Nine => write!(f, "9"),
            
            SuitVal::North => write!(f, "North"),
            SuitVal::East => write!(f, "East"),
            SuitVal::South => write!(f, "South"),
            SuitVal::West => write!(f, "West"),

            SuitVal::Red => write!(f, "Red"),
            SuitVal::White => write!(f, "White"),
            SuitVal::Green => write!(f, "Green"),
        }
    }
}













// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------
//                                          Tile Struct
// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------



#[allow(dead_code)]
#[derive(Clone, Copy, Eq, Ord, PartialOrd)]
pub struct Tile {
    pub suit : Suit,
    pub value : SuitVal,
    pub red : bool,
}

impl Tile {
    pub fn get_prev_num_tile(&self) -> Option<Tile>
    {
        match self.value.get_prev_num() {
            Some(prev_value) => Some(Tile {
                suit : self.suit,
                value : prev_value,
                red : false,
            }),
            None => None
        }
    }
    
    pub fn get_next_num_tile(&self) -> Option<Tile>
    {
        match self.value.get_next_num() {
            Some(next_value) => Some(Tile {
                suit : self.suit,
                value : next_value,
                red : false,
            }),
            None => None
        }
    }
}

/// Tiles should hash the same regardless of whether they are red or not
impl Hash for Tile {
    fn hash<H: Hasher>(&self, state : &mut H) {
        self.suit.hash(state);
        self.value.hash(state);
    }
}

impl PartialEq for Tile {
    fn eq(&self, other : &Self) -> bool {
        self.suit == other.suit && self.value == other.value
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        match self.red {
            true => write!(f, "[{}:{}:r]", self.suit, self.value),
            false => write!(f, "[{}:{}]", self.suit, self.value),
        }
    }
}

pub const INVALID_TILE : Tile = Tile { suit : Suit::Man, value : SuitVal::East, red : true };

pub fn print_tiles(tiles : &[Tile], num_to_print : usize) -> ()
{
    for i in 0..(num_to_print-1)
    {
        print!("{},", tiles[i as usize]);
    }

    println!("{}", tiles[num_to_print - 1]);
}



















// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------
//                        Set and SetType (a valid pairing of tiles) 
// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------


#[derive(EnumIter, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SetType {
    Pair,
    Sequence,
    Triplet,
    ClosedKan,
    OpenKan,
}

// A completed tile set
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Set {
    pub set_type : SetType,
    pub tiles : Vec<Tile>,
    pub ron : bool,
}

impl Set {
    pub fn has_honor_or_terminal(&self) -> bool
    {
        for tile in &self.tiles {
            if tile.suit == Suit::Honor || tile.value == SuitVal::One || tile.value == SuitVal::Nine
            {
                return true;
            }
        }
    
        return false;
    }
}




















// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------
//                                          Waits and Calls
// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------

#[derive(Clone, Eq, PartialEq)]
pub enum WaitType {
    Ryanmen, // double sided sequence
    Penchan, // One sided wait of sequence (sequence has terminal)
    Shanpon, // either of two pairs to form a triplet
    Kanchan, // middle of sequence
    Tanki, // pair wait
}

/// Designates which call is used, and also contains
/// tile info for chii is to know which tiles the player wants to combine it with
/// since one chii could have many possibilities with one player
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CallTypes {
    Tsumo,
    Ron,
    Pon,
    Kan,
    Chii(Tile, Tile),
}

impl CallTypes {
    pub fn precedence(&self) -> usize {
        match self {
            CallTypes::Tsumo | CallTypes::Ron => 2,
            CallTypes::Pon | CallTypes::Kan => 1,
            CallTypes::Chii(_,_) => 0,
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Calls {
    pub chii : bool,
    pub pon : bool,
    pub kan : bool,
    /// TODO: Might need to replace Pair with Ron
    pub pair : bool,
}

impl Calls {
    pub fn any_field_true(&self) -> bool
    {
        self.chii || self.pon || self.kan || self.pair
    }
}

impl Default for Calls {
    fn default() -> Self {
        Calls { chii: false, pon: false, kan: false, pair : false }
    }
}


