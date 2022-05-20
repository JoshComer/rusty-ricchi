#![allow(unused_variables, unused_mut)]
#![allow(unused_imports, dead_code)]


extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::IntoEnumIterator;
use rand::Rng;
use std::fmt;

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

#[allow(dead_code)]
#[derive(EnumIter, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum SuitVal {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    North,
    East,
    South,
    West,

    Red,
    White,
    Green
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

const INVALID_TILE : Tile = Tile { suit : Suit::Man, value : SuitVal::East, red : true };
const PLAYER_HAND_SIZE : usize = 13;
const STARTING_POINTS : i32 = 25000;

struct Player {
    hand : [Tile; PLAYER_HAND_SIZE],
    seat_wind : SuitVal,
    
    points : i32,
}

impl Player {
    fn sort_hand(&mut self) -> () {
        self.hand.sort();
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
            Player { 
                hand : [INVALID_TILE; PLAYER_HAND_SIZE],
                seat_wind : SuitVal::East,
                points : STARTING_POINTS,
            },
            Player { 
                hand : [INVALID_TILE; PLAYER_HAND_SIZE],
                seat_wind : SuitVal::South,
                points : STARTING_POINTS,
            },
            Player { 
                hand : [INVALID_TILE; PLAYER_HAND_SIZE],
                seat_wind : SuitVal::West,
                points : STARTING_POINTS,
            },
            Player { 
                hand : [INVALID_TILE; PLAYER_HAND_SIZE],
                seat_wind : SuitVal::North,
                points : STARTING_POINTS,
            },
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
