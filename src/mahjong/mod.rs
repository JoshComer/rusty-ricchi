//pub mod game_structs {
use strum::IntoEnumIterator;
use rand::{Rng, rngs::adapter::ReseedingRng};
use unicode_segmentation::UnicodeSegmentation;
use std::{fmt, slice::Windows, usize::MAX, iter::empty, collections::HashMap, };
use int_enum::IntEnum;
use num::{pow, bigint::ParseBigIntError, One};

use std::collections::hash_map;
use std::hash::{Hash, Hasher};


pub mod tui_output;




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
    fn is_before_num(&self, other : Self) -> bool
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

    fn is_after_num(&self, other : Self) -> bool
    {
        other.is_before_num(*self)
    }

    fn get_prev_num(&self) -> Option<Self>
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

    fn get_next_num(&self) -> Option<Self>
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


#[allow(dead_code)]
#[derive(Clone, Copy, Eq, Ord, PartialOrd)]
pub struct Tile {
    pub suit : Suit,
    pub value : SuitVal,
    pub red : bool,
}

impl Tile {
    fn get_prev_num_tile(&self) -> Option<Tile>
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
    
    fn get_next_num_tile(&self) -> Option<Tile>
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

#[derive(EnumIter, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SetType {
    Pair,
    Sequence,
    Triplet,
    ClosedKan,
    OpenKan,
}

// A completed tile set
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Set {
    pub set_type : SetType,
    tiles : [Tile ; 4],
    ron : bool,
}

