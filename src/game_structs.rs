//pub mod game_structs {
use strum::IntoEnumIterator;
use rand::Rng;
use std::fmt;
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
    num_tiles : u8,
    tiles : [Tile ; 4],
}

#[derive(Clone)]
pub enum WinningMethod {
    NotWonYet,
    Ron,
    Tsumo
}

const INVALID_TILE : Tile = Tile { suit : Suit::Man, value : SuitVal::East, red : true };
pub const PLAYER_HAND_SIZE : usize = 14;
const STARTING_POINTS : i32 = 25000;

#[derive(Clone)]
pub struct Player {
    pub hand : Vec<Tile>, 
    pub revealed_sets : Vec<Set>,

    last_picked_tile : Tile,
    seat_wind : SuitVal,
    
    points : i32,

    tenpai : bool,

    riichi : bool,
    double_riichi : bool,
    iipatsu : bool,

    ron_or_tsumo : WinningMethod,
}

impl Default for Player {
    fn default() -> Self {
        return Player { 
            hand : vec![INVALID_TILE; PLAYER_HAND_SIZE],
            last_picked_tile : INVALID_TILE,
            revealed_sets : Vec::new(),
            
            seat_wind : SuitVal::East,
            points : STARTING_POINTS,

            tenpai : false,

            riichi : false,
            double_riichi : false,
            iipatsu : false,

            ron_or_tsumo : WinningMethod::NotWonYet,
        };
    }
}

impl Player {
    fn choose_action(&mut self)
    {

    }

    fn has_winning_hand(&mut self)
    {

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
    pub fn hand_num_triplets(&self) -> u8
    {
        let mut idx : usize = 0;
        let mut num_triplets : usize = 0;

        while idx < PLAYER_HAND_SIZE
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

        return 0;
    }

    // greedy. If there's 3 of a tile, there's only 1 pair reported
    pub fn hand_num_pairs(&self) -> u8
    {
        let mut idx : usize = 0;
        let mut num_pairs = 0;

        while idx < PLAYER_HAND_SIZE
        {
            if self.hand[idx] == self.hand[idx + 1]
            {
                num_pairs += 1;
            }

            idx += 2;
        }

        return num_pairs;
    }

    pub fn hand_contains_num_of(&self, suit : Suit, value : SuitVal) -> usize
    {
        let mut num_occurrences = 0;

        for i in 0..PLAYER_HAND_SIZE
        {
            if self.hand[i].suit == suit && self.hand[i].value == value
            {
                num_occurrences += 1;
            }
        }

        return num_occurrences;
    }

    pub fn hand_contains(&self, suit : Suit, value : SuitVal) -> bool
    {
        for i in 0..PLAYER_HAND_SIZE
        {
            if self.hand[i].suit == suit && self.hand[i].value == value
            {
                return true;
            }
        }

        return false;
    }

    // excludes kazoe (yakuman from enough han)
    fn hand_yakuman_in_basic_points(&self) -> usize
    {
        0
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
    fn score_hand_basic_points(&self) -> usize
    {
        // double yakuman, come on!
        let yakuman_pts = self.hand_yakuman_in_basic_points();
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
const NUM_PLAYERS    : usize = 4;

#[allow(dead_code)]
pub struct Game {
    tiles : [Tile; NUM_GAME_TILES],
    next_tile : usize,

    players : [Player; NUM_PLAYERS],

    round_wind : SuitVal,
}

// impl Default for Game {
//     fn Default() -> Game {
//         Game { tiles: 0, players: 0 }
//     }
// }

pub fn create_game() -> Game {
    // let mut new_game = Game { 
        // tiles : [ 
                //    Tile { suit : Suit::Man, value : SuitVal::One, red : false};
                //    NUM_GAME_TILES
                // ],
        // players : [1,2,3,4]
    // };

    let mut new_game = Game {
        round_wind : SuitVal::East,
        
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


impl Game {
    fn score_points(&mut self, winning_player : Option<&Player>) -> ()
    {
        const EXHAUSTIVE_DRAW_POINTS : i32 = 3000;
        match winning_player
        {
            None => {
                let num_tenpai_players = self.players.iter().filter(|player| player.tenpai).count();

                for player in &mut self.players {
                    if player.tenpai
                    {
                        player.points += EXHAUSTIVE_DRAW_POINTS / (num_tenpai_players as i32);
                    }
                    else
                    {
                        player.points -= EXHAUSTIVE_DRAW_POINTS / (num_tenpai_players as i32);
                    }
                }
            }
            Some(winning_player) => {

            }
        }
    }

    // fisher yates shuffle of the game tiles
    fn shuffle(&mut self) -> ()
    {
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
        let dealer = self.players.iter().cycle().find(|player| player.seat_wind == SuitVal::East);
        //.cycle().find(|player| player.seat_wind == SuitVal::East);

        if dealer.is_none()
        {
            panic!("There was no player with East Wind who could be the dealer");
        }

        let dealer = unsafe { dealer.unwrap_unchecked() };

        let mut current_player = &mut dealer.clone();
        loop
        {
            // draw the next tile or exhaustive draw
            if self.next_tile == 0
            {  
                // TODO: Exhaustive draw 
                break;
            }

            let next_tile : Tile = self.tiles[self.next_tile];

            current_player.hand.push(next_tile);
            // sort the hand to make calculating whether win condition has been met eay
            // TODO: Keep track of tiles needed to win. That way we just immediately win if winning tile is drawn
            current_player.sort_hand();

            if current_player.has_complete_hand()
            {
                let mut score = current_player.score_hand();
            }

            current_player.choose_action();



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
    println!("Tiles\n--------------------------------");
    let mut i : usize = game.next_tile;
    while i < NUM_GAME_TILES - 1 {
        print!("{}:", i);
    
        for j in 0..4 {
            print!("{},", game.tiles[i]);
            i += 1;
        }
    
        print!("\n");
    }
    println!("{}:{}", NUM_GAME_TILES-1, game.tiles[NUM_GAME_TILES-1]);


    println!("\nPlayers\n--------------------------------");
    for i in 0..NUM_PLAYERS {
        print!("{} Player {}:", game.players[i as usize].seat_wind, i);
        print_tiles(&game.players[i as usize].hand, PLAYER_HAND_SIZE);
    }

}
//}