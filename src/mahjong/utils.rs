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