impl Set {
    fn has_honor_or_terminal(&self) -> bool
    {
        for tile in self.tiles {
            if tile.suit == Suit::Honor || tile.value == SuitVal::One || tile.value == SuitVal::Nine
            {
                return true;
            }
        }
    
        return false;
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum WaitType {
    Ryanmen, // double sided sequence
    Penchan, // One sided wait of sequence (sequence has terminal)
    Shanpon, // either of two pairs to form a triplet
    Kanchan, // middle of sequence
    Tanki, // pair wait
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Calls {
    chii : bool,
    pon : bool,
    kan : bool,
    /// TODO: Might need to replace Pair with Ron
    pair : bool,
}

impl Default for Calls {
    fn default() -> Self {
        Calls { chii: false, pon: false, kan: false, pair : false }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum WinningMethod {
    NotWonYet,
    Ron,
    Tsumo
}

const INVALID_TILE : Tile = Tile { suit : Suit::Man, value : SuitVal::East, red : true };
pub const PLAYER_HAND_SIZE : usize = 14;
const STARTING_POINTS : i32 = 25000;

#[derive(Clone, Eq, PartialEq)]
pub struct Player {
    pub hand : Vec<Tile>, 
    pub revealed_sets : Vec<Set>,

    /// Only the visible discards from this player
    /// Must be used with tiles_others_called in order to calculate furiten
    pub discard_pile : Vec<Tile>,

    tiles_others_called : Vec<Tile>,

    waiting_on_tiles : Vec<Tile>,
    callable_tiles : HashMap<Tile, Calls>,


    last_picked_tile : Tile,
    pub seat_wind : SuitVal,
    
    points : i32,

    tenpai : bool,

    is_human : bool,

    riichi : bool,
    double_riichi : bool,
    iipatsu : bool,

    winning_wait : Option<WaitType>,
    ron_or_tsumo : (WinningMethod, usize), // usize contains index to player that was ron'd
}

struct PlayerTileIter <'a>{
    player : &'a Player,
    pos : usize,
}

impl <'a> PlayerTileIter<'a> {
    fn construct(player : &'a Player) -> Self
    {
        return PlayerTileIter {
            player : player,
            pos : 0
        }
    }

}

impl <'a> Iterator for PlayerTileIter<'a> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item>
    {
        let total_player_tiles_len = self.player.hand.len() + self.player.revealed_sets_tiles_len();
        
        if self.pos < self.player.hand.len()
        {
            self.pos += 1;
            return Some(self.player.hand[self.pos - 1]);
        }
        else if self.pos < total_player_tiles_len
        {
            self.pos += 1;
            
            let mut revealed_pos : i32 = (self.pos - 1 - self.player.hand.len()) as i32;

            for i in 0..self.player.revealed_sets.len(){
                let set_len = match self.player.revealed_sets[i].set_type{
                    SetType::Pair => 2,
                    SetType::Sequence | SetType::Triplet => 3,
                    SetType::ClosedKan | SetType::OpenKan => 4,
                };

                revealed_pos -= set_len;
                if revealed_pos < 0
                {
                    return Some(self.player.revealed_sets[i].tiles[(revealed_pos + set_len) as usize]);
                }
                else if revealed_pos == 0
                {
                    return Some(self.player.revealed_sets[i].tiles[0]);
                }
            }
            
            return None;
        }
        else {
            return None;
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        return Player { 
            hand : vec![INVALID_TILE; PLAYER_HAND_SIZE],
            last_picked_tile : INVALID_TILE,
            revealed_sets : Vec::new(),

            discard_pile : Vec::with_capacity(70),
            tiles_others_called : Vec::with_capacity(20),

            waiting_on_tiles : Vec::new(),
            callable_tiles : HashMap::new(),

            seat_wind : SuitVal::East,
            points : STARTING_POINTS,

            tenpai : false,

            riichi : false,
            double_riichi : false,
            iipatsu : false,

            is_human : false,

            winning_wait : None,
            ron_or_tsumo : (WinningMethod::NotWonYet, 42),
        };
    }
}

impl Player {
    fn revealed_sets_tiles_len(&self) -> usize 
    {
        let mut len = 0;

        for i in 0..self.revealed_sets.len()
        {
            match self.revealed_sets[i].set_type
            {
                SetType::Pair => len += 2,
                SetType::Sequence | SetType::Triplet => len += 3,
                SetType::OpenKan | SetType::ClosedKan => len += 4
            }
        }

        return len;
    }




    // discard based
    // ---------------
    // chankan
    // houtei raoyui
    
    
    // discretionary
    // ---------------
    
    // draw based
    // ----------------
    // haitei raoyue
    // menzenchin tsumohou
    // rinshan kaihou
    
    
    // honor based
    // --------------------
    // honitsu
    // honroutou
    // shousangen
    // tanyao
    // yakuhai
    
    
    // riichi dependent
    // --------------------
    // ippatsu
    
    
    // Sequential
    // --------------------
    // Iipeikou
    // Ittsuu / Ikkitsuukan
    // Pinfu
    // Ryanpeikou
    // Sanshoku
    // sanshoku doujun
    
    
    // Terminal Based
    // ---------------------
    // chantaiyao
    // honroutou
    // junchan / Junchantaiyaochuu
    // nagashi mangan
    // tanyao
    
    
    // triplet based
    // -----------------------
    // sanankou
    // sankantsu
    // sanshoku doukou
    // shousangen
    // toitoi
    // yakuhai
    
    
    // Suit based
    // -------------------------
    // honiisou
    // Chiniisou
    
    
    // Yakuman
    // -------------------------
    
    
    // Optional/Local yaku
    // --------------------------
    // renhou
    // daisharin
    
    
    // Other
    // -------------------------


    fn has_yaku(&self) -> bool
    {
        let (num_pairs, num_sequences, num_triplets_or_quads) = self.num_pairs_sequences_and_triplets_or_quads();

        if num_pairs == 7
        {   return true;    }
        else if num_triplets_or_quads == 4
        {   return true;    }
        else if self.riichi == true || self.double_riichi == true
        {   return true;    }



        false
    }

    // fn has_winning_hand(&self) -> bool
    // {
    //     return self.has_complete_hand() && self.has_yaku();
    // }

    fn rotate_wind(&mut self)
    {
        self.seat_wind = match self.seat_wind{
            SuitVal::East => SuitVal::South,
            SuitVal::South => SuitVal::West,
            SuitVal::West => SuitVal::North,
            SuitVal::North => SuitVal::East,
            _ => panic!("lkdsfj")
        };
    }

    // commented out due to fighting the borrow checker. Had to run through Game with an index into the player
//    fn choose_discard(&mut self, game : &mut Game) -> Tile
 //   {
  // }

    fn score_hand(&self) -> i32
    {
        0
    }

    fn set_is_human(&mut self) -> &mut Player
    {
        self.is_human = true;
        self
    }

    fn set_seat_wind(&mut self, seat_wind : SuitVal) -> &mut Player
    {
        match seat_wind {
            SuitVal::South => self.seat_wind = SuitVal::South,
            SuitVal::West => self.seat_wind = SuitVal::West,
            SuitVal::North => self.seat_wind = SuitVal::North,
            _ => self.seat_wind = SuitVal::East
        }

        return self;
    }

    // greedy. If there's 4 of a tile, there's only 1 triplet reported
    pub fn hand_num_triplets(&self) -> usize
    {
        let mut idx : usize = 0;
        let mut num_triplets : usize = 0;

        while idx < self.hand.len() - 2
        {
            let curr : Tile = self.hand[idx];
            let next: Tile = self.hand[idx + 1];
            let nextnext : Tile = self.hand[idx + 2];

            if curr.suit == next.suit && curr.value == next.value
            {
                if next.suit == nextnext.suit && next.value == nextnext.value
                {
                    num_triplets += 1;

                    idx += 3;
                    continue;
                }
                else
                {
                    idx += 2;
                    continue;
                }
            }
            else
            {
                idx += 1;
                continue;
            }
        }

        return num_triplets;
    }

    // greedy. If there's 3 of a tile, there's only 1 pair reported
    pub fn hand_num_pairs(&self) -> u8
    {
        let mut idx : usize = 0;
        let mut num_pairs = 0;

        while idx < self.hand.len() - 1
        {
            if self.hand[idx] == self.hand[idx + 1]
            {
                num_pairs += 1;
                idx += 2;
            }
            else
            {
                idx += 1;
            }
        }

        return num_pairs;
    }

    pub fn tiles_num_of(&self, suit : Suit, value : SuitVal) -> usize
    {
        let mut num_occurrences = 0;

        let mut iter = PlayerTileIter::construct(self);
        let mut tile = iter.next();

        while tile != None
        {
            if tile.unwrap().suit == suit && tile.unwrap().value == value
            {
                num_occurrences += 1;
            }

            tile = iter.next();
        }

        return num_occurrences;
   }

    pub fn tiles_contain(&self, suit : Suit, value : SuitVal) -> bool
    {
        let mut iter = PlayerTileIter::construct(self);
        let mut tile = iter.next();

        while tile != None
        {
            if tile.unwrap().suit == suit && tile.unwrap().value == value
            {   return true;    }

            tile = iter.next();
        }

        return false;
   }

    // excludes kazoe (yakuman from enough han)
    fn hand_yakuman_in_basic_points(&self, game : &Game) -> usize
    {
        let mut basic_points : usize = 0;

        for yakuman in YAKUMAN_FUNCS
        { // 8000 basic points per yakuman. Some functions can return double yakuman (a value of 2)
            basic_points += 8000 * yakuman(self, game);
        }

        return basic_points;
    }

    fn hand_yaku_in_han(&self) -> usize
    {
        0
    }

    fn hand_dora_in_han(&self) -> usize
    {
        0
    }

    fn hand_fu(&self, game : &Game, round_up : bool) -> usize
    {
        // chiitoitsu (seven pairs) is always 25 fu
        if self.yaku_chiitoitsu(game) != 0
        {
            return 25;
        }

        let mut fu = 20;
        
        // add fu for revealed sets (which include closed kans)
        for set in &self.revealed_sets {
            let mut added_fu =  match set.set_type {
                                        SetType::ClosedKan => 16,        
                                        SetType::OpenKan => 8,
                                        SetType::Triplet => 2,
                                        _ => 0,
                                    };

            if set.has_honor_or_terminal()
            {
                added_fu *= 2;
            }

            fu += added_fu;
        }

        // add fu for closed triplets
        let mut i = 0;
        while i < self.hand.len() - 2
        {
            let curr : Tile = self.hand[i];
            let next : Tile = self.hand[i + 1];
            let nextnext : Tile = self.hand[i + 2];

            if curr.suit == next.suit && curr.value == next.value
            {
                if next.suit == nextnext.suit && next.value == nextnext.value
                {
                    let mut added_fu = 4;

                    if next.suit == Suit::Honor || next.value == SuitVal::One || next.value == SuitVal::Nine
                    {
                        added_fu *= 2;
                    }

                    fu += added_fu;
                    i += 3;
                    continue;
                }
                else
                {
                    i += 2;
                    continue;
                }
            }
            else
            {
                i += 1;
                continue;
            }
        }

        // add fu for a pair of honor tiles which would count as yaku
        let winning_pair = self.get_one_pair();
        if winning_pair == None
        {          
            // In riichi mahjong there is ALWAYS a winning pair. Return 0 fu since the hand isn't a winning hand
            return 0;
        }
        else if winning_pair.unwrap().tiles[0].suit == Suit::Honor
        {
            fu +=   match winning_pair.unwrap().tiles[0].value {
                        SuitVal::Red => 2,
                        SuitVal::White => 2,
                        SuitVal::Green => 2,
                        _ => 0,
                    };

            if winning_pair.unwrap().tiles[0].value == game.round_wind
            {
                fu += 2;
            }

            if winning_pair.unwrap().tiles[0].value == self.seat_wind
            {
                fu += 2;
            }
        }

        // add fu for winning wait
        fu +=   match self.winning_wait {
                    Some(WaitType::Ryanmen) => 0,
                    Some(WaitType::Kanchan) => 2,
                    Some(WaitType::Penchan) => 2,
                    Some(WaitType::Tanki)   => 2,
                    Some(WaitType::Shanpon) => 0,
                    None => panic!("Attempted to score fu for non winning player"),
                };

        // add fu for ron or tsumo
        fu +=   match self.ron_or_tsumo {
                    (WinningMethod::NotWonYet, _) => 0,
                    (WinningMethod::Ron, _) => { 
                        if self.hand_is_closed()
                         { 10 }
                        else
                         { 0 }
                    },
                    (WinningMethod::Tsumo, _) => {
                        if self.yaku_pinfu(game) != 0
                         { 0 }
                        else
                         { 2 }
                    },
                };


        // for pinfu
        if fu == 20
        {
            fu = 30;
        }

        // round up to 10
        if round_up{
            let last_dig = fu % 10;
            if last_dig != 0
            {
                fu += 10 - last_dig;
            }
        }

        return fu;
    }

    // way more complex than it should be imo. Way to go Japanese!
    fn score_hand_basic_points(&self, game : &Game) -> usize
    {
        // double yakuman, come on!
        let yakuman_pts = self.hand_yakuman_in_basic_points(game);
        if yakuman_pts > 0
        {
            return yakuman_pts;
        }


        let mut han = self.hand_yaku_in_han();
        // at least one yaku or yakuman is required to have a valid scoring hand
        if han == 0 {
            return 0;
        }

        han += self.hand_dora_in_han();

        // don't score fu if 5 han or above
        if han >= 5
        {
            if han == 5
            {   return 2000;    }
            else if han <= 7
            {   return 3000;    }
            else if han <= 10
            {   return 4000;    }
            else if han <= 12
            {   return 6000;    }
            else // hand counted as yakuman if there's enough han
            {   return 8000;   }

        }
        else
        {
            let mut basic_points = self.hand_fu(game, true) * pow(2, 2 + han); 

            // if han and fu reach over 2000 points, it's considered a 2000 point mangan
            if basic_points > 2000
            {   return 2000;            }
            else
            {   return basic_points;    }
        }

    }

    pub fn set_hand(&mut self, hand : Vec<Tile>) ->  &mut Player
    {
        self.hand = hand;
        return self;
    }

    pub fn sort_hand(&mut self) -> () {
        self.hand.sort();
    }

    fn print_player(&self) -> ()
    {
        print!("{} Player:", self.seat_wind);
        print_tiles(&self.hand, self.hand.len());
    }

    pub fn has_dragon_or_wind_yakuhai(&self, round_wind : SuitVal) -> bool
    {
        // goes until index 2, because a triplet is required. So we don't bother
        // checking the last two

        let mut i : usize = self.hand.len() - 1;
        while i >= 2
        {
            let curr : Tile = self.hand[i];
            let next : Tile = self.hand[i - 1];
            let nextnext : Tile = self.hand[i - 2];

            if curr.suit != Suit::Honor
            {
                return false;
            }
            else
            {
                if curr.value != next.value
                {
                    i -= 1;
                }
                else if curr.value != nextnext.value
                {
                    i -= 2;
                }
                else
                {
                    if curr.value == self.seat_wind
                    {   return true;    }
                    else if curr.value == round_wind
                    {   return true;    }
                    else if curr.value == SuitVal::Red
                    {   return true;    }
                    else if curr.value == SuitVal::White
                    {   return true;    }
                    else if curr.value == SuitVal::Green
                    {   return true;    }
                    else
                    {   // non yakuhai triplet found
                        i -= 3;
                    }
            }
        }
    }


        return true;
    }

    fn tile_in_hand_triplet(&self, tile : Tile) -> bool
    {
        return self.hand.iter().filter(|hand_tile| **hand_tile == tile).count() == 3;
    }

    fn tile_in_hand_sequence(&self, tile : Tile) -> bool
    {
            if tile.suit == Suit::Honor
            {   return false;   }

            // return true if there's the next two in the sequence
            return self.hand.iter().find(|hand_tile| tile.value.int_value() + 1 == hand_tile.value.int_value() && tile.suit == hand_tile.suit).is_some()
            && self.hand.iter().find(|hand_tile| tile.value.int_value() + 2 == hand_tile.value.int_value() && tile.suit == hand_tile.suit).is_some();
    }


    /// updates the callable_tiles vec on player to enable chii's and pon's
    /// ONLY UPDATES WITH PAIR WAITS IF PLAYER TENPAI IS SET TO TRUE
    fn update_callable_tiles(&mut self) -> ()
    {   // TODO: ??? Maybe make it so that we don't have to recalculate every single callable tile each time
        self.callable_tiles.clear();

        // update with upgrading open triplets to open kans
        for set in &self.revealed_sets
        {
            match set.set_type {
                SetType::Triplet => self.callable_tiles.entry(set.tiles[0]).or_default().kan = true,
                _ => ()
            }
        }

        // update with calls to triplets or kans in hand
        for tile in &self.hand
        {
            let num_in_hand = self.hand.iter().filter(|hand_tile| **hand_tile == *tile).count();

            if num_in_hand == 2
            {
                self.callable_tiles.entry(*tile).or_default().pon = true;
            }
            else if num_in_hand == 3
            {
                let mut entry = self.callable_tiles.entry(*tile).or_default();
                entry.pon = true;
                entry.kan = true;
            }
        }

        // the commented code should do the same thing as the lines below. No idea why it doesn't work sometimes though
        let mut last_numbered_tile_idx = self.hand.iter().rposition(
                |find_tile| find_tile.suit != Suit::Honor
            );


        // if there's more than one numbered tile, look through them for sequences
        if let Some(last_numbered_tile_idx) = last_numbered_tile_idx
        {
            if last_numbered_tile_idx != 0
            {
                // ignoring the first and numbered tile, because if it's part of a sequence then we'll find from the tiles next to them
                for i in 1..(last_numbered_tile_idx + 1)
                {
                    let curr_tile = self.hand[i];

                    // only checking for two tile sequences behind our current tile, because we check until the last tile
                    let prev_tile_in_sequence = self.hand.iter().find(
                        |find_tile| find_tile.value.is_before_num(curr_tile.value) && find_tile.suit == curr_tile.suit
                    );

                    // add tiles to complete the sequence from 2 tiles to 3 if there's a tile in this hand behind the current one
                    if let Some(prev_tile_in_sequence) = prev_tile_in_sequence
                    {
                        
                        if let Some(third_tile_behind) = prev_tile_in_sequence.get_prev_num_tile()
                        {
                            self.callable_tiles.entry(third_tile_behind).or_default().chii = true;
                        }

                        if let Some(third_tile_ahead) = curr_tile.get_next_num_tile()
                        {
                            self.callable_tiles.entry(third_tile_ahead).or_default().chii = true;
                        }
                    }
                }
            }
        }


        // if in tenpai, look for possible pairs
        if self.tenpai
        {
            // add lonely tiles
            for tile in &self.hand
            {
                if self.tiles_num_of(tile.suit, tile.value) == 1 && ! self.callable_tiles.contains_key(tile)
                {
                    self.callable_tiles.entry(*tile).or_default().pair = true;
                }
            }
        }
    }


    // a "complete hand" still needs a yakuhai to be considered a winning hand
    // and not all winning hands are "complete" hands.
    pub fn check_complete_hand_and_update_waits(&self) -> bool 
    {
        let (tile_vec, mut hands_vec) = self.get_hands_with_pairs();

        let mut vec_vec_of_sets : Vec<Vec<Set>> = vec![];

        for hand in &mut hands_vec
        {
            for inner_hand in Player::find_triplets_from_pair_hands(hand)
            {
                vec_vec_of_sets.push(inner_hand);
            }
        }

        let mut max : usize = 0;
        for vec_set in &vec_vec_of_sets
        {
            let local_max = vec_set.len();
            if local_max > max
            {
                max = local_max;
            }
        }

        // remove the sets which 
        vec_vec_of_sets.retain(|vec_set| vec_set.len() == max);

        println!("Len of vec_vec_of_sets is {}", vec_vec_of_sets.len());
        for vec in vec_vec_of_sets{
            print!("..next has a triplet num of {}..", vec.len());
        }
        print!("\n");

//        let (pairs, sequences, triplets_and_quads) = self.num_pairs_and_triplets();

        false
    }
}


// TODO: TESTCASE: m2,m3,m4,p3,p4,p5,p8,s4,s4,s4,s6,s8,s8,s8 - should have four triplets, but no pairs

const NUM_GAME_TILES : usize = 136;
pub const NUM_PLAYERS    : usize = 4;

#[allow(dead_code)]
pub struct Game {
    tiles : [Tile; NUM_GAME_TILES],
    pub next_tile : usize,

    dora_idx : usize,
    ura_dora_idx : usize,

    pub num_called_tiles : usize,

    curr_player_idx : usize,
    players : [Player; NUM_PLAYERS],

    round_wind : SuitVal,
}

// impl Default for Game {
//     fn Default() -> Game {
//         Game { tiles: 0, players: 0 }
//     }
// }
impl Default for Game
{
    fn default() -> Self {
        // let mut new_game = Game { 
            // tiles : [ 
                    //    Tile { suit : Suit::Man, value : SuitVal::One, red : false};
                    //    NUM_GAME_TILES
                    // ],
            // players : [1,2,3,4]
        // };

        let mut new_game = Game {
            round_wind : SuitVal::East,

            num_called_tiles : 0,

            dora_idx : NUM_GAME_TILES - 14,
            ura_dora_idx : NUM_GAME_TILES - 7,

            curr_player_idx : usize::MAX,

            tiles : [
                Tile { suit : Suit::Man, value : SuitVal::One, red : false},
                Tile { suit : Suit::Man, value : SuitVal::One, red : false},
                Tile { suit : Suit::Man, value : SuitVal::One, red : false},
                Tile { suit : Suit::Man, value : SuitVal::One, red : false},

                Tile { suit : Suit::Man, value : SuitVal::Two, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false},

                Tile { suit : Suit::Man, value : SuitVal::Three, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false},
        
                Tile { suit : Suit::Man, value : SuitVal::Four, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Four, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Four, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Four, red : false},

                Tile { suit : Suit::Man, value : SuitVal::Five, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Five, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Five, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Five, red : true},

                Tile { suit : Suit::Man, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false},

                Tile { suit : Suit::Man, value : SuitVal::Seven, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Seven, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Seven, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Seven, red : false},

                Tile { suit : Suit::Man, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Eight, red : false},

                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false},

                Tile { suit : Suit::Pin, value : SuitVal::One, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::One, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::One, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::One, red : false},

                Tile { suit : Suit::Pin, value : SuitVal::Two, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Two, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Two, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Two, red : false},

                Tile { suit : Suit::Pin, value : SuitVal::Three, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Three, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Three, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Three, red : false},
        
                Tile { suit : Suit::Pin, value : SuitVal::Four, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Four, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Four, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Four, red : false},

                Tile { suit : Suit::Pin, value : SuitVal::Five, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Five, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Five, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Five, red : true},

                Tile { suit : Suit::Pin, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Six, red : false},

                Tile { suit : Suit::Pin, value : SuitVal::Seven, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Seven, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Seven, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Seven, red : false},

                Tile { suit : Suit::Pin, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Eight, red : false},

                Tile { suit : Suit::Pin, value : SuitVal::Nine, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Nine, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Nine, red : false},
                Tile { suit : Suit::Pin, value : SuitVal::Nine, red : false},

                Tile { suit : Suit::Sou, value : SuitVal::One, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::One, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::One, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::One, red : false},

                Tile { suit : Suit::Sou, value : SuitVal::Two, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Two, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Two, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Two, red : false},

                Tile { suit : Suit::Sou, value : SuitVal::Three, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Three, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Three, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Three, red : false},
        
                Tile { suit : Suit::Sou, value : SuitVal::Four, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Four, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Four, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Four, red : false},

                Tile { suit : Suit::Sou, value : SuitVal::Five, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Five, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Five, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Five, red : true},

                Tile { suit : Suit::Sou, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Six, red : false},

                Tile { suit : Suit::Sou, value : SuitVal::Seven, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Seven, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Seven, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Seven, red : false},

                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false},

                Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false},

                Tile { suit : Suit::Honor, value : SuitVal::North, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::North, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::North, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::North, red : false },

                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },

                Tile { suit : Suit::Honor, value : SuitVal::South, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::South, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::South, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::South, red : false },

                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },

                Tile { suit : Suit::Honor, value : SuitVal::Red, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::Red, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::Red, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::Red, red : false },

                Tile { suit : Suit::Honor, value : SuitVal::White, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::White, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::White, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::White, red : false },

                Tile { suit : Suit::Honor, value : SuitVal::Green, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::Green, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::Green, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::Green, red : false },
            ],

            next_tile : 0,

            players : [
                Player::default().set_is_human().to_owned(),        
                Player::default().set_seat_wind(SuitVal::South).to_owned(),
                Player::default().set_seat_wind(SuitVal::West).to_owned(),
                Player::default().set_seat_wind(SuitVal::North).to_owned(),
            ],
        };


        return new_game;
}


}

