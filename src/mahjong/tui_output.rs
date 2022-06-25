

use std::io::Read;

use crate::mahjong::*;








// screen width
// margin and tile width take up sides of screen width
// screen_mid_width is in between these margins and tile widths
// consts for screen printing

/// Total output screen width
const SCREEN_WIDTH : usize = 150;
/// Total output screen height
const SCREEN_HEIGHT : usize = 45;

/// number of lines used to output the top player's score and hand tiles with a line for the active player indicator
const TOP_PLAYER_HAND_DISPLAY_LINES : usize = 5;
/// number of lines used to output the current player's score, hand tiles, and active indicator. An extra line for putting numbers next to the tiles for discard time
const CURR_PLAYER_HAND_DISPLAY_LINES : usize = 6;

const TILE_TOP : &str = "┌─┐";
const TILE_MID : &str = "│ │";
const TILE_BOT : &str = "└─┘";
/// the length of either TILE_TOP, TILE_MID, or TILE_BOT
const TILE_SIDE_VIEW_LEN : usize = 3;
const TILE_FRONT_VIEW_LEN : usize = 4;
const TILE_HEIGHT : usize = 3;

/// Number of lines in between the top player and bottom player hands.
/// This includes spacing between top and bottom player hands, discard piles, and the hands of players to the
/// left and right of the current player
const MIDDLE_HEIGHT : usize = 26;
/// Margins to the left and right of the TOP_PLAYER_HAND_DISPLAY_LINES, CURR_PLAYER_HAND_DISPLAY_LINES, and MIDDLE_HEIGHT output zones
const MARGIN : usize = 22;
const MARGIN_AND_TILE_WIDTH : usize = MARGIN + TILE_SIDE_VIEW_LEN;

const SCREEN_MID_WIDTH : usize = SCREEN_WIDTH - (MARGIN_AND_TILE_WIDTH * 2);
const SCREEN_MID_WIDTH_THIRD : usize = SCREEN_MID_WIDTH / 3;
const SCREEN_MID_WIDTH_LEFT : usize = SCREEN_MID_WIDTH / 2;

const SCREEN_MID_WIDTH_THIRD_MINUS_ONE : usize = SCREEN_MID_WIDTH_THIRD - 1;
const SCREEN_MID_WIDTH_MINUS_FURITEN_LEN_TWICE : usize = SCREEN_MID_WIDTH - 14;
const FURITEN_LEN : usize = 7;

const TILES_IN_DISCARD_ROW : usize = 7;

const TEXT_OUTPUT_LINES : usize = ((SCREEN_HEIGHT - TOP_PLAYER_HAND_DISPLAY_LINES) - MIDDLE_HEIGHT) - CURR_PLAYER_HAND_DISPLAY_LINES;

const ACTIVE_PLAYER_MARKER : &str = "ϕ";





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



pub fn output_game(game : &Game, player_idx : usize) -> ()
{
    if let OutputView::BoardView = crate::mahjong::OUTPUT_METHOD
    {
        output_player_perspective(game, player_idx);
    }
    else
    {
        output_row_view(game, player_idx);
    }
}

