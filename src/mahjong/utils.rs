use crate::mahjong::*;

pub fn round_up_to_100(points : i32) -> i32
{   // no rounding needed
    if points % 100 == 0
    {   return points;   }

    let last_two_digits = points % 100;

    return (points + (100 - last_two_digits)) as i32;
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

pub fn gen_random_suit(honors_available : bool) -> Suit
    {
        let suit_range = if honors_available { 0..4 } else { 0..3 };

        match rand::thread_rng().gen_range(suit_range) {
            0 => Suit::Man,
            1 => Suit::Pin,
            2 => Suit::Sou,
            3 => Suit::Honor,
            _ => panic!()
        }
    }

pub fn get_random_pair_triplet_or_kan(num_tiles : usize) -> Set
    {
        let set_type = gen_random_suit(true);
        let mut set_value;

        if set_type == Suit::Honor
        {
            set_value = match rand::thread_rng().gen_range(0..7) {
                0 => SuitVal::East, 1 => SuitVal::West, 2 => SuitVal::South, 3 => SuitVal::North,
                4 => SuitVal::Red, 5 => SuitVal::Green, 6 => SuitVal::White, _ => panic!()
            };
        }
        else {
            set_value = match rand::thread_rng().gen_range(1..10) {
                1 => SuitVal::One, 2 => SuitVal::Two, 3 => SuitVal::Three, 4 => SuitVal::Four,
                5 => SuitVal::Five, 6 => SuitVal::Six, 7 => SuitVal::Seven, 8 => SuitVal::Eight, 9 => SuitVal::Nine,
                _ => panic!()
            };
        }

        Set {
            set_type : if num_tiles == 2 { SetType::Pair } else if num_tiles == 3 { SetType::Triplet } else { SetType::Kan },
            tiles: vec![ Tile { suit : set_type, value : set_value, red : false } ; num_tiles]
        }
    }

    pub fn get_random_sequence() -> Set
    {
        let set_type = gen_random_suit(false);

        let number = match rand::thread_rng().gen_range(1..8) {
            1 => SuitVal::One, 2 => SuitVal::Two, 3 => SuitVal::Three, 4 => SuitVal::Four,
            5 => SuitVal::Five, 6 => SuitVal::Six, 7 => SuitVal::Seven, _ => panic!()
        };

        let first_tile = Tile {
            suit : set_type,
            value : number,
            red : false,
        };

        let second_tile = Tile {
            ..first_tile.get_next_num_tile().unwrap()
        };

        let third_tile = Tile {
            ..second_tile.get_next_num_tile().unwrap()
        };

        return Set { set_type: SetType::Sequence , tiles: vec![first_tile, second_tile, third_tile] };
    }