fn round_up_to_100(points : i32) -> i32
{   // no rounding needed
    if points % 100 == 0
    {   return points;   }

    let last_two_digits = points % 100;

    return (points + (100 - last_two_digits)) as i32;
}

fn mahjong_tiles_strs(tile_vec : & Vec<Tile>, line_width : usize)-> Vec<String>
{
    // returns a vector of three line strings. Each tuple is a top, middle, and bottom of 3 char high tiles.
    // Vector because multiple lines can be returned to fit within a specified line width

    const TILE_TOP : &str = "┌──┐";
    const TILE_MID_LEFT : char = '│';
    const TILE_MID_RIGHT : char = '│';
    const TILE_BOT : &str = "└──┘";

    let tile_len : usize = TILE_TOP.graphemes(true).count();

    // always has at least 3 for the top middle and bottom strings
    let mut ret_vec = vec![];
    for i in 0..((((tile_vec.len() * tile_len) / line_width) + 1) * 3)
    {
        ret_vec.push(
            String::with_capacity(line_width)
        );
    }

    let mut index = 0;
    for tile in tile_vec{
        ret_vec[index].push_str(TILE_TOP);

        if ret_vec[index].graphemes(true).count() >= line_width
        {
            index += 3;
        }
    }

    let mut index = 1;
    for tile in tile_vec {
        let mut char1 = String::from("");
        let mut char2 = String::from("");

        char1.push_str(match tile.suit {
            Suit::Man => "m",
            Suit::Pin => "p",
            Suit::Sou => "s",
            Suit::Honor => "", // don't print suit for honor, just print two chars
        });

        char2.push_str(match tile.value {        
            SuitVal::One => "1",
            SuitVal::Two => "2",
            SuitVal::Three => "3",
            SuitVal::Four => "4",
            SuitVal::Five => "5",
            SuitVal::Six => "6",
            SuitVal::Seven => "7",
            SuitVal::Eight => "8",
            SuitVal::Nine => "9",
            
            SuitVal::East => "Ea",
            SuitVal::South => "So",
            SuitVal::West => "We",
            SuitVal::North => "No",

            SuitVal::Green => "Gr",
            SuitVal::White => "Wh",
            SuitVal::Red => "Re",
        });

        if tile.suit == INVALID_TILE.suit && tile.value == INVALID_TILE.value
        {
            char1 = String::from(" ");
            char2 = String::from(" ");
        }

        if tile.red == true
        {// https://github.com/rust-lang/rust/issues/7043
            // https://github.com/rust-lang/rust/issues/21492
            // https://github.com/rust-lang/rust/issues/8706

//            char1 = String::from("日本");
 //           char2 = String::from("");
           // char1.insert_str(0, &"\u{0305}");
           // char2.insert_str(0, &"\u{0305}");
            char1 = char1.to_uppercase();
        }

        ret_vec[index].push_str(&format!("{}{}{}{}", TILE_MID_LEFT, char1, char2, TILE_MID_RIGHT));

        if ret_vec[index].graphemes(true).count() >= line_width
        {
            index += 3;
        }
    }

    let mut index = 2;
    for tile in tile_vec{
        ret_vec[index].push_str(TILE_BOT);
        if ret_vec[index].graphemes(true).count() >= line_width
        {
            index += 3;
        }
    }


    // remove extra empty vectors
    ret_vec = ret_vec.into_iter().filter(|vec| vec.len() != 0).collect();

    return ret_vec;
}

