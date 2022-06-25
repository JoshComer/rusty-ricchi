#![allow(unused_variables, unused_mut)]
#![allow(unused_imports, dead_code)]


// external modules
extern crate strum;
#[macro_use]
extern crate strum_macros;


// local modules
pub mod mahjong;
use mahjong::player::*;
use mahjong::tile::*;
use mahjong::*;

use utils::print_game_state;







fn main(){
    let mut game = Game::default();
    game.human_is_playing = true;
    game.play_game(2);


    print_game_state(&game);
}








#[test]
fn testing ()
{
    let mut player : Player = Player::default().set_hand( 
        vec![ 
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
    
    assert_eq!(false, player.has_dragon_or_wind_yakuhai(SuitVal::East));

    player.hand = vec![
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