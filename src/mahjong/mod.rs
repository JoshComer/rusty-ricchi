use std::io::Repeat;


//pub mod game_structs {
use strum::IntoEnumIterator;
use rand::{Rng, rngs::adapter::ReseedingRng};
use unicode_segmentation::UnicodeSegmentation;
use num::{pow, bigint::ParseBigIntError, One};

pub enum OutputView {
    BoardView,
    RowView,
}

const DEBUG_OUTPUT : bool = true;
pub static OUTPUT_METHOD : OutputView = OutputView::BoardView;

// local imports

pub mod tui_output;

pub mod tile;
use tile::*;

pub mod player;
use player::*;

pub mod scoring;
use scoring::*;

pub mod utils;
use utils::*;

pub mod command;
use command::*;

// TODO: TESTCASE: m2,m3,m4,p3,p4,p5,p8,s4,s4,s4,s6,s8,s8,s8 - should have four triplets, but no pairs


enum RepeatHand {
        DealerWon,
        RotateWinds
    }


pub enum NextPlayerOrWin {
    NextPlayer(usize),
    Winner(usize)
}


const NUM_GAME_TILES : usize = 136;

pub struct GameTiles {
    tiles : [Tile; NUM_GAME_TILES],
    pub next_tile : usize,

    dora_idx : usize,
    ura_dora_idx : usize,
}

impl GameTiles {
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
}

pub struct GameData {
    pub human_is_playing : bool,

    pub num_called_tiles : usize,

    round_wind : SuitVal,
    player_just_called : bool,
}

pub struct GamePlayers {
    curr_player_idx : usize,
    player_list : [Player; NUM_PLAYERS],
}

impl GamePlayers {
    fn current_player(&mut self) -> &mut Player
    {
        &mut self.player_list[self.curr_player_idx]
    }
}

#[allow(dead_code)]
pub struct Game {
//    walls   : GameTiles,
//    data    : GameData,
//    players : GamePlayers,

    tiles : [Tile; NUM_GAME_TILES],
    pub next_tile : usize,

    dora_idx : usize,
    ura_dora_idx : usize,

    curr_player_idx : usize,
    players : [Player; NUM_PLAYERS],

    pub human_is_playing : bool,

    pub num_called_tiles : usize,

    round_wind : SuitVal,
    player_just_called : bool,


}