impl Game {
    fn player_choose_discard_or_win(&mut self, player_idx : usize) -> Option<Tile>
    {
        clearscreen::clear().expect("Error! Could not clear the screen");
        tui_output::output_game_state(self, 0);

        let mut discard_idx : usize;

        // make a choice on winning or which tile to discard
        if self.players[player_idx].is_human
        {

            let player_current_hand = self.players[player_idx].hand.clone();
            let player_can_win : bool = self.players[player_idx].check_complete_hand_and_update_waits();
            self.players[player_idx].hand = player_current_hand; // checking for a complete hand requires it be sorted
                                                                // but we want the newest drawn tile to be shown to the right for discarding purposes

            // TODO: Put extra text for if you can win. Make choosing to not win make you type no-win
            if player_can_win
            {
                println!("You should be able to win 77777777777777777777777777777777777777777777777777777777777777777777777777777777777777");
            }
            println!("Enter which tile you would like to discard (\"n\" standing for \"new\" works for the rightmost drawn tile)");

            let mut input = String::from("");
            std::io::stdin().read_line(&mut input).expect("stdin readline failed");
            input = input.trim().to_lowercase();
            
            discard_idx = loop {

                let input_as_num = input.parse::<usize>();

                if let Ok(input_as_num) = input_as_num
                {
                    // We give the player numbers starting from 1, but indexes start from 0
                    let input_as_num = input_as_num - 1;

                    if input_as_num > self.players[player_idx].hand.len()
                    {
                        println!("Enter a number within the valid range!");
                        continue;
                    }
                    else
                    {
                        break input_as_num;
                    }
                }
                else if input == "n"
                {
                    break self.players[player_idx].hand.len() - 1;
                }
                else
                {
                    println!("Enter a tile to discard!");
                }

                input.clear();
                std::io::stdin().read_line(&mut input).expect("stdin readline failed");
                input = input.trim().to_lowercase();
            };
        }
        // computer picks which to discard
        else
        {
            let mut input = String::from("");
            std::io::stdin().read_line(&mut input).expect("stdin readline failed");
            discard_idx = 0;
        }

        println!("Player number {} discarded tile {}. Deck marker is {}", player_idx, discard_idx, self.next_tile);
        let discarded_tile = self.players[player_idx].hand.remove(discard_idx);
        self.players[player_idx].discard_pile.push(discarded_tile);
    
        Some(discarded_tile)
    }


    fn draw_next_tile(&mut self) -> Option<Tile>
    {
        if self.next_tile >= self.dora_idx
        {
            return None;
        }
        else
        {
            self.next_tile += 1;
            
            return Some(self.tiles[self.next_tile - 1]);
        }
    }

    fn score_points_and_advance_dealer(&mut self, winning_player_idx : Option<usize>) -> ()
    {
        const EXHAUSTIVE_DRAW_POINTS : i32 = 3000;
        match winning_player_idx
        {
            None => {
                let num_tenpai_players = self.players.iter().filter(|player| player.tenpai).count();

                // no one or everyone is in tenpai
                if num_tenpai_players == 0 || num_tenpai_players == 4
                {  return;  }

                for player in &mut self.players {
                    if player.tenpai
                    {
                        player.points += round_up_to_100(EXHAUSTIVE_DRAW_POINTS / (num_tenpai_players as i32));
                    }
                    else
                    {
                        player.points -= round_up_to_100(EXHAUSTIVE_DRAW_POINTS / ((NUM_PLAYERS - num_tenpai_players) as i32));
                    }
                }

                // rotate seats if dealer wasn't in tenpai and someone else was
                if ! self.players.iter().find(|player| player.seat_wind == SuitVal::East).unwrap().tenpai
                {
                    for player in &mut self.players { player.rotate_wind(); }
                }
            }
            Some(winning_player_idx) => {

                let basic_points = self.players[winning_player_idx].score_hand_basic_points(self);

                let winners_seat_wind : SuitVal = self.players[winning_player_idx].seat_wind;

                let other_players = self.players.iter().filter(|player| player.seat_wind != self.players[winning_player_idx].seat_wind);

                match self.players[winning_player_idx].ron_or_tsumo {
                    (WinningMethod::Ron, victim_index) =>  // "victim" is the player who got ron called on them
                    if self.players[winning_player_idx].seat_wind == SuitVal::East
                    {
                        self.players[victim_index].points -= round_up_to_100((basic_points * 6) as i32);
                        self.players[winning_player_idx].points += round_up_to_100((basic_points * 6) as i32);
                    }
                    else
                    {
                        self.players[victim_index].points -= round_up_to_100((basic_points * 4) as i32);
                        self.players[winning_player_idx].points += round_up_to_100((basic_points * 4) as i32);
                    },
                    (WinningMethod::Tsumo, _) => 
                    if self.players[winning_player_idx].seat_wind == SuitVal::East
                    {
                        for player in &mut self.players{
                            if player.seat_wind != winners_seat_wind
                            {
                                player.points -= round_up_to_100((basic_points * 2) as i32);
                            }
                        }
                        
                        self.players[winning_player_idx].points += round_up_to_100((basic_points * 2) as i32) * (NUM_PLAYERS -1) as i32;
                    }
                    else
                    {
                        for player in &mut self.players {
                            if player.seat_wind == SuitVal::East
                            {
                                player.points -= round_up_to_100((basic_points * 2) as i32);
                            }
                            else if player.seat_wind != winners_seat_wind
                            {
                                player.points  -= round_up_to_100(basic_points as i32);
                            
                            }
                            
                        }
                        
                        self.players[winning_player_idx].points += round_up_to_100((basic_points * 2) as i32);
                        self.players[winning_player_idx].points += round_up_to_100((basic_points * 2) as i32) * 2;
                        
                    },
                    (_,_) => panic!("Player won, but did not have ron or tsumo set"),
                }

                // rotate seat positions if needed
                if self.players[winning_player_idx].seat_wind != SuitVal::East
                {
                    for player in &mut self.players
                    {
                        player.rotate_wind();
                    }
                }
            }
        }
    }

