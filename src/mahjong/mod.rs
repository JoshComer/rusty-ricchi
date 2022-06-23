//pub mod game_structs {
use strum::IntoEnumIterator;
use rand::{Rng, rngs::adapter::ReseedingRng};
use unicode_segmentation::UnicodeSegmentation;
use num::{pow, bigint::ParseBigIntError, One};

const DEBUG_OUTPUT : bool = true;

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

const NUM_GAME_TILES : usize = 136;

#[allow(dead_code)]
pub struct Game {
    tiles : [Tile; NUM_GAME_TILES],
    pub next_tile : usize,

    dora_idx : usize,
    ura_dora_idx : usize,

    pub num_called_tiles : usize,

    curr_player_idx : usize,
    players : [Player; NUM_PLAYERS],

    round_wind : SuitVal,
}

impl Default for Game
{
    fn default() -> Self {

        let mut new_game = Game {
            round_wind : SuitVal::East,

            num_called_tiles : 0,

            dora_idx : NUM_GAME_TILES - 14,
            ura_dora_idx : NUM_GAME_TILES - 7,

            curr_player_idx : usize::MAX,

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
                Player::default().set_is_human().to_owned(),
                Player::default().set_seat_wind(SuitVal::South).to_owned(),
                Player::default().set_seat_wind(SuitVal::West).to_owned(),
                Player::default().set_seat_wind(SuitVal::North).to_owned(),
            ],
        };


        return new_game;
    }
}

