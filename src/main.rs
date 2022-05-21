#![allow(unused_variables, unused_mut)]
#![allow(unused_imports, dead_code)]


extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::IntoEnumIterator;
use rand::Rng;
use std::fmt;
use int_enum::IntEnum;
use num::pow;


#[allow(dead_code)]
#[derive(EnumIter, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Suit {
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
enum SuitVal {
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
struct Tile {
    suit : Suit,
    value : SuitVal,
    red : bool,
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
enum SetType {
    Pair,
    Sequence,
    Triplet,
    Quad
}

// A completed tile set
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Set {
    set_type : SetType,
    num_tiles : u8,
    tiles : [Tile ; 4],
}


const INVALID_TILE : Tile = Tile { suit : Suit::Man, value : SuitVal::East, red : true };
const PLAYER_HAND_SIZE : usize = 14;
const STARTING_POINTS : i32 = 25000;

#[derive(Clone)]
struct Player {
    hand : [Tile; PLAYER_HAND_SIZE],
    last_picked_tile : Tile,
    revealed_sets : Vec<Set>,

    seat_wind : SuitVal,
    
    points : i32,
}

impl Default for Player {
    fn default() -> Self {
        return Player { 
            hand : { 
                [INVALID_TILE; PLAYER_HAND_SIZE]
            },
            last_picked_tile : INVALID_TILE,
            revealed_sets : Vec::new(),
            
            seat_wind : SuitVal::East,
            points : STARTING_POINTS,
        };
    }
}

impl Player {
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
    fn hand_num_triplets(&self) -> u8
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
    fn hand_num_pairs(&self) -> u8
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

    fn hand_contains_num_of(&self, suit : Suit, value : SuitVal) -> usize
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

    fn hand_contains(&self, suit : Suit, value : SuitVal) -> bool
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

    fn set_hand(&mut self, hand : [Tile ; PLAYER_HAND_SIZE]) ->  &mut Player
    {
        self.hand = hand;
        return self;
    }

    fn sort_hand(&mut self) -> () {
        self.hand.sort();
    }

    fn print_player(&self) -> ()
    {
        print!("{} Player:", self.seat_wind);
        print_tiles(&self.hand, PLAYER_HAND_SIZE);
    }

    fn has_dragon_or_wind_yakuhai(&self, round_wind : SuitVal) -> bool
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
    fn has_complete_hand(&self) -> bool 
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
struct Game {
    tiles : [Tile; NUM_GAME_TILES],
    next_tile : usize,

    players : [Player; NUM_PLAYERS],
}

// impl Default for Game {
//     fn Default() -> Game {
//         Game { tiles: 0, players: 0 }
//     }
// }

fn create_game() -> Game {
    // let mut new_game = Game { 
        // tiles : [ 
                //    Tile { suit : Suit::Man, value : SuitVal::One, red : false};
                //    NUM_GAME_TILES
                // ],
        // players : [1,2,3,4]
    // };

    let mut new_game = Game {
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

    fn draw_tiles(&mut self) -> ()
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




fn print_game_state(game : &Game) -> ()
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



fn main() {
    let mut game = create_game();
    game.shuffle();
    game.draw_tiles();


    print_game_state(&game);
}








#[test]
fn testing ()
{
    let mut player : Player = Player::default().set_hand( 
        [
            Tile { suit : Suit::Man, value : SuitVal::One, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Five, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Seven, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Eight, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Nine, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
            Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
        ]).to_owned();

    player.sort_hand();
    
    assert_eq!(true, player.has_complete_hand());
    assert_eq!(false, player.has_dragon_or_wind_yakuhai(SuitVal::East));

    player.hand = [
        Tile { suit : Suit::Man, value : SuitVal::One, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Five, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Seven, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Eight, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
    ];
    player.sort_hand();

    assert_eq!(false, player.has_dragon_or_wind_yakuhai(SuitVal::East));
    assert_eq!(true, player.has_dragon_or_wind_yakuhai(SuitVal::West));
}