    // fisher yates shuffle of the game tiles
    fn shuffle(&mut self) -> ()
    {
        self.next_tile = 0;

        for i in 0..NUM_GAME_TILES-2 {
            let random_idx : usize = rand::thread_rng().gen_range(i..NUM_GAME_TILES);
            
            // exchange tiles from i and random index
            let mut temp : Tile = self.tiles[i];
            self.tiles[i] = self.tiles[random_idx];
            self.tiles[random_idx] = temp;
        }
    }

    fn divy_tiles_to_players(&mut self) -> ()
    {   // players start with 13 tiles and draw their 14th each turn
        for player in &mut self.players{
            player.hand.clear();
        }

        for i in 0..((PLAYER_HAND_SIZE - 1) * NUM_PLAYERS)
        {
            self.players[i % NUM_PLAYERS].hand.push(self.tiles[self.next_tile as usize]);
            self.next_tile += 1;
        }

        for i in 0..NUM_PLAYERS
        {
            self.players[i].sort_hand();
        }
    }

    fn clear_discards(&mut self) -> ()
    {
        for player in &mut self.players{
            player.discard_pile.clear();
            player.tiles_others_called.clear();
        }
    }

    fn play_hand(&mut self) -> ()
    {
        self.shuffle();
        self.divy_tiles_to_players();
        self.clear_discards();
        
        // Dealer is the east wind player
        self.curr_player_idx = self.players.iter()
            .position(|player| player.seat_wind == SuitVal::East)
            .expect("There was no player with East Wind who could be the dealer");

        loop
        {
                //draw the next tile or exhaustive draw
                let next_tile = self.draw_next_tile();
                if next_tile.is_none()
                {  
                    self.score_points_and_advance_dealer(None);
                    break;
                }

                let next_tile = unsafe {next_tile.unwrap_unchecked()};

                // commented out due to fighting the borrow checker
                //  let curr_player = &mut self.players[curr_player_idx];
                self.players[self.curr_player_idx].hand.push(next_tile);
                let discarded_tile = self.player_choose_discard_or_win(self.curr_player_idx);
                self.players[self.curr_player_idx].sort_hand();

                // A player always discards, unless they chose to win
                if discarded_tile.is_none()
                {
                    self.score_points_and_advance_dealer(Some(self.curr_player_idx));
                    break;
                }

              //TODO: Allow other players to chii, pon, and ron from here 
                self.curr_player_idx = (self.curr_player_idx + 1) % NUM_PLAYERS;
            
        };
    }

    fn play_round(&mut self) -> ()
    {
        const HANDS_PER_ROUND : u8 = 4;
        
        for i in 0..HANDS_PER_ROUND
        {

            self.play_hand();

            // change player seat winds, and which player is dealer
            // player seat winds change clockwise while round winds change counter clockwise (Weird)
            for player in self.players.iter_mut()
            {
                player.seat_wind = match player.seat_wind
                {
                    SuitVal::East => SuitVal::North,
                    SuitVal::North => SuitVal::West,
                    SuitVal::West => SuitVal::South,
                    SuitVal::South => SuitVal::East,
                    _ => panic!("Error: Attempted to advance player's wind from {}", player.seat_wind)
                }
            }

        }

    }


    pub fn play_game(&mut self, num_rounds : u8) -> ()
    {

        self.round_wind = SuitVal::East;

        for i in 0..num_rounds
        {

            self.play_round();
           
            // round winds change counter clockwise while player seat winds change clockwise (Weird)
            self.round_wind = match self.round_wind {
                SuitVal::East => SuitVal::South,
                SuitVal::South => SuitVal::West,
                SuitVal::West => SuitVal::North,
                SuitVal::North => SuitVal::East,
                _ => panic!("Error: Attempted to advance to next round from a round wind value of {}", self.round_wind)
            }
        }


    }
}


// fn fisher_yates_shuffle(tiles : [Tile; NUM_GAME_TILES]) -> ()
// {
//     println!("Hi");
// }
fn print_tiles(tiles : &[Tile], num_to_print : usize) -> ()
{
    for i in 0..(num_to_print-1)
    {
        print!("{},", tiles[i as usize]);
    }

    println!("{}", tiles[num_to_print - 1]);
}




pub fn print_game_state(game : &Game) -> ()
{
    println!("Tiles ({} left to draw)\n--------------------------------", NUM_GAME_TILES - game.next_tile);
    let mut i : usize = game.next_tile as usize;

    // print a row with 4 tiles on it
    while i < NUM_GAME_TILES - 4 {
        print!("{}:", i);
    
        for j in 0..4 {
            print!("{},", game.tiles[i]);
            i += 1;
        }
    
        print!("\n");
    }

    // print the last row with possibly less than 4 items
    if i < NUM_GAME_TILES
    {
        print!("{}:", i);

        while i < NUM_GAME_TILES {
            print!("{},", game.tiles[i]);
            i += 1;
        }
    }

    println!("\nPlayers\n--------------------------------");
    for i in 0..NUM_PLAYERS {
        print!("{} Player {}:{{{}pts}}", game.players[i as usize].seat_wind, i, game.players[i].points);
        print_tiles(&game.players[i as usize].hand, game.players[i as usize].hand.len());
    }

}
//}





enum YakuType {
// closed only and 1 han
    Riichi,
    Ippatsu,
    MenzenchinTsumohou,
    Pinfu,
    Iipeikou,
// 1 han
    HaiteiRaoyue,
    HouteiRaoyui,
    RinshanKaihou,
    Chankan,
    Tanyao,
    Yakuhai,
// 2 han
    DoubleRiichi,
    Chantaiyao,
    SanshokuDoujun,
    Ikkitsuukan,
    Toitoi,
    Sanankou,
    SanshokuDoukou,
    Sankantsu,
    Chiitoitsu,
    Honroutou,
    Shousangen,
// 3 han
    Honitsu,
    Junchantaiyao,
    Ryanpeikou,
// 6 han
    Chinitsu,

// YAKUMAN
    Kazoe,
    KokushiMusou,
    Daisangen,
    Suuankou,
    Shousuushi,
    Daisuushi,
    Tsuuiisou,
    Ryuuiisou,
    Chinroutou,
    ChuurenPoutou,
    Suukantsu,
    Tenhou,
    Chiihou,
    NagashiMangan,
    Renhou,
    Daisharin,
}


impl Player
{
    /// Looks through a hand and returns a tuple containing (pairs found, other tiles in the hand aside from the pair).
    /// If a tile pair has already been ron'd or tsumo'd into the revealed sets, then it returns early with just that pair
    /// since that will be the winning hand
    /// # Returns
    /// 1. `Vec<Tile>` : A vector of tiles which were found as pairs in the hand
    /// 2. `Vec<Vec<Tile>>` : A vector containing hands associated with each of the pairs. If the `Vec<Tile>` from 1 above
    /// from above contained a 4 of man, then the `Vec<Vec<Tile>>` here would contain all other tiles from
    /// the passed in hand with 2 of the 4 man tiles removed 
    fn get_hands_with_pairs(&self) -> (Vec<Tile>, Vec<Vec<Tile>>)
    {
        let mut pairs : Vec<Tile> = vec![];
        let mut hands : Vec<Vec<Tile>> = vec![vec![]];

        // possible early return
        // if a tile pair has been tsumo'd or ron'd then the winning hand contains that pair
        for set in &self.revealed_sets
        {
            match set.set_type {
                SetType::Pair => {
                    pairs.push(set.tiles[0]);
                    hands.push(self.hand.clone());
                    return (pairs, hands);
                },
                _ => ()
            }
        }

        for tile in &self.hand
        {
            if (! pairs.contains(tile)) && self.hand.iter().filter(|vec_tile| **vec_tile == *tile).count() >= 2
            {
                pairs.push(*tile);
                hands.push(self.hand.clone());
                let mut recent_hand : &mut Vec<Tile> = hands.last_mut().unwrap();
                // remove tile twice from the hand we just added, since we already decided it was the pair. We will check for combinations of the other tiles later
                recent_hand.remove(recent_hand.iter().position(|iter_tile| *iter_tile == *tile).expect("Should never happen"));
                recent_hand.remove(recent_hand.iter().position(|iter_tile| *iter_tile == *tile).expect("Should never happen"));
            }
        }

        return (pairs, hands);
    }

