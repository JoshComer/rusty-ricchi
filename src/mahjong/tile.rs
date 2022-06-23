use std::cmp::Ordering;
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd)]
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


#[derive(EnumIter, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SetType {
    Pair,
    Sequence,
    Triplet,
    Kan,
}

// A completed tile set
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Set {
    pub set_type : SetType,
    pub tiles : Vec<Tile>,
}

impl Set {
    pub fn invalid_default() -> Self {
        Set {
            set_type: SetType::Kan,
            tiles: vec![INVALID_TILE ; 4]
        }
    }

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


#[derive(Clone, Eq, PartialEq)]
pub struct CalledSet {
    pub set : Set,
    pub call_type : CallTypes,
}

pub fn find_possible_sets_with_tile(tile : Tile, hand_without_tile : &[Tile]) -> Vec<Set>
{
    let mut possible_sets : Vec<Set> = vec![];

    // find possible sets with multiple of same tile
    let num_this_tile = hand_without_tile.iter().filter(|hand_tile| **hand_tile == tile).count();

    // add pair set
    if num_this_tile >= 1
    {
        possible_sets.push(
            Set {
                set_type: SetType::Pair,
                tiles: vec![tile ; 2]
            },
        );
    }

    // add triplet set
    if num_this_tile >= 2
    {
        possible_sets.push(
            Set {
                set_type: SetType::Triplet,
                tiles: vec![tile ; 3]
            }
        );
    }

    // add kan set
    if num_this_tile >= 3
    {
        possible_sets.push(
            Set {
                set_type: SetType::Kan,
                tiles: vec![tile ; 4]
            }
        );
    }



    // find possible sets with sequence of tile
    if tile.suit != Suit::Honor && tile.value != SuitVal::Eight && tile.value != SuitVal::Nine
    {   // since hand is sorted, and we're looking left to right, only check rightwards

        let tiles_next = hand_without_tile.iter().find(
            |hand_tile| **hand_tile == tile.get_next_num_tile().unwrap()
        );

        if let Some(tiles_next) = tiles_next
        {
            let tiles_next_next = hand_without_tile.iter().find(
                |hand_tile| **hand_tile == tiles_next.get_next_num_tile().unwrap()
            );

            if let Some(tiles_next_next) = tiles_next_next
            {
                possible_sets.push(Set {
                    tiles : vec![tile, *tiles_next, *tiles_next_next],
                    set_type : SetType::Sequence
                });
            }
        }
    }

    return possible_sets;
}




pub fn get_callable_chii_combinations_with_tile(hand : &[Tile], tile : Tile) -> Vec<CalledSet>
{
    let mut last_numbered_tile_idx = hand.iter().rposition(
        |find_tile| find_tile.suit != Suit::Honor
    );

    let mut ret_vec : Vec<CalledSet> = vec![];

    let chii_set : CalledSet = CalledSet { set:
        Set { set_type: SetType::Sequence, tiles: vec![] }
        , call_type: CallTypes::Chii };



    let prev_tile = hand.iter().find(|find_tile|
        **find_tile == tile.get_prev_num_tile().unwrap_or(INVALID_TILE)
    );
    let mut prev_prev_tile : Option<&Tile> = None;

    if let Some(prev_tile) = prev_tile
    {
        prev_prev_tile = hand.iter().find(|find_tile|
            **find_tile == prev_tile.get_prev_num_tile().unwrap_or(INVALID_TILE)
        );
    }

    let next_tile = hand.iter().find(|find_tile|
        **find_tile == tile.get_next_num_tile().unwrap_or(INVALID_TILE)
    );
    let mut next_next_tile : Option<&Tile> = None;

    if let Some(next_tile) = next_tile
    {
        next_next_tile = hand.iter().find(|find_tile|
            **find_tile == next_tile.get_next_num_tile().unwrap_or(INVALID_TILE)
        );
    }

    if prev_prev_tile.is_some() && prev_tile.is_some()
    {
        ret_vec.push(CalledSet {
            set : Set {
                tiles : vec![*prev_prev_tile.unwrap(), *prev_tile.unwrap(), tile],
                ..chii_set.set
            },
            ..chii_set});
    }

    if prev_tile.is_some() && next_tile.is_some()
    {
        ret_vec.push(CalledSet {
            set : Set {
                tiles : vec![*prev_tile.unwrap(), tile, *next_tile.unwrap()],
                ..chii_set.set
            },
            ..chii_set});
    }

    if next_tile.is_some() && next_next_tile.is_some()
    {
        ret_vec.push(CalledSet {
            set : Set {
                tiles : vec![tile, *next_tile.unwrap(), *next_next_tile.unwrap()],
                ..chii_set.set
            },
            ..chii_set});
    }

    return ret_vec;
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
    Ron(SetType),
    Pon,
    OpenKan,
    ClosedKan,
    AddedKan,
    Chii,
}

impl CallTypes {
    pub fn precedence(&self) -> usize {
        match self {
            CallTypes::Tsumo => 4,
            CallTypes::AddedKan => 3, //TODO: If a player calls on kokushi musou, addedkan shouldn't have higher precedence
            CallTypes::Ron(_) => 2,
            CallTypes::Pon | CallTypes::OpenKan | CallTypes::ClosedKan => 1,
            CallTypes::Chii => 0,
        }
    }
}

impl Ord for CallTypes {
    fn cmp(&self, other : &Self) -> Ordering {
        self.precedence().cmp(&other.precedence())
    }
}

impl PartialOrd for CallTypes {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Calls {
    pub chii : bool,
    pub pon : bool,
    pub open_kan : bool, // call on discarded fourth tile with closed triplet in hand
    pub added_kan : bool, // adding fourth drawn tile to open triplet
    pub closed_kan : bool, // closed entirely
    pub ron : bool,
    pub ron_set : Set,
}

impl Calls {
    pub fn any_field_true(&self) -> bool
    {
        self.chii || self.pon || self.open_kan|| self.added_kan || self.closed_kan || self.ron
    }
}

impl Default for Calls {
    fn default() -> Self {
        Calls { chii: false, pon: false, open_kan : false, added_kan : false, closed_kan : false, ron : false, ron_set : Set::invalid_default()}
    }
}

/// Takes a hand in tenpai, and returns the tiles which would complete the hand and give it a win
/// TODO: Make compatible with yakuman, and not just basic complete hands
pub fn get_winning_tiles_from_tenpai_hand(hand : &Vec<Tile>, winning_configurations : Vec<Vec<Set>>) -> Vec<(Tile, Set)>
{
    let mut winning_tiles : Vec<(Tile, Set)> = vec![];

    for winning_sets in winning_configurations
    {
        let mut found_tile_map = vec![false ; hand.len()];

        // map tiles used in sets - to tiles in the hand, in order to find which tiles aren't used by the winning configurations
        for set in winning_sets
        {
            for tile in &set.tiles
            {
                let mut idx = hand.iter().position(|hand_tile| *hand_tile == *tile).expect("Error: Hand did not have tile in winning set");

                // since there can be duplicates of tiles, move past the ones we've already found from previous sets
                while found_tile_map[idx] == true
                {   idx += 1;   }

                if hand[idx] != *tile // sanity check
                {   panic!("Wtf?");  }

                found_tile_map[idx] = true;
            }
        }

        let first_unclaimed_tile_pos = found_tile_map.iter().position(|check_bool| *check_bool == false).expect("Error: No unclaimed tile in hand");

        // TODO: this is because there's no way to get the position of a second element like the first above. Very ugly, might need to change.
        let mut tmp_idx = first_unclaimed_tile_pos + 1;
        let second_unclaimed_tile_pos = loop {
            if tmp_idx >= found_tile_map.len() { break None; }
            else if found_tile_map[tmp_idx] == false { break Some(tmp_idx); }
            else { tmp_idx += 1; }
        };


        // Not waiting on pair, need to find which tile completes the set
        if let Some(second_unclaimed_tile_pos) = second_unclaimed_tile_pos
        {
            let first_tile = hand[first_unclaimed_tile_pos];
            let second_tile = hand[second_unclaimed_tile_pos];

            // need a triplet to complete the hand
            if first_tile == second_tile
            {
                winning_tiles.push((first_tile, Set {
                    set_type : SetType::Triplet,
                    tiles : vec![first_tile ; 3],
                }));
            }
            // need a sequence to complete the hand
            else
            {
                let prev_in_seq = if first_tile < second_tile { first_tile } else { second_tile };
                let next_in_seq = if first_tile < second_tile { second_tile } else { first_tile };

                let prev_prev_for_seq = prev_in_seq.get_prev_num_tile();
                let next_next_for_seq = next_in_seq .get_next_num_tile();

                if prev_prev_for_seq.is_none() && next_next_for_seq.is_none()
                {
                    panic!("Error: Winning tile was needed for sequence, but instead was not possible. Tiles are {} and {}", prev_in_seq, next_in_seq);
                }

                if let Some(prev_prev_for_seq) = prev_prev_for_seq
                {
                    winning_tiles.push((prev_prev_for_seq, Set {
                        set_type : SetType::Sequence,
                        tiles : vec![prev_prev_for_seq, prev_in_seq, next_in_seq]
                    }));
                }

                if let Some(next_next_for_seq) = next_next_for_seq
                {
                    winning_tiles.push((next_next_for_seq, Set{
                        set_type : SetType::Sequence,
                        tiles : vec![prev_in_seq, next_in_seq, next_next_for_seq]
                    }));
                }
            }
        }
        // waiting on pair
        else
        {
            // push the tile not in any set to the winning tiles. We need a pair with that tile
            let pair_tile = hand[first_unclaimed_tile_pos];

            winning_tiles.push((pair_tile, Set {
                set_type : SetType::Pair,
                tiles : vec![pair_tile ; 2]
            }));
        }

    }

    return winning_tiles;
}