//pub mod game_structs;
//use game_structs::*;
use crate::mahjong::*;

pub fn output_game_state(game : &Game, player_idx : usize) -> ()
{

        // outputs one "line" of tiles with 3 lines of stdout

        // screen width
        // margin and tile width take up sides of screen width
        // screen_mid_width is in between these margins and tile widths
        // consts for screen printing
        const SCREEN_WIDTH : usize = 150;
        const MIDDLE_HEIGHT : usize = 26;
        // unused currently        const SCREEN_HEIGHT : usize = 130;
        const MARGIN : usize = 22;
        const TILE_TOP : &str = "┌─┐";
        const TILE_MID : &str = "│ │";
        const TILE_BOT : &str = "└─┘";
        const TILE_LEN : usize = 3;
        const MARGIN_AND_TILE_WIDTH : usize = MARGIN + TILE_LEN;
        const SCREEN_MID_WIDTH : usize = SCREEN_WIDTH - (MARGIN_AND_TILE_WIDTH * 2);
        const SCREEN_MID_WIDTH_THIRD : usize = SCREEN_MID_WIDTH / 3;
        const SCREEN_MID_WIDTH_LEFT : usize = SCREEN_MID_WIDTH / 2;

        const TILES_IN_DISCARD_ROW : usize = 7;

        // player vars for printing info
        let curr_player : &Player = &game.players[player_idx];
        let right_player : &Player = &game.players[(player_idx + 1) % NUM_PLAYERS];
        let opposite_player : &Player = &game.players[(player_idx + 2) % NUM_PLAYERS];
        let left_player : &Player = &game.players[(player_idx + 3) % NUM_PLAYERS];

        let curr_discard_strs = mahjong_tiles_strs(&curr_player.discard_pile, TILES_IN_DISCARD_ROW * 4);
        let opposite_discard_strs = mahjong_tiles_strs(&opposite_player.discard_pile, TILES_IN_DISCARD_ROW * 4);
        let left_discard_strs = mahjong_tiles_strs(&left_player.discard_pile, TILES_IN_DISCARD_ROW * 4);
        let right_discard_strs = mahjong_tiles_strs(&right_player.discard_pile, TILES_IN_DISCARD_ROW * 4);


        let opposite_hand = &mahjong_tiles_strs(&vec![INVALID_TILE ; opposite_player.hand.len()], 1000);
        
        // print top player
        println!("{: ^SCREEN_WIDTH$}", format!("pts:{} wind:{}", opposite_player.points, opposite_player.seat_wind) );
        println!("{: ^SCREEN_WIDTH$}", opposite_hand[0]);
        println!("{: ^SCREEN_WIDTH$}", opposite_hand[1]);
        println!("{: ^SCREEN_WIDTH$}", opposite_hand[2]);


        let empty_string = String::from("");

        let left_wind_str = format!("pts:{} wind:{}",left_player.points ,left_player.seat_wind);
        let mut left_margin_iter = std::iter::repeat(&empty_string).take(MIDDLE_HEIGHT / 2).chain(
            std::iter::once(&left_wind_str).chain(
                std::iter::repeat(&empty_string).take((MIDDLE_HEIGHT / 2) + 1)
            )
        );

        let mut left_hand_num_tile_chars = 2 + left_player.hand.len(); // 2 added, because the bottom tile has 2 chars, but every other tile is just represented by the top char
        let mut left_hand_iter = std::iter::repeat(empty_string.as_ref()).take((MIDDLE_HEIGHT - left_hand_num_tile_chars) / 2).chain(
            std::iter::repeat(TILE_TOP).take(left_player.hand.len()).chain(
                std::iter::once(TILE_MID).chain(
                    std::iter::once(TILE_BOT).chain(
//                        std::iter::once(&empty_string)
                        std::iter::repeat(empty_string.as_ref()).take((((MIDDLE_HEIGHT) - left_hand_num_tile_chars) / 2) + 1)
                    ) 
                )
            )    
        );

        let mut right_hand_num_tile_chars = 2 + right_player.hand.len(); // 2 added, because the bottom tile has 2 chars, but every other tile is just represented by the top char
        let mut right_hand_iter = std::iter::repeat(empty_string.as_ref()).take((MIDDLE_HEIGHT - right_hand_num_tile_chars) / 2).chain(
            std::iter::repeat(TILE_TOP).take(right_player.hand.len()).chain(
                std::iter::once(TILE_MID).chain(
                    std::iter::once(TILE_BOT).chain(
//                        std::iter::once(&empty_string)
                        std::iter::repeat(empty_string.as_ref()).take((((MIDDLE_HEIGHT) - right_hand_num_tile_chars) / 2) + 1)
                    ) 
                )
            )    
        );


        let mut middle_discard_iter = std::iter::once(&empty_string).chain(
            opposite_discard_strs.iter().chain(
            std::iter::repeat(&empty_string).take( MIDDLE_HEIGHT - opposite_discard_strs.len() - curr_discard_strs.len() - 2).chain(
                curr_discard_strs.iter()
            ).chain(std::iter::once(&empty_string))
        ));

        let mut left_discard_iter = std::iter::repeat(&empty_string).take((MIDDLE_HEIGHT - left_discard_strs.len()) / 2).chain(
            left_discard_strs.iter().chain(
                std::iter::repeat(&empty_string).take(((MIDDLE_HEIGHT - left_discard_strs.len()) / 2) + 1)
            )
        );

        let mut right_discard_iter = std::iter::repeat(&empty_string).take((MIDDLE_HEIGHT - right_discard_strs.len()) / 2).chain(
            right_discard_strs.iter().chain(
                std::iter::repeat(&empty_string).take(((MIDDLE_HEIGHT - right_discard_strs.len()) / 2) + 1)
            )
        );

        let right_wind_str = format!("pts:{} wind:{}", right_player.points, right_player.seat_wind);
        let mut right_margin_iter = std::iter::repeat(&empty_string).take(MIDDLE_HEIGHT / 2).chain(
            std::iter::once(&right_wind_str).chain(
                std::iter::repeat(&empty_string).take((MIDDLE_HEIGHT / 2) + 1)
            )
        );


        //        let mut left_discard_iter = left_discard_strs.iter();
//        let mut right_discard_iter = right_discard_strs.iter();
        for i in 0..MIDDLE_HEIGHT
        {
            println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", 
                format!("{: ^MARGIN$}{: >TILE_LEN$}", left_margin_iter.next().unwrap_or(&empty_string), left_hand_iter.next().unwrap_or(&empty_string)), 
                format!("{: ^SCREEN_MID_WIDTH_THIRD$}{: ^SCREEN_MID_WIDTH_THIRD$}{: ^SCREEN_MID_WIDTH_THIRD$}", left_discard_iter.next().unwrap_or(&empty_string), middle_discard_iter.next().unwrap_or(&empty_string), right_discard_iter.next().unwrap_or(&empty_string)),
                format!("{: <TILE_LEN$}{: ^MARGIN$}", right_hand_iter.next().unwrap_or(&empty_string), right_margin_iter.next().unwrap_or(&empty_string))
            );
        }



        // print current player
        let current_hand = &mahjong_tiles_strs(&curr_player.hand, 1000);

        println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", current_hand[0], " ");
        println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", current_hand[1], " ");
        println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", current_hand[2], " ");
        
        if player_idx == game.curr_player_idx
        {
            let mut numbers = String::from("");
            for i in 1..(curr_player.hand.len() + 1) {    numbers.push_str(&format!(" {: <2} ", i));    }
            println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", numbers, " ");
        }
        
        println!("{: ^SCREEN_WIDTH$}", format!("pts:{} wind:{}", curr_player.points, curr_player.seat_wind) );

        // println!("{}", " ".repeat(SCREEN_WIDTH));
        // println!("{}", " ".repeat(SCREEN_WIDTH));
        // println!("{}", " ".repeat(SCREEN_WIDTH));
        println!("{}", " ".repeat(SCREEN_WIDTH));
        println!("{}", " ".repeat(SCREEN_WIDTH));
        println!("{}", " ".repeat(SCREEN_WIDTH));
        println!("{}", " ".repeat(SCREEN_WIDTH));
        println!("{}", " ".repeat(SCREEN_WIDTH));
        
}