    /// takes a sorted vector of tiles and returns possible completed combinations of triplets and sequences within it.
    /// It is assumed that a pair of tiles has been removed from this hand.
    /// Returns a vector of possibilites, because it's possible to have a hand that could be interpreted two ways passed
    /// into this function. For example, all numbers of the same suit 444555666777 could be all triplets or 3 sequences and a triplet.
    /// whichever scores higher is how it should be interpreted
    fn find_triplets_from_pair_hands(hand : &mut Vec<Tile>) -> Vec<Vec<Set>>
    {
        let mut ret_vec : Vec<Vec<Set>> = vec![vec![]];

        fn is_part_of_triplet(hand : &Vec<Tile>, check_tile : Tile) -> bool
        {
            return hand.iter().filter(|hand_tile| **hand_tile == check_tile).count() == 3;
        }

        // only checks up because we go left to right
        fn is_part_of_sequence(hand : &Vec<Tile>, check_tile : Tile) -> bool
        {
            if check_tile.suit == Suit::Honor
            {   return false;   }

            // return true if there's the next two in the sequence
            return hand.iter().find(|hand_tile| check_tile.value.int_value() + 1 == hand_tile.value.int_value() && check_tile.suit == hand_tile.suit).is_some()
            && hand.iter().find(|hand_tile| check_tile.value.int_value() + 2 == hand_tile.value.int_value() && check_tile.suit == hand_tile.suit).is_some();
        }

        // we can only have one of each triplet, so if we've seen a triplet before we don't check a permutation with it again in this iteration
        let mut seen_triplets_vec : Vec<Tile> = vec![];

        for tile in &*hand
        {
            if is_part_of_triplet(hand, *tile) && (! seen_triplets_vec.contains(tile))
            {
                let found_triplet = Set {
                    set_type : SetType::Triplet,
                    tiles : [ *tile ; 4 ],
                    ron : false
                };

                let mut new_hand = hand.clone();
                // remove 3 of the element
                new_hand.remove(new_hand.iter().position(|hand_tile| *hand_tile == *tile).unwrap());
                new_hand.remove(new_hand.iter().position(|hand_tile| *hand_tile == *tile).unwrap());
                new_hand.remove(new_hand.iter().position(|hand_tile| *hand_tile == *tile).unwrap());

                let mut sets_vec_vec = Player::find_triplets_from_pair_hands(&mut new_hand);

                for mut set_vec in sets_vec_vec {
                    set_vec.push(found_triplet);
                    ret_vec.push(set_vec);
                }

                seen_triplets_vec.push(*tile);
            }

            // not an else if, because we need to check for both possibilities
            if is_part_of_sequence(hand, *tile)
            {
                let mut new_hand = hand.clone();

                // remove and collect tiles in the sequence
                let sequence_first = new_hand.remove(new_hand.iter().position(|hand_tile| tile.value.int_value() == hand_tile.value.int_value() && tile.suit == hand_tile.suit).unwrap());
                let sequence_second = new_hand.remove(new_hand.iter().position(|hand_tile| tile.value.int_value() + 1 == hand_tile.value.int_value() && tile.suit == hand_tile.suit).unwrap());
                let sequence_third = new_hand.remove(new_hand.iter().position(|hand_tile| tile.value.int_value() + 2 == hand_tile.value.int_value() && tile.suit == hand_tile.suit).unwrap());

                let found_sequence = Set {
                    set_type : SetType::Sequence,
                    tiles : [sequence_first, sequence_second, sequence_third, INVALID_TILE],
                    ron : false
                };

                let mut sets_vec_vec = Player::find_triplets_from_pair_hands(&mut new_hand);

                for mut set_vec in sets_vec_vec {
                    set_vec.push(found_sequence);
                    ret_vec.push(set_vec);
                }
            }


        }

        return ret_vec;
    }

    fn num_pairs_sequences_and_triplets_or_quads(&self) -> (usize, usize, usize)
    {
        
        return (0, 0, 0);
    }

    fn hand_is_closed(&self) -> bool
    {
        if self.revealed_sets.len() == 0 
        {
            return true;
        }
        else
        {
            for set in &self.revealed_sets
            {
                // hand is still closed if it's only closed kans, or a set was created from ron
                if set.set_type != SetType::ClosedKan && set.ron == false
                {
                    return false;
                }
            }

            return true;
        }
    }

    // returns a pair from the hand or revealed sets. Used to find the pair for winning hands
    // in the case of the yakuman of all pairs, simply returns the first pair it finds
    fn get_one_pair(&self) -> Option<Set>
    {
        let mut ret_set = Set {
            set_type : SetType::Pair,
            tiles : [INVALID_TILE ; 4],
            ron : false
        };

        // look for a pair in revealed sets
        for set in &self.revealed_sets
        {
            if set.set_type == SetType::Pair
            {
                ret_set.tiles = set.tiles;
                ret_set.ron = set.ron;
                return Some(ret_set);
            }
        }

        // look for a pair in player's hand
        let mut i = 0;
        while i < self.hand.len() - 2
        {
            let curr : Tile = self.hand[i];
            let next : Tile = self.hand[i + 1];
            let nextnext : Tile = self.hand[i + 2];

            if curr.suit == next.suit && curr.value == next.value
            {
                if next.suit == nextnext.suit && next.value == nextnext.value
                {
                    i += 3;
                    continue;
                }
                else
                {
                    ret_set.tiles[0] = curr;
                    ret_set.tiles[1] = next;
                    return Some(ret_set);
                }
            }

            i += 1;
            continue;
        }

        let last : Tile = self.hand[self.hand.len() - 1];
        let second_last : Tile = self.hand[self.hand.len() - 2];

        if last.suit == second_last.suit && last.value == second_last.value
        {
            ret_set.tiles[0] = second_last;
            ret_set.tiles[1] = last;
            return Some(ret_set);
        }

        return None;
    }
    
    fn yaku_chiitoitsu(&self, game : &Game) -> usize
    {
        0
    }

    fn yaku_pinfu(&self, game : &Game) -> usize
    {
        0
    }

    // three great dragons
    fn yakuman_daisangen(&self, game : &Game) -> usize
    {
        return
          ( self.tiles_num_of(Suit::Honor, SuitVal::White) == 3
        &&  self.tiles_num_of(Suit::Honor, SuitVal::Red) == 3
        &&  self.tiles_num_of(Suit::Honor, SuitVal::Green) == 3 )
        as usize;
    }

    // thirteen orphans
    fn yakuman_kokushi_musou(&self, game : &Game) -> usize
    { // TODO: Double yakuman if the wait was on the pair
        if  self.tiles_contain(Suit::Man, SuitVal::One)
        &&  self.tiles_contain(Suit::Man, SuitVal::Nine)
        &&  self.tiles_contain(Suit::Pin, SuitVal::One)
        &&  self.tiles_contain(Suit::Pin, SuitVal::Nine)
        &&  self.tiles_contain(Suit::Sou, SuitVal::One)
        &&  self.tiles_contain(Suit::Sou, SuitVal::Nine)
        &&  self.tiles_contain(Suit::Honor, SuitVal::North)
        &&  self.tiles_contain(Suit::Honor, SuitVal::East)
        &&  self.tiles_contain(Suit::Honor, SuitVal::South)
        &&  self.tiles_contain(Suit::Honor, SuitVal::West)
        &&  self.tiles_contain(Suit::Honor, SuitVal::Red)
        &&  self.tiles_contain(Suit::Honor, SuitVal::White)
        &&  self.tiles_contain(Suit::Honor, SuitVal::Green)
        &&  self.hand_num_pairs() == 1 
        {
            // double yakuman if there was a 13 sided wait for the last tile
            if self.tiles_num_of(self.last_picked_tile.suit, self.last_picked_tile.value) == 2
            {   2   }
            else
            {   1   }
        }
        else
        {
            0
        }
    }

    // four concealed triplets and a pair
    fn yakuman_suuankou(&self, game : &Game) -> usize
    {
        if self.revealed_sets.len() <= 1 && self.hand_num_triplets() == 4
        {
            // double yakuman if wait is on the pair
            if self.tiles_num_of(self.last_picked_tile.suit, self.last_picked_tile.value) == 2
            {   return 2;   }
            else
            {   return 1;   }
        }

        return 0;
    }

