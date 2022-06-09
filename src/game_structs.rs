//pub mod game_structs {
use strum::IntoEnumIterator;
use rand::Rng;
use std::{fmt, slice::Windows, usize::MAX};
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

    pub discard : Vec<Tile>,

    tiles_needed_to_win : Vec<Tile>,

    last_picked_tile : Tile,
    pub seat_wind : SuitVal,
    
    points : i32,

    tenpai : bool,

    riichi : bool,
    double_riichi : bool,
    iipatsu : bool,

    winning_wait : Option<WaitType>,
    ron_or_tsumo : (WinningMethod, usize), // usize contains index to player that was ron'd
}

struct EndlessPlayerIter <'a>{
    players : &'a mut [Player ; NUM_PLAYERS],
}

impl <'a> EndlessPlayerIter{
    fn new(players : &'a [Player ; 4])
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

            discard : Vec::with_capacity(70),

            tiles_needed_to_win : Vec::new(),

            seat_wind : SuitVal::East,
            points : STARTING_POINTS,

            tenpai : false,

            riichi : false,
            double_riichi : false,
            iipatsu : false,

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

    // commented out due to fighting the borrow checker. Had to run through Game with an index into the player
//    fn choose_discard(&mut self, game : &mut Game) -> Tile
 //   {
  // }

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


fn mahjong_tiles_strs(tile_vec : & Vec<Tile>)-> (String, String, String)
{
    // returns three strings with mahjong tiles to be printed. The strings do not end in newlines

    let tile_top = "┌──┐";
    let tile_mid_left = '│';
    let tile_mid_right = '│';
    let tile_bot = "└──┘";
    
    let mut top_str = String::with_capacity(tile_top.len() * tile_vec.len());
    let mut mid_str = String::with_capacity(tile_top.len() * tile_vec.len());
    let mut bot_str = String::with_capacity(tile_top.len() * tile_vec.len());

    for tile in tile_vec{
        top_str.push_str(tile_top);
    }

    for tile in tile_vec {
//        let chars = match tile {
//            INVALID_TILE => "  ",
//        }
        let chars = "  ";
        mid_str.push_str(&format!("{}{}{}", tile_mid_left, chars, tile_mid_right));
    }

    for tile in tile_vec{
        bot_str.push_str(tile_bot);
    }

    return (top_str, mid_str, bot_str);
}

impl Game {
    fn player_choose_discard(&self, player_idx : usize) -> Tile
    {
        self.output_game_state(player_idx);
        INVALID_TILE
    }

    fn output_game_state(&self, player_idx : usize)
    {
        // outputs one "line" of tiles with 3 lines of stdout

        let (top_str, mid_str, bot_str) = mahjong_tiles_strs(&vec![INVALID_TILE ; 14]);
//·
        // print top player
        println!("{: ^100}", top_str);
        println!("{: ^100}", mid_str);
        println!("{: ^100}", bot_str);

        let tile_top = "┌─┐";
        let tile_mid = "│ │";
        let tile_bot = "└─┘";

        // print side players
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_top, " ".repeat(60), tile_top);
        println!("{: >20}{}{: <20}", tile_mid, " ".repeat(60), tile_mid);

//        for tile in &self.players[player_idx].hand {
//            println!("{}", tile);
//        }

        // print current player
        let (top_str, mid_str, bot_str) = mahjong_tiles_strs(&self.players[player_idx].hand);
        println!("{} {} {} {}", self.players[player_idx].hand.len(), player_idx, 6, 9);

        // print current player top tiles with the side player bottom tiles
        println!("{: >20}{: ^60}{: <20}", tile_bot, top_str, tile_bot);

        println!("{: ^100}", mid_str);
        println!("{: ^100}", bot_str);


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
        let original_dealer_idx = self.players.iter()
            .position(|player| player.seat_wind == SuitVal::East)
            .expect("There was no player with East Wind who could be the dealer");

        let mut curr_player_idx = original_dealer_idx;
        let mut players_iter = self.players.iter_mut();
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

                self.players[curr_player_idx].hand.push(next_tile);
                let discarded : Tile = self.player_choose_discard(curr_player_idx);
                self.players[curr_player_idx].sort_hand();

                if self.players[curr_player_idx].has_winning_hand()
                {
                    self.score_points_and_advance_dealer(Some(curr_player_idx));
                    break;
                }

              //TODO: Allow other players to chii, pon, and ron from here 

                curr_player_idx = (curr_player_idx + 1) % NUM_PLAYERS;
            

            let mut curr_player = match curr_player.next() {
                Some(next_player) => next_player,
                None => self.players.iter_mut(),
            }
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

