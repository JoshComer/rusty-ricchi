use crate::mahjong::*;




const DEBUG_OUTPUT : bool = false;




// screen width
// margin and tile width take up sides of screen width
// screen_mid_width is in between these margins and tile widths
// consts for screen printing

/// Total output screen width
const SCREEN_WIDTH : usize = 150;
/// Total output screen height
const SCREEN_HEIGHT : usize = 45;

/// number of lines used to output the top player's score and hand tiles
const TOP_PLAYER_HAND_DISPLAY_LINES : usize = 4;
/// number of lines used to output the current player's score and hand tiles. An extra line for putting numbers next to the tiles for discard time
const CURR_PLAYER_HAND_DISPLAY_LINES : usize = 5;

const TILE_TOP : &str = "┌─┐";
const TILE_MID : &str = "│ │";
const TILE_BOT : &str = "└─┘";
/// the length of either TILE_TOP, TILE_MID, or TILE_BOT
const TILE_LEN : usize = 3;

/// Number of lines in between the top player and bottom player hands.
/// This includes spacing between top and bottom player hands, discard piles, and the hands of players to the
/// left and right of the current player
const MIDDLE_HEIGHT : usize = 26;
/// Margins to the left and right of the TOP_PLAYER_HAND_DISPLAY_LINES, CURR_PLAYER_HAND_DISPLAY_LINES, and MIDDLE_HEIGHT output zones
const MARGIN : usize = 22;
const MARGIN_AND_TILE_WIDTH : usize = MARGIN + TILE_LEN;

const SCREEN_MID_WIDTH : usize = SCREEN_WIDTH - (MARGIN_AND_TILE_WIDTH * 2);
const SCREEN_MID_WIDTH_THIRD : usize = SCREEN_MID_WIDTH / 3;
const SCREEN_MID_WIDTH_LEFT : usize = SCREEN_MID_WIDTH / 2;

const TILES_IN_DISCARD_ROW : usize = 7;

const TEXT_OUTPUT_LINES : usize = ((SCREEN_HEIGHT - TOP_PLAYER_HAND_DISPLAY_LINES) - MIDDLE_HEIGHT) - CURR_PLAYER_HAND_DISPLAY_LINES;









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







/// Outputs the game from the perspective of a player passed in with player_idx
pub fn output_player_perspective(game : &Game, player_idx : usize) -> ()
{

        if ! DEBUG_OUTPUT
        {
            clearscreen::clear().expect("Error! Could not clear the screen");
        }
        // outputs one "line" of tiles with 3 lines of stdout

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



pub fn get_player_discard_idx(game : &Game, player_idx : usize) -> usize
{
            output_player_perspective(game, player_idx);


            //// TODO: Put extra text for if you can win. Make choosing to not win make you type no-win
            //if player_can_win
            //{
            //    println!("You should be able to win 77777777777777777777777777777777777777777777777777777777777777777777777777777777777777");
            //}
            println!("Enter which tile you would like to discard (\"n\" standing for \"new\" works for the rightmost drawn tile)");

            let mut input = String::from("");
            std::io::stdin().read_line(&mut input).expect("stdin readline failed");
            input = input.trim().to_lowercase();
            
            return loop {

                let input_as_num = input.parse::<usize>();

                if let Ok(input_as_num) = input_as_num
                {
                    // We give the player numbers starting from 1, but indexes start from 0
                    let input_as_num = input_as_num - 1;

                    if input_as_num > game.players[player_idx].hand.len()
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
                    break game.players[player_idx].hand.len() - 1;
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


pub fn get_player_call_choice(game : &Game, player_idx : usize, discarded_tile : Tile, all_possible_calls : &Vec<Set>)
{
    if all_possible_calls.len() == 0
    {
        panic!();
    }
    output_player_perspective(game, player_idx);
    // we need to calculate how many tile spaces to show to the player so they can choose which set to call on
    // we will insert an empty tile in between every set for spacing
    let mut num_tiles_to_display : usize = 0;

    for set in all_possible_calls
    {
        num_tiles_to_display += match set.set_type {
            SetType::ClosedKan | SetType::OpenKan => 4,
            SetType::Sequence | SetType::Triplet => 3,
            SetType::Pair => 2,
        };

        num_tiles_to_display += 1; // empty space between sets
    }
    num_tiles_to_display -= 1; // removing the last unneeded empty tile space. We already know that all_possible_calls has a length of at least 1

    // we need lines to display the tiles, a line for numbers to differentiate sets to pick, a line for input prompt, and a line for input
    const LINES_AVAILABLE_FOR_TILES : usize = SCREEN_HEIGHT - 3;

    if (num_tiles_to_display * TILE_LEN) > LINES_AVAILABLE_FOR_TILES
    {
        panic!();
    }

    println!("You have just been given the option to call");
    println!("BROOOOOOOOOOOO");
    println!("BROOOOOOOOOOOO");
    println!("BROOOOOOOOOOOO");
    println!("BROOOOOOOOOOOO");

    let mut input = String::from("");
    let input = std::io::stdin().read_line(&mut input).expect("bruhhhhhhhh input failed");
}