impl Game {
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
            if self.players[i] != self.players[self.curr_player_idx]
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
    fn execute_call_or_advance_player(&mut self, discarded_tile : Tile) -> ()
    {
        let mut calls_made : Vec<(usize, CalledSet)> = self.get_calls_on_discard(discarded_tile);

        // multiple calls can be made simultaneously. Higher precedence gets to call, and multiple
        // people can ron at the same time too
        if calls_made.len() > 1
        {
            // remove the discarded tile from the discarder's pile
            self.players[self.curr_player_idx].discard_pile.pop();

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
                // switch to the player who made the call
                self.curr_player_idx = call.0;
            }
        }
        else if calls_made.len() == 1
        {
             // remove the discarded tile from the discarder's pile
            self.players[self.curr_player_idx].discard_pile.pop();

            let call = &calls_made[0];
            self.players[call.0].open_tiles_with_call(discarded_tile, call.1.clone());
            self.curr_player_idx = call.0;
        }
        else
        {
            self.curr_player_idx = (self.curr_player_idx + 1) % NUM_PLAYERS;
        }
    }

    fn player_choose_discard_or_win(&mut self, player_idx : usize) -> Option<Tile>
    {
        let mut discard_idx : usize;

        // make a choice on winning or which tile to discard
        if self.players[player_idx].is_human
        {
            // TODO: Maybe move this part to the tui function? Haven't added in win condition output functionality yet
            let player_current_hand = self.players[player_idx].hand.clone();
            self.players[player_idx].sort_hand();
            let player_can_win : bool = self.players[player_idx].check_complete_hand_and_update_waits();
            self.players[player_idx].hand = player_current_hand; // checking for a complete hand requires it be sorted
                                                                // but we want the newest drawn tile to be shown to the right for discarding purposes

            tui_output::output_player_perspective(self, player_idx);
            let discard_choice = tui_output::get_player_discard_idx(self, player_idx, player_can_win, false);

            match discard_choice {
                DiscardChoices::DiscardTile(idx) => discard_idx = idx,
                DiscardChoices::Win => return None,
                DiscardChoices::OpenClosedKan => unimplemented!(),
            }
        }
        // computer picks which to discard
        else
        {
            tui_output::output_player_perspective(self, 0);
            let mut input = String::from("");
            std::io::stdin().read_line(&mut input).expect("stdin readline failed");
            discard_idx = 0;
        }

        println!("Player number {} discarded tile {}. Deck marker is {}", player_idx, discard_idx, self.next_tile);
        let discarded_tile = self.players[player_idx].hand.remove(discard_idx);
        self.players[player_idx].discard_pile.push(discarded_tile);
        self.players[player_idx].sort_hand();
        self.players[player_idx].update_callable_tiles();

        Some(discarded_tile)
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

    fn score_points_and_advance_dealer(&mut self, winning_player_idx : Option<usize>) -> ()
    {
        const EXHAUSTIVE_DRAW_POINTS : i32 = 3000;
        match winning_player_idx
        {
            None => {
                let num_tenpai_players = self.players.iter().filter(|player| player.tenpai).count();

                // no one or everyone is in tenpai
                if num_tenpai_players == 0 || num_tenpai_players == 4
                {  return;  }

                for player in &mut self.players {
                    if player.tenpai
                    {
                        player.points += utils::round_up_to_100(EXHAUSTIVE_DRAW_POINTS / (num_tenpai_players as i32));
                    }
                    else
                    {
                        player.points -= utils::round_up_to_100(EXHAUSTIVE_DRAW_POINTS / ((NUM_PLAYERS - num_tenpai_players) as i32));
                    }
                }

                // rotate seats if dealer wasn't in tenpai and someone else was
                if ! self.players.iter().find(|player| player.seat_wind == SuitVal::East).unwrap().tenpai
                {
                    for player in &mut self.players { player.rotate_wind(); }
                }
            }
            Some(winning_player_idx) => {

                let basic_points = self.players[winning_player_idx].score_hand_basic_points(self);

                let winners_seat_wind : SuitVal = self.players[winning_player_idx].seat_wind;

                let winning_player = &mut self.players[winning_player_idx];
                let other_players = self.players.iter().filter(|player| player.seat_wind != self.players[winning_player_idx].seat_wind);

                match self.players[winning_player_idx].ron_or_tsumo {
                    (WinningMethod::Ron, victim_index) =>  // "victim" is the player who got ron called on them
                    if self.players[winning_player_idx].seat_wind == SuitVal::East
                    {
                        self.players[victim_index].points -= round_up_to_100((basic_points * 6) as i32);
                        self.players[winning_player_idx].points += round_up_to_100((basic_points * 6) as i32);
                    }
                    else
                    {
                        self.players[victim_index].points -= round_up_to_100((basic_points * 4) as i32);
                        self.players[winning_player_idx].points += round_up_to_100((basic_points * 4) as i32);
                    },
                    (WinningMethod::Tsumo, _) =>
                    if self.players[winning_player_idx].seat_wind == SuitVal::East
                    {
                        for player in &mut self.players{
                            if player.seat_wind != winners_seat_wind
                            {
                                player.points -= round_up_to_100((basic_points * 2) as i32);
                            }
                        }

                        self.players[winning_player_idx].points += round_up_to_100((basic_points * 2) as i32) * (NUM_PLAYERS -1) as i32;
                    }
                    else
                    {
                        for player in &mut self.players {
                            if player.seat_wind == SuitVal::East
                            {
                                player.points -= round_up_to_100((basic_points * 2) as i32);
                            }
                            else if player.seat_wind != winners_seat_wind
                            {
                                player.points  -= round_up_to_100(basic_points as i32);

                            }

                        }

                        self.players[winning_player_idx].points += round_up_to_100((basic_points * 2) as i32);
                        self.players[winning_player_idx].points += round_up_to_100((basic_points * 2) as i32) * 2;

                    },
                    (_,_) => panic!("Player won, but did not have ron or tsumo set"),
                }

                // rotate seat positions if needed
                if self.players[winning_player_idx].seat_wind != SuitVal::East
                {
                    for player in &mut self.players
                    {
                        player.rotate_wind();
                    }
                }
            }
        }
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
        }
    }


    fn play_hand(&mut self) -> ()
    {
        self.setup_for_hand();

        // Dealer is the east wind player
        self.curr_player_idx = self.players.iter()
            .position(|player| player.seat_wind == SuitVal::East)
            .expect("There was no player with East Wind who could be the dealer");

        loop
        {
                //draw the next tile or exhaustive draw
                let next_tile = self.draw_next_tile();
                if next_tile.is_none()
                {
                    self.score_points_and_advance_dealer(None);
                    break;
                }

                let next_tile = unsafe {next_tile.unwrap_unchecked()};

                // commented out due to fighting the borrow checker
                //  let curr_player = &mut self.players[curr_player_idx];

                // push the next tile without sorting to keep the tile on the right for display purposes
                // since after discarding
                self.players[self.curr_player_idx].hand.push(next_tile);
                let discarded_tile = self.player_choose_discard_or_win(self.curr_player_idx);
                self.players[self.curr_player_idx].sort_hand();

                // A player always discards, unless they chose to win
                if discarded_tile.is_none()
                {
                    self.score_points_and_advance_dealer(Some(self.curr_player_idx));
                    break;
                }

                let discarded_tile = unsafe { discarded_tile.unwrap_unchecked() };

                self.execute_call_or_advance_player(discarded_tile);

                //TODO: Allow other players to chii, pon, and ron from here

        };
    }

    fn play_round(&mut self) -> ()
    {
        const HANDS_PER_ROUND : u8 = 4;

        for i in 0..HANDS_PER_ROUND
        {

            self.play_hand();

            // change player seat winds, and which player is dealer
            // player seat winds change clockwise while round winds change counter clockwise (Weird)
            for player in self.players.iter_mut()
            {
                player.seat_wind = match player.seat_wind
                {
                    SuitVal::East => SuitVal::North,
                    SuitVal::North => SuitVal::West,
                    SuitVal::West => SuitVal::South,
                    SuitVal::South => SuitVal::East,
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













































// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------
//                                         Tests
// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------


#[test]
fn test_scoring()
{
    let mut game : Game = Game::default();

    game.players[0].ron_or_tsumo = (WinningMethod::Ron, 1);
    game.score_points_and_advance_dealer(Some(0));




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
        Tile { suit : Suit::Man, value : SuitVal::One, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::One, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::One, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false }, // the duplicate for pair. Set as the last tile drawn, so it should be double yakuman
    );

    game.players[0].sort_hand();

    game.players[0].ron_or_tsumo = (WinningMethod::Tsumo, 0);
    game.players[0].last_picked_tile = Tile { suit : Suit::Honor, value : SuitVal::East, red : false };
    game.next_tile = 1;

    assert_eq!(scoring::yakuman_kokushi_musou(&game.players[0], &game), 2);
    assert_eq!(scoring::yakuman_chiihou(&game.players[0], &game), 1);
    assert_eq!(scoring::yakuman_daisangen(&game.players[0], &game), 0);

    game.score_points_and_advance_dealer(Some(0));

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
        Tile { suit : Suit::Pin, value : SuitVal::Seven, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Seven, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Seven, red : false },
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

    game.players[0].ron_or_tsumo = (WinningMethod::Ron, 2);
    game.players[0].last_picked_tile = Tile { suit : Suit::Honor, value : SuitVal::Green, red : false };
    game.next_tile = 46;

    assert_eq!(yakuman_kokushi_musou(&game.players[0], &game), 0);
    assert_eq!(yakuman_chiihou(&game.players[0], &game), 0);
    assert_eq!(yakuman_daisangen(&game.players[0], &game), 1);

    game.score_points_and_advance_dealer(Some(0));

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
        Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Pin, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
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

    game.players[0].last_picked_tile = Tile { suit : Suit::Man, value : SuitVal::Seven, red : false };
    game.players[0].ron_or_tsumo = (WinningMethod::Ron , 3);
    game.next_tile = 45;


    assert_eq!(yakuman_suuankou(&game.players[0], &game), 2);

    game.score_points_and_advance_dealer(Some(0));

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
        Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Five, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::South, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::South, red : false },
    );

    winning_player.sort_hand();

    winning_player.called_sets = vec!(
        CalledSet {
            set : Set {
            set_type : SetType::Kan,
            tiles : vec![
                Tile { suit : Suit::Man, value : SuitVal::One, red : false } ; 4
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

    winning_player.ron_or_tsumo = (WinningMethod::Ron, 0);

    // test without rounding to ensure fu is correct
    assert_eq!(game.players[1].hand_fu(&game, false), 102);
    //    assert_eq!(winning_player.hand_yaku_in_han(), 1);
    // TODO: Check for han value in hand
    game.score_points_and_advance_dealer(Some(1));


}



#[test]
fn test_fu_2()
{
    let mut game = Game::default();

    assert_eq!(game.players[0].seat_wind, SuitVal::East);

    let mut winning_player = &mut game.players[0];

    winning_player.hand = vec!(
        Tile { suit : Suit::Sou, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Three, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Four, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
        Tile { suit : Suit::Honor, value : SuitVal::East, red : false },
    );

    winning_player.sort_hand();

    winning_player.called_sets = vec!(
        CalledSet {
            set : Set {
            set_type : SetType::Kan,
            tiles : vec![
                Tile { suit : Suit::Pin, value : SuitVal::One, red : false } ; 4
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
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false } ; 4
            ],
            },
            call_type : CallTypes::OpenKan
        },
    );

    winning_player.last_picked_tile = Tile { suit : Suit::Honor, value : SuitVal::East, red : false };
    winning_player.winning_wait = Some(WaitType::Tanki);

    winning_player.ron_or_tsumo = (WinningMethod::Tsumo, 0);


    // test without rounding to ensure fu is correct
    assert_eq!(game.players[0].hand_fu(&game, false), 108);
    //    assert_eq!(winning_player.hand_yaku_in_han(), 1);
    // TODO: Check for han value in hand
    game.score_points_and_advance_dealer(Some(0));


}

#[test]
fn test_fu_open_pinfu()
{
    let mut game = Game::default();

    assert_eq!(game.players[0].seat_wind, SuitVal::East);

    let mut winning_player = &mut game.players[0];

    winning_player.hand = vec!(
        Tile { suit : Suit::Man, value : SuitVal::One, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Two, red : false },
        Tile { suit : Suit::Sou, value : SuitVal::Two, red : false },
    );

    winning_player.called_sets = vec!(
        CalledSet {
            set : Set {
                set_type : SetType::Sequence,
                tiles : vec![
                    Tile { suit : Suit::Pin, value : SuitVal::Two, red : false },
                    Tile { suit : Suit::Pin, value : SuitVal::Three, red : false },
                    Tile { suit : Suit::Pin, value : SuitVal::Four, red : false },
                ],
            },
            call_type : CallTypes::Chii
        },
        CalledSet {
            set : Set {
                set_type : SetType::Sequence,
                tiles : vec![
                    Tile { suit : Suit::Sou, value : SuitVal::Five, red : false },
                    Tile { suit : Suit::Sou, value : SuitVal::Six, red : false },
                    Tile { suit : Suit::Sou, value : SuitVal::Seven, red : false },
                ],
            },
            call_type : CallTypes::Chii
        },
        CalledSet {
            set : Set {
                set_type : SetType::Sequence,
                tiles : vec![
                    Tile { suit : Suit::Sou, value : SuitVal::Three, red : false },
                    Tile { suit : Suit::Sou, value : SuitVal::Four, red : false },
                    Tile { suit : Suit::Sou, value : SuitVal::Five, red : false },
                ],
            },
            call_type : CallTypes::Ron(SetType::Sequence)
        },
    );

    winning_player.last_picked_tile = Tile { suit : Suit::Sou, value : SuitVal::Three, red : false };
    winning_player.winning_wait = Some(WaitType::Ryanmen);

    winning_player.ron_or_tsumo = (WinningMethod::Ron, 3);


    // hand gets 0 fu, but hands with 0 fu are rounded up to 30
    assert_eq!(game.players[0].hand_fu(&game, false), 30);

    // test that there's no fu points, even with a tsumo
    game.players[0].called_sets[2] = CalledSet {
            set : Set {
                set_type : SetType::Sequence,
                tiles : vec![
                    Tile { suit : Suit::Sou, value : SuitVal::Four, red : false },
                    Tile { suit : Suit::Sou, value : SuitVal::Five, red : false },
                    Tile { suit : Suit::Sou, value : SuitVal::Six, red : false },
                ],
            },
            call_type : CallTypes::Tsumo
    };
    game.players[0].last_picked_tile = Tile { suit : Suit::Sou, value : SuitVal::Six, red : false };
    game.players[0].ron_or_tsumo = (WinningMethod::Tsumo, 0);

//    TODO Detect pinfu properly, so as to correctly give no
//    assert_eq!(game.players[0].hand_fu(&game, false), 30);

    //    assert_eq!(winning_player.hand_yaku_in_han(), 1);
    // TODO: Check for han value in hand
    game.score_points_and_advance_dealer(Some(0));


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
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Five, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
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
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Five, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
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
                Tile { suit : Suit::Man, value : SuitVal::One, red : false },
                Tile { suit : Suit::Man, value : SuitVal::One, red : false },
                Tile { suit : Suit::Man, value : SuitVal::One, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Four, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Five, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Seven, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Eight, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false },
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
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Eight, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Nine, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::One, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Two, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Three, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Four, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Five, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Six, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Seven, red : true} ), true);
    }

    // TODO: Test this hand for tenpai detection
    {
        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Seven, red : false },
                Tile { suit : Suit::Man, value : SuitVal::Eight, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::One, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::One, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::One, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Nine, red : false },
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

        assert_eq!(player.callable_tiles.len(), 11);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::One, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Two, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Three, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Four, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Five, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Six, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Seven, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Eight, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Man, value : SuitVal::Nine, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Sou, value : SuitVal::One, red : true} ), true);
        assert_eq!(player.callable_tiles.contains_key( &Tile { suit : Suit::Sou, value : SuitVal::Nine, red : true} ), true);
    }

    {
        let sou_tile = Tile {
            suit : Suit::Sou,
            value : SuitVal::East,
            red : true
        };

        let mut player = Player {
            hand : vec![
                Tile { suit : Suit::Sou, value : SuitVal::Three, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Five, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Six, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Seven, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false },
                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false },
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

        let chii_pon_or_kan_call = Calls {
            chii : true,
            kan : true,
            pon : true,
            ron : false,
            ron_set : Set::invalid_default()
        };

        assert_eq!(player.callable_tiles.len(), 7);

        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Three, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Three, ..sou_tile }).or_default(), ron_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Four, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Four, ..sou_tile}).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Five, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Five, ..sou_tile }).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Six, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Six, ..sou_tile}).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Seven, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Seven, ..sou_tile}).or_default(), chii_call_only);

        assert_eq!(player.callable_tiles.contains_key( &Tile { value : SuitVal::Eight, ..sou_tile} ), true);
        assert_eq!(*player.callable_tiles.entry(Tile { value : SuitVal::Eight, ..sou_tile}).or_default(), chii_pon_or_kan_call);
    }
}


// TODO: This is the next thing
pub enum DiscardChoices {
    DiscardTile(usize),
    Win,
    OpenClosedKan,
}