impl Default for Game {
    fn default() -> Self {
        Game {
//            data : GameData {
                human_is_playing : false,
                player_just_called : false,
                round_wind : SuitVal::East,
                num_called_tiles : 0,
//            },

//            players : GamePlayers {
                curr_player_idx : usize::MAX,
                players : [
                    Player::default().set_seat_wind(SuitVal::East).set_number(0).set_is_human().to_owned(),
                    Player::default().set_seat_wind(SuitVal::South).set_number(1).to_owned(),
                    Player::default().set_seat_wind(SuitVal::West).set_number(2).to_owned(),
                    Player::default().set_seat_wind(SuitVal::North).set_number(3).to_owned(),
                ],
//            },

//            walls : GameTiles {

            dora_idx : NUM_GAME_TILES - 14,
            ura_dora_idx : NUM_GAME_TILES - 7,

            tiles : [
                Tile::man_tile(1), Tile::man_tile(1), Tile::man_tile(1), Tile::man_tile(1),
                Tile::man_tile(2), Tile::man_tile(2), Tile::man_tile(2), Tile::man_tile(2),
                Tile::man_tile(3), Tile::man_tile(3), Tile::man_tile(3), Tile::man_tile(3),
                Tile::man_tile(4), Tile::man_tile(4), Tile::man_tile(4), Tile::man_tile(4),
                Tile::man_tile(5), Tile::man_tile(5), Tile::man_tile(5),
                Tile { suit : Suit::Man, value : SuitVal::Five, red : true},
                Tile::man_tile(6), Tile::man_tile(6), Tile::man_tile(6), Tile::man_tile(6),
                Tile::man_tile(7), Tile::man_tile(7), Tile::man_tile(7), Tile::man_tile(7),
                Tile::man_tile(8), Tile::man_tile(8), Tile::man_tile(8), Tile::man_tile(8),
                Tile::man_tile(9), Tile::man_tile(9), Tile::man_tile(9), Tile::man_tile(9),

                Tile::pin_tile(1), Tile::pin_tile(1), Tile::pin_tile(1), Tile::pin_tile(1),
                Tile::pin_tile(2), Tile::pin_tile(2), Tile::pin_tile(2), Tile::pin_tile(2),
                Tile::pin_tile(3), Tile::pin_tile(3), Tile::pin_tile(3), Tile::pin_tile(3),
                Tile::pin_tile(4), Tile::pin_tile(4), Tile::pin_tile(4), Tile::pin_tile(4),
                Tile::pin_tile(5), Tile::pin_tile(5), Tile::pin_tile(5),
                Tile { suit : Suit::Pin, value : SuitVal::Five, red : true},
                Tile::pin_tile(6), Tile::pin_tile(6), Tile::pin_tile(6), Tile::pin_tile(6),
                Tile::pin_tile(7), Tile::pin_tile(7), Tile::pin_tile(7), Tile::pin_tile(7),
                Tile::pin_tile(8), Tile::pin_tile(8), Tile::pin_tile(8), Tile::pin_tile(8),
                Tile::pin_tile(9), Tile::pin_tile(9), Tile::pin_tile(9), Tile::pin_tile(9),

                Tile::sou_tile(1), Tile::sou_tile(1), Tile::sou_tile(1), Tile::sou_tile(1),
                Tile::sou_tile(2), Tile::sou_tile(2), Tile::sou_tile(2), Tile::sou_tile(2),
                Tile::sou_tile(3), Tile::sou_tile(3), Tile::sou_tile(3), Tile::sou_tile(3),
                Tile::sou_tile(4), Tile::sou_tile(4), Tile::sou_tile(4), Tile::sou_tile(4),
                Tile::sou_tile(5), Tile::sou_tile(5), Tile::sou_tile(5),
                Tile { suit : Suit::Sou, value : SuitVal::Five, red : true},
                Tile::sou_tile(6), Tile::sou_tile(6), Tile::sou_tile(6), Tile::sou_tile(6),
                Tile::sou_tile(7), Tile::sou_tile(7), Tile::sou_tile(7), Tile::sou_tile(7),
                Tile::sou_tile(8), Tile::sou_tile(8), Tile::sou_tile(8), Tile::sou_tile(8),
                Tile::sou_tile(9), Tile::sou_tile(9), Tile::sou_tile(9), Tile::sou_tile(9),


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
//            }
        }
    }
}

impl Game {
    fn current_player(&mut self) -> &mut Player
    {
        &mut self.players[self.curr_player_idx]
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


    /// returns the position of the human player within the player_list
    fn human_player_position(&self) -> usize
    {
        0
    }

    fn draw_from_dead_wall(&mut self) -> Tile
    {
        unimplemented!()
    }

    fn open_closed_kan(&mut self, player_idx : usize, kanned_tile : Tile) -> Option<usize>
    {
        // add kan to revealed sets
        self.players[player_idx].hand.retain(|hand_tile| *hand_tile != kanned_tile);
        self.players[player_idx].called_sets.push(
            CalledSet {
                call_type : CallTypes::ClosedKan,
                set : Set::kan(kanned_tile),
            }
        );

        let next_tile = self.draw_from_dead_wall();
        self.players[player_idx].hand.push( next_tile );

        let player_can_win = self.players[player_idx].check_complete_hand_and_update_waits();
        // TODO: Rinshan Kaihou

        // draw next tile. It's illegal to kan on the last tile, so there's always a tile to draw or we've broken the rules
        // TODO: last tile from the wall is added to dead wall here
        let discard_or_win = tui_output::get_player_discard_idx(self, player_idx, player_can_win, false);

        match discard_or_win {

            DiscardChoices::Win => return None,
            DiscardChoices::DiscardTile(tile_idx) => return Some(tile_idx),
            DiscardChoices::OpenClosedKan(newly_kanned_tile) => return self.open_closed_kan(player_idx, newly_kanned_tile),
            DiscardChoices::AddedKan(sdlkfj) => unimplemented!()
        }

    }

    fn reveal_dora(&mut self) -> ()
    {
        if self.dora_idx < NUM_GAME_TILES
        {
            self.dora_idx += 1;
        }
    }

    fn dump_game_state(&self)
    {
        for i in 0..NUM_PLAYERS
        {
            println!("Player {}", i);
            println!("-------------------------");
            self.players[i].dump_player_state();
            print!("\n");
        }
    }

    fn get_calls_on_discard(&mut self, discarded_tile : Tile) -> Vec<(usize, CalledSet)>
    {
        let mut calls_made : Vec<(usize, CalledSet)> = Vec::with_capacity(4);

        for i in 0..NUM_PLAYERS{
            if self.players[i] != self.players[self.curr_player_idx] && self.players[i].furiten == false
            {
                println!("Checking if player {} needs tile {}. Their callable_tiles len is {}", i, discarded_tile, self.players[i].callable_tiles.len());
                if self.players[i].callable_tiles.contains_key(&discarded_tile)
                {
                    let entry = self.players[i].callable_tiles.entry(discarded_tile).or_default();

                    let mut possible_calls = entry.clone();

                    // checks if we're directly to the right of whoever discarded without overflow
                    // if we're not, then we can't chii
                    if (i + NUM_PLAYERS - 1) % NUM_PLAYERS != (self.curr_player_idx)
                    {
                        possible_calls.chii = false;
                    }

                    if possible_calls.any_field_true()
                    {
                        let call_made = Player::choose_whether_to_call(i, discarded_tile, self);

                        if let Some(call_made) = call_made {
                            calls_made.push((i, call_made));
                        }
                    }
                }
            }
        }

        return calls_made;
    }


    /// queries players if they can and want to make a call off of a discard
    /// returns the index to the next player if a call was made
    fn execute_call_or_advance_player(&mut self, discarded_tile : Tile) -> NextPlayerOrWin
    {
        let mut calls_made : Vec<(usize, CalledSet)> = self.get_calls_on_discard(discarded_tile);

        // multiple calls can be made simultaneously. Higher precedence gets to call, and multiple
        // people can ron at the same time too
        if calls_made.len() > 1
        {
            self.player_just_called = true;
            // remove the discarded tile from the discarder's pile
            self.current_player().discard_pile.pop();

            let highest_call_precedence = calls_made.iter().max_by_key(
                |call| call.1.call_type.precedence()
            ).unwrap().1.call_type.precedence();

            calls_made.retain(|call|
                call.1.call_type.precedence() == highest_call_precedence
            );

            // this is only possible if multiple people Ron at the same time
            if calls_made.len() > 1
            {
                unimplemented!();
            }
            else
            {
                let call = &calls_made[0];
                self.players[call.0].open_tiles_with_call(discarded_tile, call.1.clone());
                self.current_player().tiles_others_called.push(discarded_tile);
                // switch to the player who made the call
                return match call.1.call_type
                {
                    CallTypes::Ron(set) => {
                        self.players[call.0].ron_or_tsumo = WinningMethod::Ron(self.curr_player_idx);
                        NextPlayerOrWin::Winner(call.0)
                    },
                    _ => NextPlayerOrWin::NextPlayer(call.0)
                }
            }
        }
        else if calls_made.len() == 1
        {
            self.player_just_called = true;
             // remove the discarded tile from the discarder's pile
            self.current_player().discard_pile.pop();

            let call = &calls_made[0];
            self.players[call.0].open_tiles_with_call(discarded_tile, call.1.clone());
            self.current_player().tiles_others_called.push(discarded_tile);

            return match call.1.call_type
            {
                CallTypes::Ron(set) => NextPlayerOrWin::Winner(call.0),
                _ => NextPlayerOrWin::NextPlayer(call.0)
            }
        }
        else
        {
            self.player_just_called = false;
            return NextPlayerOrWin::NextPlayer((self.curr_player_idx + 1) % NUM_PLAYERS);
        }
    }

    fn player_discard_tile(&mut self, player_idx : usize, discard_idx : usize) -> Tile
    {
        if DEBUG_OUTPUT
        {
            println!("Player number {} discarded tile {}. Deck marker is {}", player_idx, discard_idx, self.next_tile);
        }

        let discarded_tile = self.players[player_idx].hand.remove(discard_idx);
        self.players[player_idx].discard_pile.push(discarded_tile);
        self.players[player_idx].sort_hand();
        self.players[player_idx].update_callable_tiles();
        self.players[player_idx].check_complete_hand_and_update_waits();

        return discarded_tile;
    }

    fn player_choose_discard_idx_or_win(&mut self, player_idx : usize) -> Option<usize>
    {
        let mut discard_idx : usize;
        let mut player = &mut self.players[player_idx];

        // make a choice on winning or which tile to discard
        if player.is_human
        {
            // TODO: Maybe move this part to the tui function? Haven't added in win condition output functionality yet
            let player_current_hand = player.hand.clone();
            player.sort_hand();
            let player_can_win : bool = player.check_complete_hand_and_update_waits();
            player.hand = player_current_hand; // checking for a complete hand requires it be sorted
                                                                // but we want the newest drawn tile to be shown to the right for discarding purposes

            tui_output::output_game(self, player_idx);
            let discard_choice = tui_output::get_player_discard_idx(self, player_idx, player_can_win, false);
            let player = &mut self.players[player_idx];

            match discard_choice {
                DiscardChoices::DiscardTile(idx) => discard_idx = idx,
                DiscardChoices::Win => {
                    player.ron_or_tsumo = WinningMethod::Tsumo;
                    return None
                },
                DiscardChoices::OpenClosedKan(kanned_tile) => match self.open_closed_kan(player_idx, kanned_tile)
                    {
                        Some(idx) => discard_idx = idx,
                        // TODO: Update winning method accordingly if needed
                        None => return None
                    },
                DiscardChoices::AddedKan(kanned_tile) => unimplemented!(),
            }
        }
        // computer picks which to discard
        else
        {
            discard_idx = player.ai_discard();
            tui_output::output_game(self, self.human_player_position());
            let mut input = String::from("");
            std::io::stdin().read_line(&mut input).expect("stdin readline failed");
        }

        return Some(discard_idx);
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
    {   // players start with 13 tiles and draw their 14th each turn
        for player in &mut self.players{
            player.hand.clear();
        }

        for i in 0..((PLAYER_HAND_SIZE - 1) * NUM_PLAYERS)
        {
            self.players[i % NUM_PLAYERS].hand.push(self.tiles[self.next_tile as usize]);
            self.next_tile += 1;
        }

        for i in 0..NUM_PLAYERS
        {
            self.players[i].sort_hand();
        }
    }

    /// Performs setup at the start of each hand such as shuffling tiles,
    /// divying them to players, clearing player discards, etc.
    fn setup_for_hand(&mut self) -> ()
    {
        self.shuffle();
        self.divy_tiles_to_players();

        // clear discards
        for player in &mut self.players{
            player.discard_pile.clear();
            player.tiles_others_called.clear();
            player.called_sets.clear();
            player.winning_call_tiles.clear();
            player.callable_tiles.clear();

            player.update_callable_tiles();
        }
    }

    fn play_hand(&mut self) -> RepeatHand
    {
        self.setup_for_hand();

        // Dealer is the east wind player
        self.curr_player_idx = self.players.iter()
            .position(|player| player.seat_wind == SuitVal::East)
            .expect("There was no player with East Wind who could be the dealer");

        loop
        {
                //draw the next tile or exhaustive draw
                if ! self.player_just_called
                {
                    let next_tile = self.draw_next_tile();
                    if next_tile.is_none()
                    {
                        scoring::score_points(self, None);
                        return RepeatHand::RotateWinds;
                    }

                    let next_tile = unsafe {next_tile.unwrap_unchecked()};
                    self.current_player().hand.push(next_tile);
                }

                // push the next tile without sorting to keep the tile on the right for display purposes
                // since after discarding
                let discarded_idx = self.player_choose_discard_idx_or_win(self.curr_player_idx);
                let mut discarded_tile : Tile;
                if let Some(idx) = discarded_idx
                {
                    discarded_tile = self.player_discard_tile(self.curr_player_idx, idx);
                }
                else // None
                {
                    // A player always discards, unless they chose to win
                        scoring::score_points(self, Some(self.curr_player_idx));
                        if self.players[self.curr_player_idx].seat_wind == self.round_wind
                        {
                            return RepeatHand::DealerWon;
                        }
                        else
                        {
                            return RepeatHand::RotateWinds;
                        }
                }

                self.current_player().sort_hand();

                let next_or_win = self.execute_call_or_advance_player(discarded_tile);

                match next_or_win {
                    NextPlayerOrWin::NextPlayer(next_index) => self.curr_player_idx = next_index,
                    NextPlayerOrWin::Winner(winner_index) => {
                        self.players[winner_index].ron_or_tsumo = WinningMethod::Ron(self.curr_player_idx);
                        self.curr_player_idx = winner_index;
                        scoring::score_points(self, Some(winner_index));

                        let temp_wind = self.round_wind;
                        if self.current_player().seat_wind == temp_wind
                        {
                            return RepeatHand::DealerWon;
                        }
                        else
                        {
                            return RepeatHand::RotateWinds;
                        }
                    }
                }
        };
    }

    fn play_round(&mut self) -> ()
    {
        const HANDS_PER_ROUND : usize = 4;

        let mut times_rotated : usize = 0;
        while times_rotated < HANDS_PER_ROUND
        {

            let rotate_or_stay = self.play_hand();

            if let RepeatHand::RotateWinds = rotate_or_stay
            {
                times_rotated += 1;

                for player in &mut self.players
                {
                    player.rotate_wind();
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

            // round winds change counter clockwise while player seat winds change clockwise (Weird)
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


pub enum PlayerAdvancement {
    NextPlayer(usize),
    Call(usize),
}










































// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------
//                                         Tests
// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------


#[test]
fn test_scoring()
{
    let mut game : Game = Game::default();

    game.players[0].ron_or_tsumo = WinningMethod::Ron(1);
    scoring::score_points(&mut game, Some(0));




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
        Tile::man_tile(1),
        Tile::man_tile(9),
        Tile::pin_tile(1),
        Tile::pin_tile(9),
        Tile::sou_tile(1),
        Tile::sou_tile(9),
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false }, // the duplicate for pair. Set as the last tile drawn, so it should be double yakuman
    );

    game.players[0].sort_hand();

    game.players[0].ron_or_tsumo = WinningMethod::Tsumo;
    game.players[0].last_picked_tile = Tile { suit : Suit::Honor, value : SuitVal::East, red : false };
    game.next_tile = 1;

    assert_eq!(scoring::yakuman_kokushi_musou(&game.players[0], &game), 2);
    assert_eq!(scoring::yakuman_chiihou(&game.players[0], &game), 1);
    assert_eq!(scoring::yakuman_daisangen(&game.players[0], &game), 0);

    scoring::score_points(&mut game, Some(0));

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
        Tile::pin_tile(7),
        Tile::pin_tile(7),
        Tile::pin_tile(7),
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
    );

    game.players[0].called_sets = vec!(
        CalledSet {
        set : Set {
                set_type : SetType::Triplet,
                tiles : vec![
                    Tile { suit : Suit::Honor, value : SuitVal::Green, red : false } ; 3
                ],
            },
        call_type : CallTypes::Ron(SetType::Triplet)
        }
    );

    game.players[0].sort_hand();

    game.players[0].ron_or_tsumo = WinningMethod::Ron(2);
    game.players[0].last_picked_tile = Tile { suit : Suit::Honor, value : SuitVal::Green, red : false };
    game.next_tile = 46;

    assert_eq!(yakuman_kokushi_musou(&game.players[0], &game), 0);
    assert_eq!(yakuman_chiihou(&game.players[0], &game), 0);
    assert_eq!(yakuman_daisangen(&game.players[0], &game), 1);

    scoring::score_points(&mut game, Some(0));

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
        Tile::man_tile(4),
        Tile::man_tile(4),
        Tile::man_tile(4),
        Tile::pin_tile(2),
        Tile::pin_tile(2),
        Tile::pin_tile(2),
        Tile::sou_tile(9),
        Tile::sou_tile(9),
        Tile::sou_tile(9),
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
    );

    game.players[0].called_sets = vec!(
        CalledSet {
            set : Set {
                    set_type : SetType::Pair,
                    tiles : vec![
                        Tile {  suit : Suit::Man, value : SuitVal::Seven, red : false} ; 2
                    ],
                },
            call_type : CallTypes::Ron(SetType::Pair)
        }
    );

    game.players[0].sort_hand();

    game.players[0].last_picked_tile = Tile::man_tile(7);
    game.players[0].ron_or_tsumo = WinningMethod::Ron(3);
    game.next_tile = 45;


    assert_eq!(yakuman_suuankou(&game.players[0], &game), 2);

    scoring::score_points(&mut game, Some(0));

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
        Tile::man_tile(4),
        Tile::man_tile(5),
        Tile::man_tile(6),
        Tile { suit : Suit::Honor, value : SuitVal::South, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::South, red : false },
    );

    winning_player.sort_hand();

    winning_player.called_sets = vec!(
        CalledSet {
            set : Set {
            set_type : SetType::Kan,
            tiles : vec![
                Tile::man_tile(1) ; 4
            ],
            },
            call_type : CallTypes::ClosedKan
        },
        CalledSet {
            set : Set {
            set_type : SetType::Kan,
            tiles : vec![
                Tile { suit : Suit::Honor, value : SuitVal::Red, red : false } ; 4
            ],
            },
            call_type : CallTypes::ClosedKan
        },
        CalledSet {
            set : Set {
            set_type : SetType::Triplet,
            tiles : vec![
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                ],
            },
            call_type : CallTypes::Ron(SetType::Triplet)
        }
    );

    winning_player.last_picked_tile = Tile { suit : Suit::Honor, value : SuitVal::East, red : false };
    winning_player.winning_wait = Some(WaitType::Shanpon);

    winning_player.ron_or_tsumo = WinningMethod::Ron(0);

    // test without rounding to ensure fu is correct
    assert_eq!(game.players[1].hand_fu(&game, false), 102);
    //    assert_eq!(winning_player.hand_yaku_in_han(), 1);
    // TODO: Check for han value in hand
    scoring::score_points(&mut game, Some(1));


}



#[test]
fn test_fu_2()
{
    let mut game = Game::default();

    assert_eq!(game.players[0].seat_wind, SuitVal::East);

    let mut winning_player = &mut game.players[0];

    winning_player.hand = vec!(
        Tile::sou_tile(2),
        Tile::sou_tile(3),
        Tile::sou_tile(4),
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
    );

    winning_player.sort_hand();

    winning_player.called_sets = vec!(
        CalledSet {
            set : Set {
            set_type : SetType::Kan,
            tiles : vec![
                Tile::pin_tile(1) ; 4
            ],
            },
            call_type : CallTypes::ClosedKan
        },
        CalledSet {
            set : Set {
            set_type : SetType::Kan,
            tiles : vec![
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false } ; 4
            ],

            },
            call_type : CallTypes::ClosedKan
        },
        CalledSet {
            set : Set {
            set_type : SetType::Kan,
            tiles : vec![
                Tile::man_tile(9) ; 4
            ],
            },
            call_type : CallTypes::OpenKan
        },
    );

    winning_player.last_picked_tile = Tile { suit : Suit::Honor, value : SuitVal::East, red : false };
    winning_player.winning_wait = Some(WaitType::Tanki);

    winning_player.ron_or_tsumo = WinningMethod::Tsumo;


    // test without rounding to ensure fu is correct
    assert_eq!(game.players[0].hand_fu(&game, false), 108);
    //    assert_eq!(winning_player.hand_yaku_in_han(), 1);
    // TODO: Check for han value in hand
    scoring::score_points(&mut game, Some(0));


}

#[test]
fn test_fu_open_pinfu()
{
    let mut game = Game::default();

    assert_eq!(game.players[0].seat_wind, SuitVal::East);

    let mut winning_player = &mut game.players[0];

    winning_player.hand = vec!(
        Tile::man_tile(1),
        Tile::man_tile(2),
        Tile::man_tile(3),
        Tile::sou_tile(2),
        Tile::sou_tile(2),
    );

    winning_player.called_sets = vec!(
        CalledSet {
            set : Set {
                set_type : SetType::Sequence,
                tiles : vec![
                    Tile::pin_tile(2),
                    Tile::pin_tile(3),
                    Tile::pin_tile(4),
                ],
            },
            call_type : CallTypes::Chii
        },
        CalledSet {
            set : Set {
                set_type : SetType::Sequence,
                tiles : vec![
                    Tile::sou_tile(5),
                    Tile::sou_tile(6),
                    Tile::sou_tile(7),
                ],
            },
            call_type : CallTypes::Chii
        },
        CalledSet {
            set : Set {
                set_type : SetType::Sequence,
                tiles : vec![
                    Tile::sou_tile(3),
                    Tile::sou_tile(4),
                    Tile::sou_tile(5),
                ],
            },
            call_type : CallTypes::Ron(SetType::Sequence)
        },
    );

    winning_player.last_picked_tile = Tile::sou_tile(3);
    winning_player.winning_wait = Some(WaitType::Ryanmen);

    winning_player.ron_or_tsumo = WinningMethod::Ron(3);


    // hand gets 0 fu, but hands with 0 fu are rounded up to 30
    assert_eq!(game.players[0].hand_fu(&game, false), 30);

    // test that there's no fu points, even with a tsumo
    game.players[0].called_sets[2] = CalledSet {
            set : Set {
                set_type : SetType::Sequence,
                tiles : vec![
                    Tile::sou_tile(4),
                    Tile::sou_tile(5),
                    Tile::sou_tile(6),
                ],
            },
            call_type : CallTypes::Tsumo
    };
    game.players[0].last_picked_tile = Tile::sou_tile(6);
    game.players[0].ron_or_tsumo = WinningMethod::Tsumo;

//    TODO Detect pinfu properly, so as to correctly give no
//    assert_eq!(game.players[0].hand_fu(&game, false), 30);

    //    assert_eq!(winning_player.hand_yaku_in_han(), 1);
    // TODO: Check for han value in hand
    scoring::score_points(&mut game, Some(0));


}





#[test]
fn test_callable_tiles()
{
    {
        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
            ],
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        assert_eq!(player.callable_tiles.len(), 1);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Honor, value : SuitVal::West, red : true} ), true);

        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
            ],
            ..Player::default()
        };
    }

    {
        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile::man_tile(2),
                Tile::man_tile(3),
                Tile::man_tile(4),
                Tile::man_tile(5),
                Tile::man_tile(6),
            ],
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        assert_eq!(player.callable_tiles.len(), 9);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Honor, value : SuitVal::West, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Honor, value : SuitVal::East, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::One, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Two, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Three, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Four, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Five, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Six, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Seven, red : true} ), true);
    }

    {
        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
                Tile::man_tile(2),
                Tile::man_tile(3),
                Tile::man_tile(4),
                Tile::man_tile(5),
                Tile::man_tile(6),
            ],
            tenpai : true,
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        assert_eq!(player.callable_tiles.len(), 9);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Honor, value : SuitVal::West, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Honor, value : SuitVal::East, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::One, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Two, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Three, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Four, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Five, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Six, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Seven, red : true} ), true);
    }

    {
        let mut player = Player {
            hand : vec![
                Tile::man_tile(1), Tile::man_tile(1), Tile::man_tile(1),
                Tile::man_tile(2),
                Tile::man_tile(3),
                Tile::man_tile(4),
                Tile::man_tile(5),
                Tile::man_tile(6),
                Tile::man_tile(7),
                Tile::man_tile(8),
                Tile::man_tile(9), Tile::man_tile(9), Tile::man_tile(9),
            ],
            tenpai : true,
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        for (tile, calls) in &player.callable_tiles
        {
            println!("Callable is {}", tile)
        }

        assert_eq!(player.callable_tiles.len(), 9);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(1) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(2) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(3) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(4) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(5) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(6) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(7) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(8) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(9) ), true);
    }

    // TODO: Test this hand for tenpai detection
    {
        let mut player = Player {
            hand : vec![
                Tile::man_tile(2), Tile::man_tile(2),
                Tile::man_tile(3), Tile::man_tile(3),
                Tile::man_tile(6),
                Tile::man_tile(7), // Tile::man_tile(7), if this tile is added, then the test changes below
                Tile::man_tile(8),

                Tile::sou_tile(1), Tile::sou_tile(1), Tile::sou_tile(1),
                Tile::sou_tile(9), Tile::sou_tile(9), Tile::sou_tile(9),
            ],
            tenpai : true,
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        println!("NEWTEST");
        for (tile, calls) in &player.callable_tiles
        {
            println!("Callable is {}", tile)
        }

        println!("{:#?}", player.callable_tiles);
        assert_eq!(player.callable_tiles.len(),/*11*/ 10);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(1) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(2) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(3) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(4) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(5) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(6) ), true);
        //assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(7) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(8) ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile::man_tile(9) ), true);
    }

    {
        let mut player = Player {
            hand : vec![
                Tile::sou_tile(3),
                Tile::sou_tile(5),
                Tile::sou_tile(6),
                Tile::sou_tile(7),
                Tile::sou_tile(8),
                Tile::sou_tile(8),
                Tile::sou_tile(8),
            ],
            tenpai : true,
            ..Player::default()
        };

        player.sort_hand();
        player.update_callable_tiles();

        println!("NEWTEST");
        for (tile, calls) in &player.callable_tiles
        {
            println!("Callable is {}", tile)
        }

        let ron_call_only = Calls {
            ron : true,
            ..Calls::default()
        };

        let chii_call_only = Calls {
            chii : true,
            ..Calls::default()
        };

        let chii_pon_or_hand_kans_call = Calls {
            chii : true,
            open_kan : true,
            closed_kan : true,
            added_kan : false,
            pon : true,
            ron : false,
            ron_set : Set::invalid_default()
        };
//  TODO: This was working before, but it might not now since it's testing in a very weird way by manually setting tenpai. Refactor this
//        println!("{:#?}", player.callable_tiles);
//        assert_eq!(player.callable_tiles.len(), 7);
//
//        assert_eq!(player.callable_tiles.contains_key( &Tile::sou_tile(3) ), true);
//        assert_eq!(*player.callable_tiles.entry(Tile::sou_tile(3)).or_default(), ron_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile::sou_tile(4) ), true);
        assert_eq!(*player.callable_tiles.entry(Tile::sou_tile(4)).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile::sou_tile(5) ), true);
        assert_eq!(*player.callable_tiles.entry(Tile::sou_tile(5)).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile::sou_tile(6) ), true);
        assert_eq!(*player.callable_tiles.entry(Tile::sou_tile(6)).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile::sou_tile(7) ), true);
        assert_eq!(*player.callable_tiles.entry(Tile::sou_tile(7)).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile::sou_tile(8) ), true);
        assert_eq!(*player.callable_tiles.entry(Tile::sou_tile(8)).or_default(), chii_pon_or_hand_kans_call);
    }
}