    // three little winds and four great winds
    fn yakuman_suushiihou(&self, game : &Game) -> usize
    {
        let num_winds =
            [   self.tiles_num_of(Suit::Honor, SuitVal::East),
                self.tiles_num_of(Suit::Honor, SuitVal::South),
                self.tiles_num_of(Suit::Honor, SuitVal::West),
                self.tiles_num_of(Suit::Honor, SuitVal::North)  ];

        let num_wind_sets  = num_winds.into_iter().filter(|&t| t >= 3 ).count();
        let num_wind_pairs = num_winds.into_iter().filter(|&t| t == 2 ).count();

        // Four great winds - Double yakuman
        if num_wind_sets == 4
        {
            return 2;
        } 
        // three little winds
        else if num_wind_sets == 3 && num_wind_pairs == 1
        {
            return 1;
        }
        else
        {
            return 0;
        }
    }

    // all honor tiles
    fn yakuman_tsuuiisou(&self, game : &Game) -> usize
    {
        // since honor tiles are always sorted to the right, we can just check
        // if the leftmost tile is an honor tile to see if they're all honors
        // returns 1 for 1 yakuman if the condition is met, otherwise 0 for no yakuman
        return (self.hand[0].suit == Suit::Honor) as usize;
    }

    // all green tiles
    fn yakuman_ryuuiisou(&self, game : &Game) -> usize
    {
        for i in 0..self.hand.len()
        {
            let cur : Tile = self.hand[i];
            
            // suit check
            if cur.suit != Suit::Sou && cur.suit != Suit::Honor
            {
                return 0;
            }

            // value check
            if cur.value != SuitVal::Two && cur.value != SuitVal::Three 
                && cur.value != SuitVal::Four && cur.value != SuitVal::Six
                && cur.value != SuitVal::Eight && cur.value != SuitVal::Green
            {
                return 0;
            }
        }

        return 1;
    }

    // all terminal tiles
    fn yakuman_chinroutou(&self, game : &Game) -> usize
    {
       for i in 0..self.hand.len()
       {
           let cur : Tile = self.hand[i];

           if cur.value != SuitVal::One && cur.value != SuitVal::Nine
           {
               return 0;
           }
       } 

       return 1;
    }

    // TODO: The opened door? Forget the english translation. Full straight with extra terminals
    fn yakuman_chuuren_poutou(&self, game : &Game) -> usize
    { // TODO: Double yakuman if the last tile chosen was the extra tile

        // check for all tiles being the same suit
        for i in 1..self.hand.len()
        {
            if self.hand[i].suit != self.hand[0].suit
            {
                return 0;
            }
        }

        return 
             ( self.hand.iter().filter(|&t| t.value == SuitVal::One).count() >= 3
            && self.hand.iter().filter(|&t| t.value == SuitVal::Two).count() >= 1
            && self.hand.iter().filter(|&t| t.value == SuitVal::Three).count() >= 1
            && self.hand.iter().filter(|&t| t.value == SuitVal::Four).count() >= 1
            && self.hand.iter().filter(|&t| t.value == SuitVal::Five).count() >= 1
            && self.hand.iter().filter(|&t| t.value == SuitVal::Six).count() >= 1
            && self.hand.iter().filter(|&t| t.value == SuitVal::Seven).count() >= 1
            && self.hand.iter().filter(|&t| t.value == SuitVal::Eight).count() >= 1
            && self.hand.iter().filter(|&t| t.value == SuitVal::Nine).count() >= 3
            ) as usize;
    }


    fn yakuman_suukantsu(&self, game : &Game) -> usize
    {
        // checking for the 4 quads
        if self.revealed_sets.iter().filter(
            |&set| set.set_type == SetType::OpenKan || set.set_type == SetType::ClosedKan).count() != 4
        {
            return 0;
        }

        // checking for the pair
        // since the 4 quads have to be revealed, the pair could be the only tiles in the hand, or also revealed
        if self.hand_num_pairs() == 1
        || self.revealed_sets.iter().filter(
            |&set| set.set_type == SetType::Pair).count() == 1
        {
            return 1;
        }

        return 0;
    }

    // the below two yakuman are for the dealer getting a completed hand, and the dealer
    // drawing a complete hand with his first tile
    // Dealer has completed hand on draw
    fn yakuman_tenhou(&self, game : &Game) -> usize
    {
        if self.seat_wind == SuitVal::East
        && self.revealed_sets.len() == 0
        && game.next_tile == 0
        {
            return 1;
        }

        return 0;
    }

    // dealer completes hand with first draw
    fn yakuman_chiihou(&self, game : &Game) -> usize
    {
        // TODO: If any tile call made by any player, does it interrupt this? Or only if this player has called? For now, assuming any player
        // also assuming that calling a closed kan eliminates possibility of this hand
        if game.next_tile < NUM_PLAYERS
        && game.next_tile != 0
        && game.num_called_tiles == 0
        && self.seat_wind == SuitVal::East
        && self.revealed_sets.len() == 0
        {
            return 1;
        }

        return 0;
    }


}

const YAKUMAN_FUNCS : [ &dyn Fn(&Player, &Game) -> usize ; 11] = [
    &Player::yakuman_daisangen,    
    &Player::yakuman_kokushi_musou,    
    &Player::yakuman_suuankou,
    &Player::yakuman_suushiihou,
    &Player::yakuman_tsuuiisou,    
    &Player::yakuman_ryuuiisou,    
    &Player::yakuman_chinroutou,
    &Player::yakuman_chuuren_poutou,
    &Player::yakuman_suukantsu,
    &Player::yakuman_tenhou,
    &Player::yakuman_chiihou,
 ];












#[test]
fn test_scoring()
{
    let mut game : Game = Game::default();

    game.players[0].ron_or_tsumo = (WinningMethod::Ron, 1);
    game.score_points_and_advance_dealer(Some(0));




}

// tests both yakuman hand detection, and some scoring
#[test]
fn test_kokushi_musou()
{
    let mut game = Game::default();

    game.players[0].hand = vec!(
        Tile { suit : Suit::Honor, value : SuitVal::North, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::South, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::Green, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::Red, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::White, red : false },
        Tile { suit : Suit::Man, value : SuitVal::One, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::One, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::One, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false }, // the duplicate for pair. Set as the last tile drawn, so it should be double yakuman
    );

    game.players[0].sort_hand();

    game.players[0].ron_or_tsumo = (WinningMethod::Tsumo, 0);
    game.players[0].last_picked_tile = Tile { suit : Suit::Honor, value : SuitVal::East, red : false };
    game.next_tile = 1;

    assert_eq!(game.players[0].yakuman_kokushi_musou(&game), 2);
    assert_eq!(game.players[0].yakuman_chiihou(&game), 1);
    assert_eq!(game.players[0].yakuman_daisangen(&game), 0);

    game.score_points_and_advance_dealer(Some(0));

    // Damn, I wish I could have a hand like this sometime
    assert_eq!(game.players[0].points, 169000);
    assert_eq!(game.players.iter().filter(|player| player.points == -23000).count(), 3);
}

