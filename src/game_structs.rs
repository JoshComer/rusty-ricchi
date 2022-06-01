//pub mod game_structs {
use strum::IntoEnumIterator;
use rand::Rng;
use std::{fmt};
use int_enum::IntEnum;
use num::pow;



#[allow(dead_code)]
#[derive(EnumIter, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
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
#[derive(EnumIter, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, IntEnum)]
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
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tile {
    pub suit : Suit,
    pub value : SuitVal,
    pub red : bool,
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
    Quad
}

// A completed tile set
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Set {
    pub set_type : SetType,
    tiles : [Tile ; 4],
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

    tiles_needed_to_win : Vec<Tile>,

    last_picked_tile : Tile,
    pub seat_wind : SuitVal,
    
    points : i32,

    tenpai : bool,

    riichi : bool,
    double_riichi : bool,
    iipatsu : bool,

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
        else if self.pos < total_player_tiles_len as usize
        {
            self.pos += 1;
            
            let mut revealed_pos : i32 = (self.pos - self.player.hand.len()) as i32;

            for i in 0..self.player.revealed_sets.len(){
                revealed_pos -= match self.player.revealed_sets[i].set_type{
                    SetType::Pair => 2,
                    SetType::Sequence | SetType::Triplet => 3,
                    SetType::Quad => 4,
                };
                
                if revealed_pos < 0
                {
                    return Some(self.player.revealed_sets[i].tiles[(-revealed_pos) as usize]);
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

            tiles_needed_to_win : Vec::new(),

            seat_wind : SuitVal::East,
            points : STARTING_POINTS,

            tenpai : false,

            riichi : false,
            double_riichi : false,
            iipatsu : false,

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
                SetType::Quad => len += 4
            }
        }

        return len;
    }

    fn has_winning_hand(&self) -> bool
    {
        return false;
    }

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

    fn choose_discard(&mut self) -> Tile
    {
        INVALID_TILE
    }

    fn score_hand(&self) -> i32
    {
        0
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

        while idx < PLAYER_HAND_SIZE - 1
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

        for i in 0..self.hand.len()
        {
            if self.hand[i].suit == suit && self.hand[i].value == value
            {
                num_occurrences += 1;
            }
        }

        for i in 0..self.revealed_sets.len()
        {
            let mut set_len;

            match self.revealed_sets[i].set_type {
                SetType::Pair => {
                    set_len = 2;
                }
                SetType::Quad => {
                    set_len = 4;
                }
                SetType::Triplet | SetType::Sequence => {
                    set_len = 3;
                }
            }


            for j in 0..set_len
            {
                if self.revealed_sets[i].tiles[j].suit == suit && self.revealed_sets[i].tiles[j].value == value
                {
                    num_occurrences += 1;
                }
            }
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

        for i in 0..self.hand.len()
        {
            if self.hand[i].suit == suit && self.hand[i].value == value
            {
                return true;
            }
        }

        for i in 0..self.revealed_sets.len()
        {
            let mut set_len;

            match self.revealed_sets[i].set_type {
                SetType::Pair => {
                    set_len = 2;
                }
                SetType::Quad => {
                    set_len = 4;
                }
                SetType::Triplet | SetType::Sequence => {
                    set_len = 3;
                }
            }

            for j in 0..set_len {
                if self.revealed_sets[i].tiles[j].suit == suit && self.revealed_sets[i].tiles[j].value == value
                {
                    return true;
                }
            }

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

    fn hand_fu(&self) -> usize
    {
        // chiitoitsu (seven pairs) is always 25 fu
        if self.hand_num_pairs() == 7
        {
            return 25;
        }



        0
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
            let mut basic_points = self.hand_fu() * pow(2, 2 + han); 

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
        print_tiles(&self.hand, PLAYER_HAND_SIZE);
    }

    pub fn has_dragon_or_wind_yakuhai(&self, round_wind : SuitVal) -> bool
    {
        // goes until index 2, because a triplet is required. So we don't bother
        // checking the last two

        let mut i : usize = PLAYER_HAND_SIZE - 1;
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

    // a "complete hand" still needs a yakuhai to be considered a winning hand
    // and not all winning hands are "complete" hands.
    pub fn has_complete_hand(&self) -> bool 
    {
        // suits can only interact with themselves, 
        // so we test for sequences, triplets, and pairs in each suit

        // a "set" is either a sequence or a triplet
        let mut num_sets : i8 = 0;
        let mut num_pairs : i8 = 0;

        let mut curr_tile_idx : usize = 0;

        // Since only having one tile of a suit doesn't allow for a set or pair, we only test for the next tile
        while curr_tile_idx + 1 < PLAYER_HAND_SIZE 
           && self.hand[curr_tile_idx + 1].suit == Suit::Man
        {
            let curr_tile : Tile = self.hand[curr_tile_idx];
            let next_tile : Tile = self.hand[curr_tile_idx + 1];
            
            // pair or triplet
            if next_tile.value == curr_tile.value
            {
                if curr_tile_idx + 2 < PLAYER_HAND_SIZE 
                    && self.hand[curr_tile_idx + 2].value == curr_tile.value
                {
                    num_sets += 1;
                    curr_tile_idx = curr_tile_idx + 3;
                }
                else
                {
                    num_pairs += 1;
                    curr_tile_idx = curr_tile_idx + 2;
                }

            }
            // sequence
            else if next_tile.value.int_value() == curr_tile.value.int_value() + 1
            {
                if curr_tile_idx + 2 < PLAYER_HAND_SIZE 
                    && self.hand[curr_tile_idx + 2].value.int_value() == next_tile.value.int_value() + 1
                {
                    num_sets += 1;
                    curr_tile_idx = curr_tile_idx + 3;
                }
                else
                {
                    num_pairs += 1;
                    curr_tile_idx = curr_tile_idx + 2;
                }               
            }
            else
            {
                curr_tile_idx += 1;
            }

        }

        // while next_tile.suit == Suit::Pin
        // {

        // }

        // while next_tile.suit == Suit::Sou
        // {

        // }

        // // honor tiles can only come in triplets
        // while next_tile.suit == Suit::Honor
        // {

        // }

        return num_pairs == 1 && num_sets == 4;
    }
}


const NUM_GAME_TILES : usize = 136;
pub const NUM_PLAYERS    : usize = 4;

#[allow(dead_code)]
pub struct Game {
    tiles : [Tile; NUM_GAME_TILES],
    pub next_tile : usize,

    dora_idx : usize,
    ura_dora_idx : usize,

    pub num_called_tiles : usize,

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
                Player::default(),        
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


impl Game {
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
    {
        for i in 0..(PLAYER_HAND_SIZE * NUM_PLAYERS)
        {
            self.players[i % NUM_PLAYERS].hand[i / 4] = self.tiles[self.next_tile as usize];
            self.next_tile += 1;
        }

        for i in 0..NUM_PLAYERS
        {
            self.players[i].sort_hand();
        }
    }


    fn play_hand(&mut self) -> ()
    {
        self.shuffle();
        self.divy_tiles_to_players();
       
        // Dealer is the east wind player
        let original_dealer = self.players.iter().position(|player| player.seat_wind == SuitVal::East);

        if original_dealer.is_none()
        {
            panic!("There was no player with East Wind who could be the dealer");
        }

        let original_dealer_idx = unsafe { original_dealer.unwrap_unchecked() };

        let mut curr_player_idx = original_dealer_idx;
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

            let curr_player = &mut self.players[curr_player_idx];

            curr_player.hand.push(next_tile);
            let discarded : Tile = curr_player.choose_discard();
            curr_player.sort_hand();

            if curr_player.has_winning_hand()
            {
                self.score_points_and_advance_dealer(Some(curr_player_idx));
                break;
            }
            
          //TODO: Allow other players to chii, pon, and ron from here 

            curr_player_idx = (curr_player_idx + 1) % NUM_PLAYERS;
        }
    }


    fn play_round(&mut self) -> ()
    {
        const HANDS_PER_ROUND : u8 = 4;
        
        for i in 0..HANDS_PER_ROUND
        {

            self.play_hand();

            // change player seat winds, and which player is dealer
            for player in self.players.iter_mut()
            {
                player.seat_wind = match player.seat_wind
                {
                    SuitVal::East => SuitVal::South,
                    SuitVal::South => SuitVal::West,
                    SuitVal::West => SuitVal::North,
                    SuitVal::North => SuitVal::East,
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
        print_tiles(&game.players[i as usize].hand, PLAYER_HAND_SIZE);
    }

}
//}






impl Player
{
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
        for i in 1..PLAYER_HAND_SIZE
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
            |&set| set.set_type == SetType::Quad).count() != 4
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
            ]
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
            ]
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