pub fn output_row_view(game : &Game, player_idx : usize) -> ()
{
        if ! DEBUG_OUTPUT
        {
            clearscreen::clear().expect("Error! Could not clear the screen");
        }

        // print game header
        println!("    Round Wind: {}", game.round_wind);
        println!("-------------------------");
        println!();


        for i in 0..NUM_PLAYERS
        {
            let loop_player = &game.players[(player_idx + i) % NUM_PLAYERS];

            // only reveal other player hands for debugging purposes
            let hand = if ! DEBUG_OUTPUT && ! loop_player.is_human {
                    mahjong_tiles_strs(&vec![INVALID_TILE ; loop_player.hand.len()], 1000)
                }
                else {
                    mahjong_tiles_strs(&loop_player.hand, 1000)
                };

            
            let discard_pile = mahjong_tiles_strs(&loop_player.discard_pile, 1000);

            println!("{}  Pts:{} Wind:{}   -- Tenpai:{}    {}",
            if *loop_player == game.players[game.curr_player_idx] { ACTIVE_PLAYER_MARKER } else { " " },
            loop_player.points, loop_player.seat_wind, if DEBUG_OUTPUT || loop_player.is_human { loop_player.tenpai.to_string() } else { String::from("N/A") },
            if loop_player.furiten { "FURITEN" } else { " " });

            let empty_string = String::from("");
            if loop_player.is_human
            {
                let mut numbers = String::from("");
                for i in 1..(loop_player.hand.len() + 1) {    numbers.push_str(&format!(" {: <2} ", i));    }
                println!("         {}", numbers);
            }

            println!("         {}", hand.get(0).unwrap_or(&empty_string));
            println!("Hand:    {}", hand.get(1).unwrap_or(&empty_string));
            println!("         {}", hand.get(2).unwrap_or(&empty_string));

            println!("         {}", discard_pile.get(0).unwrap_or(&empty_string));
            println!("Discard: {}", discard_pile.get(1).unwrap_or(&empty_string));
            println!("         {}", discard_pile.get(2).unwrap_or(&empty_string));

            let mut loop_player_revealed_sets = vec![];

            for set in &loop_player.called_sets {
                for item in &set.set.tiles {
                    loop_player_revealed_sets.push(item.clone());
                }

                loop_player_revealed_sets.push(INVALID_TILE);
            }

            let loop_player_revealed_sets = &mahjong_tiles_strs(&loop_player_revealed_sets, 1000);

            println!("         {}", loop_player_revealed_sets.get(0).unwrap_or(&empty_string));
            println!("Opened:  {}", loop_player_revealed_sets.get(1).unwrap_or(&empty_string));
            println!("         {}", loop_player_revealed_sets.get(2).unwrap_or(&empty_string));



            print!("\n");

        }
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
        println!("{: ^SCREEN_WIDTH$}", if *opposite_player == game.players[game.curr_player_idx] { ACTIVE_PLAYER_MARKER } else { "" });

        let empty_string = String::from("");

        let left_wind_str = format!("pts:{} wind:{}",left_player.points ,left_player.seat_wind);
        let mut left_margin_iter = std::iter::repeat(&empty_string).take(MIDDLE_HEIGHT / 2).chain(
            std::iter::once(&left_wind_str).chain(
                std::iter::repeat(&empty_string).take((MIDDLE_HEIGHT / 2) + 1)
            )
        );


        let mut left_active_iter = std::iter::repeat(empty_string.as_ref()).take(MIDDLE_HEIGHT / 2 ).chain(
            std::iter::once(if game.players[game.curr_player_idx] == *left_player { ACTIVE_PLAYER_MARKER } else { "" }).chain(
                std::iter::repeat(empty_string.as_ref())
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

        let mut right_active_iter = std::iter::repeat(empty_string.as_ref()).take(MIDDLE_HEIGHT / 2 - 1).chain(
            std::iter::once(if game.players[game.curr_player_idx] == *right_player { ACTIVE_PLAYER_MARKER } else { "" }).chain(
                std::iter::repeat(empty_string.as_ref())
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

        let round_wind_str = format!("Round Wind: {}", game.round_wind);
        let mut middle_discard_iter = opposite_discard_strs.iter().chain(
            std::iter::repeat(&empty_string).take( (MIDDLE_HEIGHT - opposite_discard_strs.len() - curr_discard_strs.len()) / 2 - 1).chain(
                std::iter::once(&round_wind_str).chain(
                    std::iter::repeat(&empty_string).take( (MIDDLE_HEIGHT - opposite_discard_strs.len() - curr_discard_strs.len()) / 2 - 1).chain(
                        curr_discard_strs.iter()
                )
            ))
        );

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
                format!("{: ^MARGIN$}{: >TILE_SIDE_VIEW_LEN$}", left_margin_iter.next().unwrap_or(&empty_string), left_hand_iter.next().unwrap_or(&empty_string)),
                format!("{: ^SCREEN_MID_WIDTH_THIRD$}{: ^SCREEN_MID_WIDTH_THIRD$}{: ^SCREEN_MID_WIDTH_THIRD$}", 
                    format!("{: <1}{: ^SCREEN_MID_WIDTH_THIRD_MINUS_ONE$}", left_active_iter.next().unwrap() ,left_discard_iter.next().unwrap_or(&empty_string)), 
                    format!("{}", middle_discard_iter.next().unwrap_or(&empty_string)), 
                    format!("{: ^SCREEN_MID_WIDTH_THIRD_MINUS_ONE$}{: >1}", right_discard_iter.next().unwrap_or(&empty_string), right_active_iter.next().unwrap())),
                format!("{: <TILE_SIDE_VIEW_LEN$}{: ^MARGIN$}", right_hand_iter.next().unwrap_or(&empty_string), right_margin_iter.next().unwrap_or(&empty_string))
            );
        }



        // print current player
        let current_hand = &mahjong_tiles_strs(&curr_player.hand, 1000);
        let mut current_player_revealed_sets = vec![];

        for set in &curr_player.called_sets {
            for item in &set.set.tiles {
                current_player_revealed_sets.push(item.clone());
            }

            current_player_revealed_sets.push(INVALID_TILE);
        }

        let current_player_revealed_sets = &mahjong_tiles_strs(&current_player_revealed_sets, MARGIN_AND_TILE_WIDTH);
        let mut revealed_sets_iter = current_player_revealed_sets.iter();

        println!("{: ^MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: ^MARGIN_AND_TILE_WIDTH$}", 
            " ", 
            format!("{: ^FURITEN_LEN$}{: ^SCREEN_MID_WIDTH_MINUS_FURITEN_LEN_TWICE$}{: ^FURITEN_LEN$}",
                if curr_player.furiten { "FURITEN" } else { " " },
                if *curr_player == game.players[game.curr_player_idx] { ACTIVE_PLAYER_MARKER } else { "" },
                " "),
            " ");
        
        println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", current_hand[0], revealed_sets_iter.next().unwrap_or(&empty_string));
        println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", current_hand[1], revealed_sets_iter.next().unwrap_or(&empty_string));
        println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", current_hand[2], revealed_sets_iter.next().unwrap_or(&empty_string));

        if player_idx == game.curr_player_idx
        {
            let mut numbers = String::from("");
            for i in 1..(curr_player.hand.len() + 1) {    numbers.push_str(&format!(" {: <2} ", i));    }
            println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", numbers, revealed_sets_iter.next().unwrap_or(&empty_string));
        }

        println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", format!("pts:{} wind:{}", curr_player.points, curr_player.seat_wind), revealed_sets_iter.next().unwrap_or(&empty_string));

        // println!("{}", " ".repeat(SCREEN_WIDTH));
        // println!("{}", " ".repeat(SCREEN_WIDTH));
        // println!("{}", " ".repeat(SCREEN_WIDTH));
        println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", " ", revealed_sets_iter.next().unwrap_or(&empty_string));
        println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", " ", revealed_sets_iter.next().unwrap_or(&empty_string));
        println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", " ", revealed_sets_iter.next().unwrap_or(&empty_string));
        println!("{: >MARGIN_AND_TILE_WIDTH$}{: ^SCREEN_MID_WIDTH$}{: <MARGIN_AND_TILE_WIDTH$}", " ", " ", revealed_sets_iter.next().unwrap_or(&empty_string));
        println!("{}", " ".repeat(SCREEN_WIDTH));
}


/// mutability of game is only for debug
pub fn get_player_discard_idx(game : &mut Game, player_idx : usize, player_can_win : bool, player_can_close_kan : bool) -> DiscardChoices
{
            game.dump_game_state();
            output_game(game, player_idx);

            let mut input = String::from("");

            if player_can_win
            {
                println!("You can win with your current hand right now. Win? Type 'y' for yes or 'n' for no");
                loop {
                    std::io::stdin().read_line(&mut input).expect("stdin readline failed");
                    input = input.trim().to_lowercase();

                    if input == "y"
                    {
                        return DiscardChoices::Win;
                    }
                    else if input == "n"
                    {
                        output_game(game, player_idx);
                        break;
                    }
                    else
                    {
                        println!("Please enter 'y' or 'n'");
                    }
                }
            }

            println!("Enter which tile you would like to discard (\"n\" standing for \"new\" works for the rightmost drawn tile)");

            std::io::stdin().read_line(&mut input).expect("stdin readline failed");
            input = input.trim().to_lowercase();

            return loop {

                let input_as_num = input.parse::<usize>();

                if let Ok(input_as_num) = input_as_num
                {
                    if input_as_num > game.players[player_idx].hand.len() || input_as_num == 0
                    {
                        println!("Enter a number within the valid range!");
                    }
                    else
                    {
                        // We give the player numbers starting from 1, but indexes start from 0
                        break DiscardChoices::DiscardTile(input_as_num - 1);
                    }
                }
                else if input == "n"
                {
                    break DiscardChoices::DiscardTile(game.players[player_idx].hand.len() - 1);
                }
                else if input == "debug"
                {
                    loop {
                        println!("What debug option? 1 - add tile, 2 - remove tile, 'w' - get winning hand, 'f' - set or unset furiten, 'e' - exit debug mode");
                        let mut input = String::from("");
                        std::io::stdin().read_line(&mut input).expect("stdin readline failed");
                        input = input.trim().to_lowercase();

                        let mut input_as_num = input.parse::<usize>();

                        if let Ok(input_as_num) = input_as_num
                        {
                            if input_as_num == 1
                            {
                                output_game(game, player_idx);
                                println!("Enter tile to add in this format: {{h:ea}} - honor east wind tile. {{m:3}} 3 man tile (red not supported)");
                                let mut input = String::from("");
                                std::io::stdin().read_line(&mut input).expect("Reading in from stdin failed");
                                input = input.trim().to_lowercase();
                                let mut input_successful : bool = false;

                                if input.as_bytes()[0] == ('{' as u8)
                                {
                                    let suit = input.as_bytes()[1];
                                    if input.as_bytes()[2] == (':' as u8)
                                    {
                                        let value = input.as_bytes()[3];
                                        if input.as_bytes()[4] == ('}' as u8)
                                        {
                                            let new_tile = Tile {
                                                suit : match suit as char {
                                                    'h' => Suit::Honor,
                                                    'm' => Suit::Man,
                                                    'p' => Suit::Pin,
                                                    's' => Suit::Sou,
                                                    _ => Suit::Honor
                                                },
                                                value : match value as char {
                                                    '1' => SuitVal::One, '2' => SuitVal::Two,
                                                    '3' => SuitVal::Three, '4' => SuitVal::Four,
                                                    '5' => SuitVal::Five, '6' => SuitVal::Six,
                                                    '7' => SuitVal::Seven, '8' => SuitVal::Eight,
                                                    '9' => SuitVal::Nine,
                                                    'e' => SuitVal::East, 's' => SuitVal::South,
                                                    'w' => SuitVal::West, 'n' => SuitVal::North,
                                                    'h' => SuitVal::White, 'r' => SuitVal::Red,
                                                    'g' => SuitVal::Green,
                                                    _ => SuitVal::Red
                                                },
                                                red : false
                                            };

                                            game.players[player_idx].hand.push(new_tile);
                                            input_successful = true;
                                        }
                                    }
                                }

                                if ! input_successful
                                {
                                    println!("Did not successfully enter new tile");
                                }
                            }
                            else if input_as_num == 2
                            {
                                output_game(game, player_idx);
                                println!("Enter tile to remove by index");
                                let mut input = String::from("");
                                std::io::stdin().read_line(&mut input).expect("Reading in from stdin failed");
                                input = input.trim().to_lowercase();
                                let mut input_as_num = input.parse::<usize>();

                                if let Ok(input_as_num) = input_as_num
                                {
                                    if input_as_num <= game.players[player_idx].hand.len() && input_as_num != 0
                                    {
                                        game.players[player_idx].hand.remove(input_as_num - 1);
                                        output_game(game, player_idx);
                                    }
                                    else {
                                        println!("Invalid index for removal {}", input_as_num);
                                    }
                                }
                            }
                        }
                        else if input == "f"
                        {
                            game.players[player_idx].furiten = !game.players[player_idx].furiten;
                        }
                        else if input == "w"
                        {
                            game.players[player_idx].hand = vec![
                                Tile::man_tile(1),
                                Tile::man_tile(1),
                                Tile::man_tile(1),
                                Tile::man_tile(1),
                                Tile::man_tile(2),
                                Tile::man_tile(3),
                                Tile::man_tile(4),
                                Tile::man_tile(5),
                                Tile::man_tile(6),
                                Tile::man_tile(7),
                                Tile::man_tile(8),
                                Tile::man_tile(9),
                                Tile::man_tile(9),
                                Tile::man_tile(9),
                ];
                        }
                        else if input == "e"
                        {
                            output_game(game, player_idx);
                            break;
                        }

                        output_game(game, player_idx);
                    }
                }
                else
                {
                    output_game(game, player_idx);
                    println!("Enter a tile to discard!");
                }

                input.clear();
                std::io::stdin().read_line(&mut input).expect("stdin readline failed");
                input = input.trim().to_lowercase();
            };

}


pub fn get_player_call_choice(game : &Game, player_idx : usize, discarded_tile : Tile, all_possible_calls : &Vec<CalledSet>) -> Option<CalledSet>
{
    game.dump_game_state();
    output_game(game, player_idx);
    // we need to calculate how many tile spaces to show to the player so they can choose which set to call on
    // we will insert an empty tile in between every set for spacing
    let mut num_tiles_to_display : usize = 0;

    for set in all_possible_calls
    {
        num_tiles_to_display += set.set.tiles.len();

        num_tiles_to_display += 1; // empty space between sets
    }
    
    if num_tiles_to_display > 0
    {
        num_tiles_to_display -= 1; // removing the last unneeded empty tile space. We already know that all_possible_calls has a length of at least 1
    }

    // we need lines to display the tiles, a line for numbers to differentiate sets to pick, a line for input prompt, and a line for input
    const LINES_AVAILABLE_FOR_TILES : usize = SCREEN_HEIGHT - 3;

    if (num_tiles_to_display * TILE_FRONT_VIEW_LEN) > (LINES_AVAILABLE_FOR_TILES * TILE_FRONT_VIEW_LEN)
    {
        panic!("There were more tiles available to call on then there was space to print them. {}:tiles_to_display, {}:Lines available", num_tiles_to_display, LINES_AVAILABLE_FOR_TILES);
    }

    let mut call_tile_strs : Vec<String> = vec![];

    for (i, set) in all_possible_calls.iter().enumerate() {
        let mut set_strs = mahjong_tiles_strs(&set.set.tiles, SCREEN_WIDTH);

        set_strs.insert(0, (i + 1).to_string());

        if let CallTypes::Ron(tile) = set.call_type
        {
            set_strs[0].push_str("-ron");
        }
        else {
            match set.set.set_type {
                SetType::Kan => set_strs[0].push_str("-kan"),
                SetType::Sequence => set_strs[0].push_str("-seq"),
                SetType::Triplet => set_strs[0].push_str("-trip"),
                _ => ()
        }

        }
            for i in 0..set_strs.len()
        {
            if call_tile_strs.len() < set_strs.len()
            {
                call_tile_strs.push(set_strs[i].clone());
            }
            else
            {
                call_tile_strs[i].push_str(&set_strs[i]);
            }
        }

        // insert empty tiles
        if call_tile_strs[0].chars().count() < (SCREEN_WIDTH - TILE_FRONT_VIEW_LEN) && call_tile_strs.len() > 0
        {
            let len = call_tile_strs.len();

            call_tile_strs[len - 1] += "    ";
            call_tile_strs[len - 2] += "    ";
            call_tile_strs[len - 3] += "    ";
            call_tile_strs[len - 4] += "    ";
        }
    }


    for line in call_tile_strs {
        println!("{: ^SCREEN_WIDTH$}", line);
    }


    loop {
        println!("Would you like to call? Press the number for the corresponding call, or 'n' to skip");

        let mut input = String::from("");
        std::io::stdin().read_line(&mut input).expect("bruhhhhhhhh input failed");

        let input = input.trim().to_lowercase();
        let input_as_num = input.parse::<usize>();

        if let Ok(input_as_num) = input_as_num
        {
            if input_as_num <= all_possible_calls.len() && input_as_num != 0
            {
                return Some(all_possible_calls[input_as_num - 1].clone());
            }
        }
        else if input == "n"
        {
            return None;
        }

        println!("Please enter a valid number!");
    }

}




pub fn output_player_win_or_lose(winning_player : &Player, human_is_playing : bool) -> ()
{
    let you_win_str = vec!["__   __                   __        __  _           _   _   _   _   _   _",
                                    "\\ \\ / /   ___    _   _    \\ \\      / / (_)  _ __   | | | | | | | | | | | |",
                                    " \\ V /   / _ \\  | | | |    \\ \\ /\\ / /  | | | \'_ \\  | | | | | | | | | | | |",
                                    "  | |   | (_) | | |_| |     \\ V  V /   | | | | | | |_| |_| |_| |_| |_| |_|",
                                    "  |_|    \\___/   \\__,_|      \\_/\\_/    |_| |_| |_| (_) (_) (_) (_) (_) (_)"];

    let you_lose_str = vec!["\\ \\ / /   ___    _   _    | |       ___    ___    ___  | | | | | | | | | | | |",
                                    " \\ V /   / _ \\  | | | |   | |      / _ \\  / __|  / _ \\ | | | | | | | | | | | |",
                                    "  | |   | (_) | | |_| |   | |___  | (_) | \\__ \\ |  __/ |_| |_| |_| |_| |_| |_|",
                                    "  |_|    \\___/   \\__,_|   |_____|  \\___/  |___/  \\___| (_) (_) (_) (_) (_) (_)"];



    if ! DEBUG_OUTPUT
    {
        clearscreen::clear().expect("Could not clear screen");
    }

    let mut winning_hand = mahjong_tiles_strs(&winning_player.hand, 1000);
    let mut winning_sets = vec![];
    for set in &winning_player.called_sets {
        for tile in &set.set.tiles {
            winning_sets.push(tile.clone());
        }
        winning_sets.push(INVALID_TILE);
    }
    let winning_sets = mahjong_tiles_strs(&winning_sets, 1000);

    for i in 0..winning_sets.len()
    {
        winning_hand[i].push_str("   ");
        winning_hand[i].push_str(winning_sets[i].as_str());
    }

    let winning_hand : Vec<&str> = winning_hand.iter().map(|string| string.as_str()).collect();

    let empty_string = "";
    let mut output_win_string_iter = std::iter::repeat(&empty_string).take(
        (SCREEN_HEIGHT - you_win_str.len()) / 2
    ).chain(
        (if winning_player.is_human { you_win_str.iter() } else { you_lose_str.iter() }).chain(
            std::iter::once(&empty_string).chain(
                std::iter::once(&"The winning had was").chain(
                    std::iter::once(&"----------------------").chain
                    (
                        winning_hand.iter()
                    )
                )
            )
            )
    );

    for i in 0..SCREEN_HEIGHT
    {
        println!("{: ^SCREEN_WIDTH$}", output_win_string_iter.next().unwrap_or(&empty_string));
    }


    if human_is_playing
    {
        let mut worthless = String::from("");
        std::io::stdin().read_line(&mut worthless).expect("Stdin failed");
    }
}