#[test]
fn test_daisangen()
{
    let mut game = Game::default();

    game.players[0].hand = vec!(
        Tile { suit : Suit::Honor, value : SuitVal::Red, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::Red, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::Red, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::White, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::White, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::White, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Seven, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Seven, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Seven, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
    );

    game.players[0].revealed_sets = vec!(
        Set {
            set_type : SetType::Triplet,
            tiles : [
                Tile { suit : Suit::Honor, value : SuitVal::Green, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::Green, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::Green, red : false },
                INVALID_TILE
            ],
            ron : true
        }
    );

    game.players[0].sort_hand();

    game.players[0].ron_or_tsumo = (WinningMethod::Ron, 2);
    game.players[0].last_picked_tile = Tile { suit : Suit::Honor, value : SuitVal::Green, red : false };
    game.next_tile = 46;

    assert_eq!(game.players[0].yakuman_kokushi_musou(&game), 0);
    assert_eq!(game.players[0].yakuman_chiihou(&game), 0);
    assert_eq!(game.players[0].yakuman_daisangen(&game), 1);

    game.score_points_and_advance_dealer(Some(0));

    // Damn, I wish I could have a hand like this sometime
    assert_eq!(game.players[0].points, 73000);
    assert_eq!(game.players[1].points, 25000);
    assert_eq!(game.players[2].points, -23000);
    assert_eq!(game.players[3].points, 25000);
}

#[test]
fn test_suuankou()
{
    let mut game = Game::default();

    game.players[0].hand = vec!(
        Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
    );

    game.players[0].revealed_sets = vec!(
        Set {
            set_type : SetType::Pair,
            tiles : [
                Tile {  suit : Suit::Man, value : SuitVal::Seven, red : false},
                Tile {  suit : Suit::Man, value : SuitVal::Seven, red : false},
                INVALID_TILE,
                INVALID_TILE
            ],
            ron : true,
        }
    );

    game.players[0].sort_hand();

    game.players[0].last_picked_tile = Tile { suit : Suit::Man, value : SuitVal::Seven, red : false };
    game.players[0].ron_or_tsumo = (WinningMethod::Ron , 3);
    game.next_tile = 45;


    assert_eq!(game.players[0].yakuman_suuankou(&game), 2);
    
    game.score_points_and_advance_dealer(Some(0));

    assert_eq!(game.players[0].points, 25000 + (16000 * 6));
    assert_eq!(game.players[1].points, 25000);
    assert_eq!(game.players[2].points, 25000);
    assert_eq!(game.players[3].points, 25000 - (16000 * 6));
}

#[test]
fn test_suushiihou()
{
    let game = Game::default();

//    game.players[3].yakuman_suushiihou(&game)
}

#[test]
fn test_fu_1()
{
    let mut game = Game::default();

    game.round_wind = SuitVal::South;

    assert_eq!(game.players[1].seat_wind, SuitVal::South);

    let mut winning_player = &mut game.players[1];

    winning_player.hand = vec!(
        Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Five, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::South, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::South, red : false },
    );

    winning_player.sort_hand();

    winning_player.revealed_sets = vec!(
        Set {
            set_type : SetType::ClosedKan,
            tiles : [
                Tile { suit : Suit::Man, value : SuitVal::One, red : false } ; 4
            ],
            ron : false
        },
        Set {
            set_type : SetType::ClosedKan,
            tiles : [
                Tile { suit : Suit::Honor, value : SuitVal::Red, red : false } ; 4
            ],
            ron : false,
        },
        Set {
            set_type : SetType::Triplet,
            tiles : [
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                INVALID_TILE
          ],
          ron : true
        }
    );

    winning_player.last_picked_tile = Tile { suit : Suit::Honor, value : SuitVal::East, red : false };
    winning_player.winning_wait = Some(WaitType::Shanpon);

    winning_player.ron_or_tsumo = (WinningMethod::Ron, 0);

    // test without rounding to ensure fu is correct
    assert_eq!(game.players[1].hand_fu(&game, false), 102);
    //    assert_eq!(winning_player.hand_yaku_in_han(), 1);
    // TODO: Check for han value in hand
    game.score_points_and_advance_dealer(Some(1));


}



#[test]
fn test_fu_2()
{
    let mut game = Game::default();

    assert_eq!(game.players[0].seat_wind, SuitVal::East);

    let mut winning_player = &mut game.players[0];

    winning_player.hand = vec!(
        Tile { suit : Suit::Sou, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Three, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
    );

    winning_player.sort_hand();

    winning_player.revealed_sets = vec!(
        Set {
            set_type : SetType::ClosedKan,
            tiles : [
                Tile { suit : Suit::Pin, value : SuitVal::One, red : false } ; 4
            ],
            ron : false
        },
        Set {
            set_type : SetType::ClosedKan,
            tiles : [
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false } ; 4
            ],
            ron : false,
        },
        Set {
            set_type : SetType::OpenKan,
            tiles : [
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false } ; 4
            ],
            ron : false
        },
   );

    winning_player.last_picked_tile = Tile { suit : Suit::Honor, value : SuitVal::East, red : false };
    winning_player.winning_wait = Some(WaitType::Tanki);

    winning_player.ron_or_tsumo = (WinningMethod::Tsumo, 0);


    // test without rounding to ensure fu is correct
    assert_eq!(game.players[0].hand_fu(&game, false), 108);
    //    assert_eq!(winning_player.hand_yaku_in_han(), 1);
    // TODO: Check for han value in hand
    game.score_points_and_advance_dealer(Some(0));


}

#[test]
fn test_fu_open_pinfu()
{
    let mut game = Game::default();

    assert_eq!(game.players[0].seat_wind, SuitVal::East);

    let mut winning_player = &mut game.players[0];

    winning_player.hand = vec!(
        Tile { suit : Suit::Man, value : SuitVal::One, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Two, red : false },
   );

    winning_player.revealed_sets = vec!(
        Set {
            set_type : SetType::Sequence,
            tiles : [
                Tile { suit : Suit::Pin, value : SuitVal::Two, red : false },
                Tile { suit : Suit::Pin, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Pin, value : SuitVal::Four, red : false },
                INVALID_TILE
            ],
            ron : false
        },
        Set {
            set_type : SetType::Sequence,
            tiles : [
                Tile { suit : Suit::Sou, value : SuitVal::Five, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Six, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Seven, red : false },
                INVALID_TILE
            ],
            ron : false,
        },
        Set {
            set_type : SetType::Sequence,
            tiles : [
                Tile { suit : Suit::Sou, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Four, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Five, red : false },
                INVALID_TILE
            ],
            ron : true
        },
   );

    winning_player.last_picked_tile = Tile { suit : Suit::Sou, value : SuitVal::Three, red : false };
    winning_player.winning_wait = Some(WaitType::Ryanmen);

    winning_player.ron_or_tsumo = (WinningMethod::Ron, 3);


    // hand gets 0 fu, but hands with 0 fu are rounded up to 30
    assert_eq!(game.players[0].hand_fu(&game, false), 30);
    
    // test that there's no fu points, even with a tsumo
    game.players[0].revealed_sets[2] = Set {
            set_type : SetType::Sequence,
            tiles : [
                Tile { suit : Suit::Sou, value : SuitVal::Four, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Five, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Six, red : false },
                INVALID_TILE
            ],
            ron : false
    };
    game.players[0].last_picked_tile = Tile { suit : Suit::Sou, value : SuitVal::Six, red : false };
    game.players[0].ron_or_tsumo = (WinningMethod::Tsumo, 0);

//    TODO Detect pinfu properly, so as to correctly give no 
//    assert_eq!(game.players[0].hand_fu(&game, false), 30);

    //    assert_eq!(winning_player.hand_yaku_in_han(), 1);
    // TODO: Check for han value in hand
    game.score_points_and_advance_dealer(Some(0));


}










#[test]
fn test_callable_tiles()
{
    {
        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
            ],
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        assert_eq!(player.callable_tiles.len(), 1);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Honor, value : SuitVal::West, red : true} ), true);

        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
            ],
            ..Player::default()
        };
    }
    
    {
        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Five, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
            ],
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        assert_eq!(player.callable_tiles.len(), 9);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Honor, value : SuitVal::West, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Honor, value : SuitVal::East, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::One, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Two, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Three, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Four, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Five, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Six, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Seven, red : true} ), true);
    }

    {
        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Five, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
            ],
            tenpai : true,
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        assert_eq!(player.callable_tiles.len(), 9);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Honor, value : SuitVal::West, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Honor, value : SuitVal::East, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::One, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Two, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Three, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Four, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Five, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Six, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Seven, red : true} ), true);
    }

    {
        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Man, value : SuitVal::One, red : false },
                Tile { suit : Suit::Man, value : SuitVal::One, red : false },
                Tile { suit : Suit::Man, value : SuitVal::One, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Five, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Seven, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Eight, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false },
            ],
            tenpai : true,
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        for (tile, calls) in &player.callable_tiles
        {
            println!("Callable is {}", tile)
        }

        assert_eq!(player.callable_tiles.len(), 9);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Eight, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Nine, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::One, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Two, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Three, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Four, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Five, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Six, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Seven, red : true} ), true);
    }

    // TODO: Test this hand for tenpai detection
    {
        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Seven, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Eight, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::One, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::One, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::One, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
            ],
            tenpai : true,
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        println!("NEWTEST");
        for (tile, calls) in &player.callable_tiles
        {
            println!("Callable is {}", tile)
        }

        assert_eq!(player.callable_tiles.len(), 11);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::One, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Two, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Three, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Four, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Five, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Six, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Seven, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Eight, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Nine, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Sou, value : SuitVal::One, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Sou, value : SuitVal::Nine, red : true} ), true);
    }

    {
        let sou_tile = Tile {
            suit : Suit::Sou,
            value : SuitVal::East,
            red : true
        };

        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Sou, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Five, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Six, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Seven, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false },
            ],
            tenpai : true,
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        println!("NEWTEST");
        for (tile, calls) in &player.callable_tiles
        {
            println!("Callable is {}", tile)
        }

        let pair_call_only = Calls {
            pair : true,
            ..Calls::default()
        };

        let chii_call_only = Calls {
            chii : true,
            ..Calls::default()
        };

        let chii_pon_or_kan_call = Calls {
            chii : true,
            kan : true,
            pon : true,
            pair : false,
        };

        assert_eq!(player.callable_tiles.len(), 7);
        
        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Three, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Three, ..sou_tile }).or_default(), pair_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Four, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Four, ..sou_tile}).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Five, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Five, ..sou_tile }).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Six, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Six, ..sou_tile}).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Seven, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Seven, ..sou_tile}).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Eight, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Eight, ..sou_tile}).or_default(), chii_pon_or_kan_call);
    }
}




