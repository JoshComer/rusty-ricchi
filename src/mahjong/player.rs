use std::collections::hash_map;
use std::hash::{Hash, Hasher};
use std::{fmt, slice::Windows, usize::MAX, iter::empty, collections::HashMap, };
use int_enum::IntEnum;
use num::pow;

use rand::Rng;

use crate::mahjong::tile::*;
use crate::mahjong::Game;

use crate::mahjong::tui_output;

use crate::mahjong::scoring;

use crate::mahjong::utils;

pub const NUM_PLAYERS    : usize = 4;


pub const PLAYER_HAND_SIZE : usize = 14;
const STARTING_POINTS : i32 = 25000;

#[derive(Clone, Eq, PartialEq)]
pub struct Player {
    pub hand : Vec<Tile>,
    pub called_sets : Vec<CalledSet>,

    /// Only the visible discards from this player
    /// Must be used with tiles_others_called in order to calculate furiten
    pub discard_pile : Vec<Tile>,

    pub tiles_others_called : Vec<Tile>,

    /// only used for display purposes. Not to determine if a tile can be called on or not
    pub winning_call_tiles : Vec<Tile>,

    pub callable_tiles : HashMap<Tile, Calls>,


    pub last_picked_tile : Tile,
    pub seat_wind : SuitVal,

    pub points : i32,

    pub tenpai : bool,
    pub furiten : bool,

    pub is_human : bool,

    pub riichi : bool,
    pub double_riichi : bool,
    pub iipatsu : bool,

    pub winning_wait : Option<WaitType>,
    pub ron_or_tsumo : (WinningMethod, usize), // usize contains index to player that was ron'd

    pub ai_algorithm : AIAlgorithm,

    /// starts from 1 and indicates player's position in the game's players array.
    /// This can be found by subtracting 1 from the player number since it starts from 1
    pub player_number : usize,
}

struct PlayerTileIter <'a>{
    player : &'a Player,
    pos : usize,
}

impl <'a> PlayerTileIter<'a> {
    fn construct(player : &'a Player) -> Self
    {
        return PlayerTileIter {
            player : player,
            pos : 0
        }
    }

}

impl <'a> Iterator for PlayerTileIter<'a> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item>
    {
        let total_player_tiles_len = self.player.hand.len() + self.player.revealed_sets_tiles_len();

        if self.pos < self.player.hand.len()
        {
            self.pos += 1;
            return Some(self.player.hand[self.pos - 1]);
        }
        else if self.pos < total_player_tiles_len
        {
            self.pos += 1;

            let mut revealed_pos : i32 = (self.pos - 1 - self.player.hand.len()) as i32;

            for i in 0..self.player.called_sets.len(){
                let set_len = match self.player.called_sets[i].set.set_type{
                    SetType::Pair => 2,
                    SetType::Sequence | SetType::Triplet => 3,
                    SetType::Kan => 4,
                };

                revealed_pos -= set_len;
                if revealed_pos < 0
                {
                    return Some(self.player.called_sets[i].set.tiles[(revealed_pos + set_len) as usize]);
                }
                else if revealed_pos == 0
                {
                    return Some(self.player.called_sets[i].set.tiles[0]);
                }
            }

            return None;
        }
        else {
            return None;
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        return Player {
            player_number : usize::MAX,

            hand : vec![INVALID_TILE; PLAYER_HAND_SIZE],
            last_picked_tile : INVALID_TILE,
            called_sets : Vec::new(),

            discard_pile : Vec::with_capacity(70),
            tiles_others_called : Vec::with_capacity(20),

            winning_call_tiles : Vec::new(),
            callable_tiles : HashMap::new(),

            seat_wind : SuitVal::East,
            points : STARTING_POINTS,

            tenpai : false,
            furiten : false,

            riichi : false,
            double_riichi : false,
            iipatsu : false,

            is_human : false,

            winning_wait : None,
            ron_or_tsumo : (WinningMethod::NotWonYet, 42),

            ai_algorithm : AIAlgorithm::SimpleDiscardAlwaysCall,
        };
    }
}

impl Player {
    pub fn set_number(&mut self, number : usize) -> &mut Self
    {
        self.player_number = number;
        self
    }

    pub fn ai_call(&self, discard_tile : Tile) -> Option<CalledSet>
    {
        match self.ai_algorithm {
            AIAlgorithm::DumbAsBricks => return None,
            
            AIAlgorithm::SimpleDiscardAlwaysCall => {
                let possible_calls = self.callable_tiles.get(&discard_tile).unwrap();

                if possible_calls.ron == true {
                    return Some(CalledSet {
                        call_type : CallTypes::Ron(possible_calls.ron_set.set_type),
                        set : possible_calls.ron_set.clone(),
                    })
                }
                else if possible_calls.open_kan {
                    return Some(CalledSet {
                        call_type : CallTypes::OpenKan,
                        set : Set::kan(discard_tile)
                    })
                }
                else if possible_calls.pon {
                    return Some(CalledSet {
                        call_type : CallTypes::Pon,
                        set : Set::triplet(discard_tile)
                    })
                }
                else if possible_calls.chii {
                    let chiiable_sets = get_callable_chii_combinations_with_tile(&self.hand, discard_tile);
                    return Some(chiiable_sets[0].clone());
                }
                else
                {
                    return None;
                }
            }
        }
    }

    pub fn ai_discard(&self) -> usize
    {
        match self.ai_algorithm {
            AIAlgorithm::DumbAsBricks => return 0,

            AIAlgorithm::SimpleDiscardAlwaysCall => {
                // if we decide to keep tiles, we remove them from this vector. This contains the tiles to pick from randomly to discard
                // at the end of the algorithm
                let mut hand_copy = self.hand.clone();

                // keep honors if there's two, otherwise discard
                let mut hand_honor_tiles = self.hand.clone();
                hand_honor_tiles.retain(|tile| tile.suit == Suit::Honor);

                for tile in hand_honor_tiles
                {
                    // check if there's two of them
                    if ! self.callable_tiles.contains_key(&tile)
                    {
                        return self.hand.iter().position(|hand_tile| *hand_tile == tile).unwrap();
                    }
                    else
                    {
                        hand_copy.retain(|hand_tile| *hand_tile != tile);
                    }
                }

                // keep terminals if there's two of them, or if they have the adjacent sequence number. Otherwise discard
                let mut hand_terminal_tiles = self.hand.clone();
                hand_terminal_tiles.retain(|tile| tile.value == SuitVal::One || tile.value == SuitVal::Nine);

                for tile in hand_terminal_tiles
                {
                    // check if there's two of them
                    if ! self.callable_tiles.contains_key(&tile)
                    {
                        // check if the hand contains an adjacent tile (if so, then chii-ing is an option)
                        if ! numbered_tile_has_a_neighbor(tile, &self.hand)
                        {
                            return self.hand.iter().position(|hand_tile| *hand_tile == tile).unwrap();
                        }
                    }
                    
                    hand_copy.retain(|hand_tile| *hand_tile != tile);
                }

                // remove non-terminal number tiles without a pair or neighbor
                for tile in hand_copy.clone()
                {
                    if ! self.callable_tiles.contains_key(&tile)
                    {
                        // due to previous logic, ALL tiles within hand_copy at this point aren't terminals
                        if ! numbered_tile_has_a_neighbor(tile, &self.hand)
                        {
                            return self.hand.iter().position(|hand_tile| *hand_tile == tile).unwrap();
                        }
                        else
                        {
                            hand_copy.retain(|hand_tile| *hand_tile != tile);
                        }
                    }
                }

                // discard any remaining tiles which don't have pair or neighbor
                if hand_copy.len() != 0
                {
                    return self.hand.iter().position(|hand_tile| *hand_tile == hand_copy[0]).unwrap();
                }

                // we must remove a tile with a pair or neighbor now
                // TODO: Don't discard numbers part of existing sets
                return rand::thread_rng().gen_range(0..self.hand.len());
            }
        }
    }

    pub fn dump_player_state(&self)
    {
        print!("Hand:");
        for tile in &self.hand
        {
            print!("{},", tile);
        }
        print!("\n");

        print!("Called Sets:");
        for called_set in &self.called_sets
        {
            for tile in &called_set.set.tiles
            {
                print!("{},", tile);
            }
            print!("-");
        }
        print!("\n");

        print!("Waiting on Tiles:");
        for tile in &self.winning_call_tiles
        {
            print!("{},", tile);
        }
        print!("\n");

        print!("Callable Tiles:");
        for callable in &self.callable_tiles
        {
            println!("{}:{{{:?}}}", callable.0, callable.1);
            let chiiable_sets = get_callable_chii_combinations_with_tile(&self.hand, *callable.0);
            for called_set in chiiable_sets
            {
                for tile in &called_set.set.tiles
                {
                    print!("{},", tile);
                }
                print!("-");
            }
            println!("");
        }
        print!("\n");

        print!("Tiles others called:");
        for tile in &self.tiles_others_called
        {
            print!("{},", tile);
        }
        print!("\n");

        println!("Tenpai:{} --- Furiten:{}", self.tenpai, self.furiten);

    }

    fn revealed_sets_tiles_len(&self) -> usize
    {
        let mut len = 0;

        for i in 0..self.called_sets.len()
        {
            len += self.called_sets[i].set.tiles.len();
        }

        return len;
    }


    // discard based
    // ---------------
    // chankan
    // houtei raoyui


    // discretionary
    // ---------------

    // draw based
    // ----------------
    // haitei raoyue
    // menzenchin tsumohou
    // rinshan kaihou


    // honor based
    // --------------------
    // honitsu
    // honroutou
    // shousangen
    // tanyao
    // yakuhai


    // riichi dependent
    // --------------------
    // ippatsu


    // Sequential
    // --------------------
    // Iipeikou
    // Ittsuu / Ikkitsuukan
    // Pinfu
    // Ryanpeikou
    // Sanshoku
    // sanshoku doujun


    // Terminal Based
    // ---------------------
    // chantaiyao
    // honroutou
    // junchan / Junchantaiyaochuu
    // nagashi mangan
    // tanyao


    // triplet based
    // -----------------------
    // sanankou
    // sankantsu
    // sanshoku doukou
    // shousangen
    // toitoi
    // yakuhai


    // Suit based
    // -------------------------
    // honiisou
    // Chiniisou


    // Yakuman
    // -------------------------


    // Optional/Local yaku
    // --------------------------
    // renhou
    // daisharin


    // Other
    // -------------------------


    fn has_yaku(&self) -> bool
    {
        let (num_pairs, num_sequences, num_triplets_or_quads) = self.num_pairs_sequences_and_triplets_or_quads();

        if num_pairs == 7
        {   return true;    }
        else if num_triplets_or_quads == 4
        {   return true;    }
        else if self.riichi == true || self.double_riichi == true
        {   return true;    }



        false
    }

    // fn has_winning_hand(&self) -> bool
    // {
    //     return self.has_complete_hand() && self.has_yaku();
    // }

    /// rotates the players wind counter clockwise (against intutition)
    pub fn rotate_wind(&mut self)
    {
        self.seat_wind = match self.seat_wind{
            SuitVal::East => SuitVal::North,
            SuitVal::North => SuitVal::West,
            SuitVal::West => SuitVal::South,
            SuitVal::South => SuitVal::East,
            _ => panic!("lkdsfj")
        };
    }

    // commented out due to fighting the borrow checker. Had to run through Game with an index into the player
//    fn choose_discard(&mut self, game : &mut Game) -> Tile
 //   {
  // }

    fn score_hand(&self) -> i32
    {
        0
    }

    pub fn set_is_human(&mut self) -> &mut Player
    {
        self.is_human = true;
        self
    }

    pub fn set_seat_wind(&mut self, seat_wind : SuitVal) -> &mut Player
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
    pub fn hand_num_triplets(&self) -> usize
    {
        let mut idx : usize = 0;
        let mut num_triplets : usize = 0;

        while idx < self.hand.len() - 2
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

        return num_triplets;
    }

    // greedy. If there's 3 of a tile, there's only 1 pair reported
    pub fn hand_num_pairs(&self) -> u8
    {
        let mut idx : usize = 0;
        let mut num_pairs = 0;

        while idx < self.hand.len() - 1
        {
            if self.hand[idx] == self.hand[idx + 1]
            {
                num_pairs += 1;
                idx += 2;
            }
            else
            {
                idx += 1;
            }
        }

        return num_pairs;
    }

    pub fn tiles_num_of(&self, suit : Suit, value : SuitVal) -> usize
    {
        let mut num_occurrences = 0;

        let mut iter = PlayerTileIter::construct(self);
        let mut tile = iter.next();

        while tile != None
        {
            if tile.unwrap().suit == suit && tile.unwrap().value == value
            {
                num_occurrences += 1;
            }

            tile = iter.next();
        }

        return num_occurrences;
   }

    pub fn tiles_contain(&self, suit : Suit, value : SuitVal) -> bool
    {
        let mut iter = PlayerTileIter::construct(self);
        let mut tile = iter.next();

        while tile != None
        {
            if tile.unwrap().suit == suit && tile.unwrap().value == value
            {   return true;    }

            tile = iter.next();
        }

        return false;
   }

    // excludes kazoe (yakuman from enough han)
    fn hand_yakuman_in_basic_points(&self, game : &Game) -> usize
    {
        let mut basic_points : usize = 0;

        for yakuman in scoring::YAKUMAN_FUNCS
        { // 8000 basic points per yakuman. Some functions can return double yakuman (a value of 2)
            basic_points += 8000 * yakuman(self, game);
        }

        return basic_points;
    }

    fn hand_yaku_in_han(&self) -> usize
    {
        0
    }

    fn hand_dora_in_han(&self) -> usize
    {
        0
    }

    pub fn hand_fu(&self, game : &Game, round_up : bool) -> usize
    {
        // chiitoitsu (seven pairs) is always 25 fu
        if scoring::yaku_chiitoitsu(&self, game) != 0
        {
            return 25;
        }

        let mut fu = 20;

        // add fu for revealed sets (which include closed kans)
        for set in &self.called_sets {
            let mut added_fu =  match set.call_type {
                                        CallTypes::ClosedKan => 16,
                                        CallTypes::OpenKan => 8,
                                        CallTypes::Pon => 2,
                                        CallTypes::Ron(set_type) => {
                                            match set_type {
                                                SetType::Kan => 8,
                                                SetType::Triplet => 2,
                                                _ => 0
                                            }
                                        }
                                        _ => 0,
                                    };

            if set.set.has_honor_or_terminal()
            {
                added_fu *= 2;
            }

            fu += added_fu;
        }

        // add fu for closed triplets
        let mut i = 0;
        while i < self.hand.len() - 2
        {
            let curr : Tile = self.hand[i];
            let next : Tile = self.hand[i + 1];
            let nextnext : Tile = self.hand[i + 2];

            if curr.suit == next.suit && curr.value == next.value
            {
                if next.suit == nextnext.suit && next.value == nextnext.value
                {
                    let mut added_fu = 4;

                    if next.suit == Suit::Honor || next.value == SuitVal::One || next.value == SuitVal::Nine
                    {
                        added_fu *= 2;
                    }

                    fu += added_fu;
                    i += 3;
                    continue;
                }
                else
                {
                    i += 2;
                    continue;
                }
            }
            else
            {
                i += 1;
                continue;
            }
        }

        // add fu for a pair of honor tiles which would count as yaku
        let winning_pair = self.get_one_pair();
        if let None = winning_pair
        {
            // In riichi mahjong there is ALWAYS a winning pair. Return 0 fu since the hand isn't a winning hand
            return 0;
        }
        else if let Some(winning_pair) = winning_pair
        {
            if winning_pair.tiles[0].suit == Suit::Honor
            {
                fu +=   match winning_pair.tiles[0].value {
                            SuitVal::Red => 2,
                            SuitVal::White => 2,
                            SuitVal::Green => 2,
                            _ => 0,
                        };

                if winning_pair.tiles[0].value == game.round_wind
                {
                    fu += 2;
                }

                if winning_pair.tiles[0].value == self.seat_wind
                {
                    fu += 2;
                }
            }
        }

        // add fu for winning wait
        fu +=   match self.winning_wait {
                    Some(WaitType::Ryanmen) => 0,
                    Some(WaitType::Kanchan) => 2,
                    Some(WaitType::Penchan) => 2,
                    Some(WaitType::Tanki)   => 2,
                    Some(WaitType::Shanpon) => 0,
                    None => panic!("Attempted to score fu for non winning player"),
                };

        // add fu for ron or tsumo
        fu +=   match self.ron_or_tsumo {
                    (WinningMethod::NotWonYet, _) => 0,
                    (WinningMethod::Ron, _) => {
                        if self.hand_is_closed()
                        { 10 }
                        else
                        { 0 }
                    },
                    (WinningMethod::Tsumo, _) => {
                        if scoring::yaku_pinfu(&self, game) != 0
                        { 0 }
                        else
                        { 2 }
                    },
                };


        // for pinfu
        if fu == 20
        {
            fu = 30;
        }

        // round up to 10
        if round_up{
            let last_dig = fu % 10;
            if last_dig != 0
            {
                fu += 10 - last_dig;
            }
        }

        return fu;
    }

    // way more complex than it should be imo. Way to go Japanese!
    pub fn score_hand_basic_points(&self, game : &Game) -> usize
    {
        // double yakuman, come on!
        let yakuman_pts = self.hand_yakuman_in_basic_points(game);
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
            let mut basic_points = self.hand_fu(game, true) * pow(2, 2 + han);

            // if han and fu reach over 2000 points, it's considered a 2000 point mangan
            if basic_points > 2000
            {   return 2000;            }
            else
            {   return basic_points;    }
        }

    }

    pub fn set_hand(&mut self, hand : Vec<Tile>) ->  &mut Player
    {
        self.hand = hand;
        return self;
    }

    pub fn sort_hand(&mut self) -> () {
        self.hand.sort();
    }

    fn print_player(&self) -> ()
    {
        print!("{} Player:", self.seat_wind);
        print_tiles(&self.hand, self.hand.len());
    }

    pub fn has_dragon_or_wind_yakuhai(&self, round_wind : SuitVal) -> bool
    {
        // goes until index 2, because a triplet is required. So we don't bother
        // checking the last two

        let mut i : usize = self.hand.len() - 1;
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

    fn tile_in_hand_triplet(&self, tile : Tile) -> bool
    {
        return self.hand.iter().filter(|hand_tile| **hand_tile == tile).count() == 3;
    }

    fn tile_in_hand_sequence(&self, tile : Tile) -> bool
    {
            if tile.suit == Suit::Honor
            {   return false;   }

            // return true if there's the next two in the sequence
            return self.hand.iter().find(|hand_tile| tile.value.int_value() + 1 == hand_tile.value.int_value() && tile.suit == hand_tile.suit).is_some()
            && self.hand.iter().find(|hand_tile| tile.value.int_value() + 2 == hand_tile.value.int_value() && tile.suit == hand_tile.suit).is_some();
    }


    /// updates the callable_tiles vec on player to enable chii's and pon's
    pub fn update_callable_tiles(&mut self) -> ()
    {   // TODO: ??? Maybe make it so that we don't have to recalculate every single callable tile each time
        self.callable_tiles.clear();

        // update with upgrading open triplets to open kans
        for set in &self.called_sets
        {
            match set.set.set_type {
                SetType::Triplet => self.callable_tiles.entry(set.set.tiles[0]).or_default().added_kan = true,
                _ => ()
            }
        }

        // update with calls to triplets or kans in hand
        for tile in &self.hand
        {
            let num_in_hand = self.hand.iter().filter(|hand_tile| **hand_tile == *tile).count();

            if num_in_hand == 2
            {
                self.callable_tiles.entry(*tile).or_default().pon = true;
            }
            else if num_in_hand == 3
            {
                let mut entry = self.callable_tiles.entry(*tile).or_default();
                entry.pon = true;
                entry.open_kan = true;
                entry.closed_kan = true;
            }
        }

        let mut last_numbered_tile_idx = self.hand.iter().rposition(
                |find_tile| find_tile.suit != Suit::Honor
            );


        // if there's more than one numbered tile, look through them for sequences
        if let Some(last_numbered_tile_idx) = last_numbered_tile_idx
        {
            if last_numbered_tile_idx != 0
            {
                // ignoring the first and numbered tile, because if it's part of a sequence then we'll find from the tiles next to them
                for i in 1..(last_numbered_tile_idx + 1)
                {
                    let curr_tile = self.hand[i];

                    // only checking for two tile sequences behind our current tile, because we check until the last tile
                    let prev_tile_in_sequence = self.hand.iter().find(
                        |find_tile| find_tile.value.is_before_num(curr_tile.value) && find_tile.suit == curr_tile.suit
                    );

                    // add tiles to complete the sequence from 2 tiles to 3 if there's a tile in this hand behind the current one
                    if let Some(prev_tile_in_sequence) = prev_tile_in_sequence
                    {

                        if let Some(third_tile_behind) = prev_tile_in_sequence.get_prev_num_tile()
                        {
                            self.callable_tiles.entry(third_tile_behind).or_default().chii = true;
                        }

                        if let Some(third_tile_ahead) = curr_tile.get_next_num_tile()
                        {
                            self.callable_tiles.entry(third_tile_ahead).or_default().chii = true;
                        }
                    }
                }
            }
        }

// remove this and add real ron detction in
//        // if in tenpai, look for possible pairs
//        if self.tenpai
//        {
//            // add lonely tiles
//            for tile in &self.hand
//            {
//                if self.tiles_num_of(tile.suit, tile.value) == 1 && ! self.callable_tiles.contains_key(tile)
//                {
//                    self.callable_tiles.entry(*tile).or_default().ron = true;
//                }
//            }
//        }
    }


    // a "complete hand" still needs a yakuhai to be considered a winning hand
    // and not all winning hands are "complete" hands.
    /// Returns whether a hand can win in it's current configuration
    pub fn check_complete_hand_and_update_waits(&mut self) -> bool
    {
        // needs 5 sets. 4 sequences, triplets, or kans, and 1 pair
        if self.called_sets.len() == 5
        {
            return true;
        }

        // Sometimes there's more than 1 possible hand close to winning
        // If there's more than 1 Vec<Set> returned though, they're gauranteed to be
        // the same amount of steps away from winning
        let most_hand_sets : Vec<Vec<Set>> = self.find_best_hands(&self.hand[..]);

        if most_hand_sets.len() == 0
        {
            self.tenpai = false;
            self.furiten = false;
            return false;
        }
        else
        {
            let num_sets_in_hand = most_hand_sets[0].len();

            if num_sets_in_hand + self.called_sets.len() == 5
            {
                return true;
            }
            else if num_sets_in_hand + self.called_sets.len() == 4
            {
                self.set_tenpai_true_and_update_winning_tiles(most_hand_sets);
                self.check_and_set_furiten();
                return false;
            }
            else {
                self.tenpai = false;
                self.furiten = false;
                return false;
            }
        }

    }
}

impl Player
{
    pub fn check_and_set_furiten(&mut self) -> ()
    {
        for tile in &self.winning_call_tiles
        {
            if self.discard_pile.contains(&tile)
            {
                self.furiten = true;
                return;
            }

            if self.tiles_others_called.contains(&tile)
            {
                self.furiten = true;
                return;
            }
        }

        self.furiten = false;
        return;
    }

    /// One of the most important functions. Determines the best possible groupings of tiles
    /// within a players hands. This function only returns hands which are the smallest number
    /// of "Sets" or tile groups away from winning. There can be multiple hands the same steps
    /// away though (hence the Vec<Vec<>>).
    fn find_best_hands(&self, remaining_tiles : &[Tile]) -> Vec<Vec<Set>>
    {
        let mut best_hands : Vec<Vec<Set>> = vec![];

        for i in 0..remaining_tiles.len()
        {
            // try to find a set match for this tile. If there is one, then recurse further
            // if not, then just iterate to the next option
            let total_hand = &remaining_tiles[i..]; // TODO: Remove name from this variable, and revamp how these variables are used
            let this_tile = remaining_tiles[i];
            let hand_without_this = &remaining_tiles[(i+1)..];

            let sets_possible_with_this_tile = find_possible_sets_with_tile(this_tile, hand_without_this);

            if sets_possible_with_this_tile.len() == 0
            {
                continue;
            }
            else {

                for set in &sets_possible_with_this_tile
                {
                    let mut hand_with_set_tiles_removed : Vec<Tile> = total_hand.to_vec();

                    // remove set tiles from hand
                    for tile in &set.tiles {
                        let pos = hand_with_set_tiles_removed.iter().position(
                            |check_tile| *check_tile == *tile
                        ).expect("Attempted to remove a tile from a set which should be in the hand");

                        hand_with_set_tiles_removed.remove(pos);
                    }

                    let mut a_set_removed_best_hands = self.find_best_hands(&hand_with_set_tiles_removed);

                    if a_set_removed_best_hands.len() > 0
                    {
                        for mut hand in a_set_removed_best_hands
                        {
                            hand.push(set.clone());
/*
                            // remove hands which contain more than one pair TODO: Check for 7 pairs yakuman
                            if hand.iter().filter(|set| set.set_type == SetType::Pair).count() <= 1
                            {
                                best_hands.push(hand);
                            }
*/                            
                            // if a hand contains more than one pair, we'll add the pairs towards the end later TODO: Check for 7 pairs yakuman
                            // so remove them and use the pair closest to left. Otherwise we miss the leftmost pair
                            while  hand.iter().filter(|set| set.set_type == SetType::Pair).count() > 1
                            {
                                let remove_idx = hand.iter().position(|set| set.set_type == SetType::Pair).unwrap();
                                hand.remove(remove_idx);
                            }
                            
                            best_hands.push(hand);
//                            */
                        }
                    }
                    else {
                        best_hands.push(vec![set.clone()]);
                    }
                }
            }
        }

        // remove hands if they have less possible sets than the maximum
        let mut max_hand_len = 0;
        for hand in &best_hands {
            if hand.len() > max_hand_len
            {
                max_hand_len = hand.len();
            }
        }

        best_hands.retain(|sets| sets.len() == max_hand_len);

        return best_hands;
    }

    fn check_furiten(&self) -> ()
    {
        unimplemented!();
    }

    pub fn set_tenpai_true_and_update_winning_tiles(&mut self, best_hands : Vec<Vec<Set>>) -> ()
    {
        self.tenpai = true;
//         self.furiten = self.check_furiten();
        // TODO: Prompt for riichi or double riichi

        let winning_calls = get_winning_tiles_from_tenpai_hand(&self.hand, best_hands);

        // TODO: Remove this awful logic, and only append new winning tiles probably. Might have to leave it though, idk
        self.winning_call_tiles.clear();

        for (tile, set) in winning_calls
        {
            self.winning_call_tiles.push(tile);

            let entry = self.callable_tiles.entry(tile).or_insert( Calls::default() );
            entry.ron = true;
            entry.ron_set = set;
        }
    }

    /// Takes the call made, and actually removes the tiles from players hand and moves them to the players revealed sets
    pub fn open_tiles_with_call(&mut self, discarded_tile : Tile, called_set : CalledSet)
    {
        // TODO: Sometimes the hand might have tiles left in it that are left
        // remove tiles from hand
        match called_set.set.set_type {
            SetType::Kan =>
                self.hand.retain(|hand_tile| *hand_tile != called_set.set.tiles[0]),
            SetType::Triplet => {
                for i in 0..2 {
                    let remove_idx = self.hand.iter().position(
                        |hand_tile| *hand_tile == discarded_tile
                    ).unwrap();

                    self.hand.remove(remove_idx);
                }
            },
            SetType::Pair => {
                    let remove_idx = self.hand.iter().position(
                        |hand_tile| *hand_tile == discarded_tile
                    ).unwrap();

                    self.hand.remove(remove_idx);
            }
            SetType::Sequence => {
                // Remove tiles from sequence. One tile will be missing, and We'll just skip it
                for i in 0..3
                {
                    let tile_to_remove = called_set.set.tiles[i];
                    let pos = self.hand.iter().position(
                        |hand_tile| *hand_tile == tile_to_remove
                    );

                    if let Some(pos) = pos
                    {   // guards against consuming a tile from the hand if you call chii on a tile you already have in your hand
                        if self.hand[pos] != discarded_tile
                        {
                            self.hand.remove(pos);
                        }
                    }
                }
            },
        }

        // Add the called set to player's revealed sets
        // if added kan, change the already revealed triplet to a quad
        if called_set.call_type == CallTypes::AddedKan
        {
            let mut triplet_set = self.called_sets.iter_mut().find(|set| set.set.set_type == SetType::Triplet && set.set.tiles[0] == discarded_tile).expect("Did not find revealed triplet for an added kan call");
            triplet_set.set.set_type = SetType::Kan;
            triplet_set.call_type = CallTypes::AddedKan;
            triplet_set.set.tiles.push(discarded_tile);
        }
        else
        {
            self.called_sets.push(called_set);
        }
    }

    pub fn choose_whether_to_call(self_index : usize, discarded_tile : Tile, game : &mut Game) -> Option<CalledSet>
    {
        println!("\n\n\n\n\n\n\n\ncalling choice called for {} on {}\n\n\n\n\n\n\n\n", self_index, discarded_tile);

        // TODO: DONT FORGET TO SHUFFLE AND TO UPDATE CALLABLE TILES ON THIS PLAYER IF A CALL IS ACTUALLY MADE
        if ! game.players[self_index].is_human
        {
            return game.players[self_index].ai_call(discarded_tile);
        }
        else
        {
            let mut all_possible_calls : Vec<CalledSet> = vec![];

            let possible_calls = game.players[self_index].callable_tiles.entry(discarded_tile).or_default();

            if possible_calls.pon
            {
                all_possible_calls.push(
                        CalledSet { set : Set {
                            set_type : SetType::Triplet,
                            tiles : vec![discarded_tile ; 3],
                        },
                        call_type : CallTypes::Pon,
                    }
                );
            }
            if possible_calls.open_kan
            {
                all_possible_calls.push(
                    CalledSet {
                        set : Set {
                            set_type : SetType::Kan, // closed kan happens at discard
                            tiles : vec![discarded_tile ; 4],
                        },
                    call_type : CallTypes::OpenKan,
                    });
            }
            if possible_calls.closed_kan && game.curr_player_idx == self_index
            {
                all_possible_calls.push(
                    CalledSet {
                        set : Set {
                            set_type : SetType::Kan, // closed kan happens at discard
                            tiles : vec![discarded_tile ; 4],
                        },
                    call_type : CallTypes::ClosedKan,
                    });
            }
            // added kan is only possible during drawing. Not for calling on other player's discarded tiles
            if possible_calls.chii && (game.curr_player_idx + 1) % NUM_PLAYERS == self_index
            {
                let mut chiiable_sets = get_callable_chii_combinations_with_tile(&game.players[self_index].hand, discarded_tile);
                all_possible_calls.append(&mut chiiable_sets);
            }
            if possible_calls.ron
            {
                all_possible_calls.push(
                    CalledSet {
                        set : possible_calls.ron_set.clone(),
                        call_type: CallTypes::Ron(possible_calls.ron_set.set_type)
                    });
            }

            tui_output::output_game(game, self_index);
            let call_made = tui_output::get_player_call_choice(game, self_index, discarded_tile, &mut all_possible_calls);

            return call_made;
        }
    }

    /// Looks through a hand and returns a tuple containing (pairs found, other tiles in the hand aside from the pair).
    /// If a tile pair has already been ron'd or tsumo'd into the revealed sets, then it returns early with just that pair
    /// since that will be the winning hand
    /// # Returns
    /// 1. `Vec<Tile>` : A vector of tiles which were found as pairs in the hand
    /// 2. `Vec<Vec<Tile>>` : A vector containing hands associated with each of the pairs. If the `Vec<Tile>` from 1 above
    /// from above contained a 4 of man, then the `Vec<Vec<Tile>>` here would contain all other tiles from
    /// the passed in hand with 2 of the 4 man tiles removed
    fn get_hands_with_pairs(&self) -> (Vec<Tile>, Vec<Vec<Tile>>)
    {
        let mut pairs : Vec<Tile> = vec![];
        let mut hands : Vec<Vec<Tile>> = vec![vec![]];

        // possible early return
        // if a tile pair has been tsumo'd or ron'd then the winning hand contains that pair
        for called_set in &self.called_sets
        {
            match called_set.set.set_type {
                SetType::Pair => {
                    pairs.push(called_set.set.tiles[0]);
                    hands.push(self.hand.clone());
                    return (pairs, hands);
                },
                _ => ()
            }
        }

        for tile in &self.hand
        {
            if (! pairs.contains(tile)) && self.hand.iter().filter(|vec_tile| **vec_tile == *tile).count() >= 2
            {
                pairs.push(*tile);
                hands.push(self.hand.clone());
                let mut recent_hand : &mut Vec<Tile> = hands.last_mut().unwrap();
                // remove tile twice from the hand we just added, since we already decided it was the pair. We will check for combinations of the other tiles later
                recent_hand.remove(recent_hand.iter().position(|iter_tile| *iter_tile == *tile).expect("Should never happen"));
                recent_hand.remove(recent_hand.iter().position(|iter_tile| *iter_tile == *tile).expect("Should never happen"));
            }
        }

        return (pairs, hands);
    }

    /// takes a sorted vector of tiles and returns possible completed combinations of triplets and sequences within it.
    /// It is assumed that a pair of tiles has been removed from this hand.
    /// Returns a vector of possibilites, because it's possible to have a hand that could be interpreted two ways passed
    /// into this function. For example, all numbers of the same suit 444555666777 could be all triplets or 3 sequences and a triplet.
    /// whichever scores higher is how it should be interpreted
    fn find_triplets_from_pair_hands(hand : &mut Vec<Tile>) -> Vec<Vec<Set>>
    {
        let mut ret_vec : Vec<Vec<Set>> = vec![vec![]];

        fn is_part_of_triplet(hand : &Vec<Tile>, check_tile : Tile) -> bool
        {
            return hand.iter().filter(|hand_tile| **hand_tile == check_tile).count() == 3;
        }

        // only checks up because we go left to right
        fn is_part_of_sequence(hand : &Vec<Tile>, check_tile : Tile) -> bool
        {
            if check_tile.suit == Suit::Honor
            {   return false;   }

            // return true if there's the next two in the sequence
            return hand.iter().find(|hand_tile| check_tile.value.int_value() + 1 == hand_tile.value.int_value() && check_tile.suit == hand_tile.suit).is_some()
            && hand.iter().find(|hand_tile| check_tile.value.int_value() + 2 == hand_tile.value.int_value() && check_tile.suit == hand_tile.suit).is_some();
        }

        // we can only have one of each triplet, so if we've seen a triplet before we don't check a permutation with it again in this iteration
        let mut seen_triplets_vec : Vec<Tile> = vec![];

        for tile in &*hand
        {
            if is_part_of_triplet(hand, *tile) && (! seen_triplets_vec.contains(tile))
            {
                let found_triplet = Set {
                    set_type : SetType::Triplet,
                    tiles : vec![ *tile ; 3 ],
                };

                let mut new_hand = hand.clone();
                // remove 3 of the element
                new_hand.remove(new_hand.iter().position(|hand_tile| *hand_tile == *tile).unwrap());
                new_hand.remove(new_hand.iter().position(|hand_tile| *hand_tile == *tile).unwrap());
                new_hand.remove(new_hand.iter().position(|hand_tile| *hand_tile == *tile).unwrap());

                let mut sets_vec_vec = Player::find_triplets_from_pair_hands(&mut new_hand);

                for mut set_vec in sets_vec_vec {
                    set_vec.push(found_triplet.clone());
                    ret_vec.push(set_vec);
                }

                seen_triplets_vec.push(*tile);
            }

            // not an else if, because we need to check for both possibilities
            if is_part_of_sequence(hand, *tile)
            {
                let mut new_hand = hand.clone();

                // remove and collect tiles in the sequence
                let sequence_first = new_hand.remove(new_hand.iter().position(|hand_tile| tile.value.int_value() == hand_tile.value.int_value() && tile.suit == hand_tile.suit).unwrap());
                let sequence_second = new_hand.remove(new_hand.iter().position(|hand_tile| tile.value.int_value() + 1 == hand_tile.value.int_value() && tile.suit == hand_tile.suit).unwrap());
                let sequence_third = new_hand.remove(new_hand.iter().position(|hand_tile| tile.value.int_value() + 2 == hand_tile.value.int_value() && tile.suit == hand_tile.suit).unwrap());

                let found_sequence = Set {
                    set_type : SetType::Sequence,
                    tiles : vec![sequence_first, sequence_second, sequence_third],
                };

                let mut sets_vec_vec = Player::find_triplets_from_pair_hands(&mut new_hand);

                for mut set_vec in sets_vec_vec {
                    set_vec.push(found_sequence.clone());
                    ret_vec.push(set_vec);
                }
            }


        }

        return ret_vec;
    }

    fn num_pairs_sequences_and_triplets_or_quads(&self) -> (usize, usize, usize)
    {

        return (0, 0, 0);
    }

    fn hand_is_closed(&self) -> bool
    {
        if self.called_sets.len() == 0
        {
            return true;
        }
        else
        {
            for set in &self.called_sets
            {
                // hand is still closed if it's only closed kans, or a set was created from ron
                if set.call_type != CallTypes::ClosedKan
                {
                    if let CallTypes::Ron(_) = set.call_type
                    {
                        continue; // really hackish, but there's no way to negate an if let statement
                    }
                    else {
                        return false;
                    }
                }
            }

            return true;
        }
    }

    /// returns a pair from the hand or revealed sets. Used to find the pair from winning hands for fu calculation
    /// in the case of the yakuman of all pairs, simply returns the first pair it finds
    fn get_one_pair(&self) -> Option<Set>
    {
        let mut ret_set = Set {
            set_type : SetType::Pair,
            tiles : Vec::with_capacity(2),
        };

        // look for a pair in revealed sets
        for revealed_set in &self.called_sets
        {
            if revealed_set.set.set_type == SetType::Pair
            {
                ret_set.tiles = revealed_set.set.tiles.clone();
                return Some(ret_set);
            }
        }

        // special cases
        if self.hand.len() < 2
        {
            return None;
        }

        // look for a pair in player's hand
        let mut i = 0;
        while i < self.hand.len() - 2
        {
            let curr : Tile = self.hand[i];
            let next : Tile = self.hand[i + 1];
            let nextnext : Tile = self.hand[i + 2];

            if curr.suit == next.suit && curr.value == next.value
            {
                if next.suit == nextnext.suit && next.value == nextnext.value
                {
                    i += 3;
                    continue;
                }
                else
                {
                    ret_set.tiles.push(curr);
                    ret_set.tiles.push(next);
                    return Some(ret_set);
                }
            }

            i += 1;
            continue;
        }

        let last : Tile = self.hand[self.hand.len() - 1];
        let second_last : Tile = self.hand[self.hand.len() - 2];

        if last.suit == second_last.suit && last.value == second_last.value
        {
            ret_set.tiles.push(second_last);
            ret_set.tiles.push(last);
            return Some(ret_set);
        }

        return None;
    }



}


#[derive(Clone, Eq, PartialEq)]
pub enum WinningMethod {
    NotWonYet,
    Ron,
    Tsumo
}


pub enum DiscardChoices {
    DiscardTile(usize),
    Win,
    OpenClosedKan(Tile),
    AddedKan(Tile),
}

#[derive(Clone, Eq, PartialEq)]
pub enum AIAlgorithm {
    DumbAsBricks,
    SimpleDiscardAlwaysCall,
}














































// tests


#[test]
fn test_check_complete_hand_and_update_waits()
{
    fn check_hand_wins(hand : Vec<Tile>) -> bool
    {
        let mut player = Player {
            hand,
            ..Player::default()
        };
        player.sort_hand();

        let player_can_win = player.check_complete_hand_and_update_waits();

        return player_can_win;
    }

    fn assert_hand_wins(hand : Vec<Tile>) -> ()
    {
        assert_eq!(true, check_hand_wins(hand))
    }

    fn assert_hand_loses(hand : Vec<Tile>) -> ()
    {
        assert_eq!(false, check_hand_wins(hand))
    }

    assert_hand_wins(vec![Tile { suit : Suit::Honor, value : SuitVal::East, red : false},
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false},
                Tile { suit : Suit::Honor, value : SuitVal::East, red : false},
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false},
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false},
                Tile { suit : Suit::Honor, value : SuitVal::West, red : false},
                Tile { suit : Suit::Honor, value : SuitVal::Red, red : false},
                Tile { suit : Suit::Honor, value : SuitVal::Red, red : false},
                Tile { suit : Suit::Honor, value : SuitVal::Red, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Seven, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Sou, value : SuitVal::Eight, red : false},]);


    assert_hand_wins(vec![Tile { suit : Suit::Man, value : SuitVal::One, red : false},
                Tile::man_tile( 1 ), Tile::man_tile( 1 ), Tile::man_tile( 1 ),
                Tile::man_tile( 2 ),
                Tile::man_tile( 3 ),
                Tile::man_tile( 4 ),
                Tile::man_tile( 5 ),
                Tile::man_tile( 6 ),
                Tile::man_tile( 7 ),
                Tile::man_tile( 8 ),
                Tile::man_tile( 9 ), Tile::man_tile( 9 ), Tile::man_tile( 9 ),
                ]);

    assert_hand_wins(vec![Tile { suit : Suit::Man, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Man, value : SuitVal::One, red : false},
                Tile { suit : Suit::Man, value : SuitVal::One, red : false},
                Tile { suit : Suit::Man, value : SuitVal::One, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Two, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Three, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Four, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Five, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Six, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Seven, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Eight, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false},
                Tile { suit : Suit::Man, value : SuitVal::Nine, red : false},]);




    const NUM_RAND_TESTS : usize = 100;

    // TODO: Add called sets in revealed sets, tenpai hand testing, and incorrect hand testing
    for i in 0..NUM_RAND_TESTS
    {
        let mut sets : Vec<Set> = vec![];

        for i in 0..5
        {
            let new_set = match rand::thread_rng().gen_range(0..3) {
                0 => utils::get_random_sequence(),
                1 => utils::get_random_pair_triplet_or_kan(3),
                2 => utils::get_random_pair_triplet_or_kan(4),
                _ => panic!()
            };

            sets.push(new_set);
        }

        sets.push(utils::get_random_pair_triplet_or_kan(2));

        let mut hand : Vec<Tile> = vec![];

        for set in sets {
            for tile in set.tiles {
                hand.push(tile);
            }
        }

        println!("hand is ");
        for tile in &hand {
            print!("{},", tile);
        }
        print!("\n");
        check_hand_wins(hand);
    }

    let mut player = Player {
        hand : vec![  Tile::man_tile(6), Tile::man_tile(6),
                        Tile::sou_tile(6), Tile::sou_tile(6)
                    ],
        called_sets : vec![
            CalledSet {
                call_type : CallTypes::Chii,
                set : Set::from_tiles(&vec![ Tile::man_tile(7), Tile::man_tile(8), Tile::man_tile(9)]), 
            },
            CalledSet {
                call_type : CallTypes::Chii,
                set : Set::from_tiles(&vec![ Tile::pin_tile(1), Tile::pin_tile(2), Tile::pin_tile(3)]),
            },
            CalledSet {
                call_type : CallTypes::Pon,
                set : Set::from_tiles(&vec![ Tile::pin_tile(9), Tile::pin_tile(9), Tile::pin_tile(9)]),
            }
        ],
        ..Player::default()
    };
    player.sort_hand();
    player.check_complete_hand_and_update_waits();

    println!("{:#?}", player.winning_call_tiles);
    assert_eq!(player.winning_call_tiles.contains(&Tile::man_tile(6)), true);
    assert_eq!(player.winning_call_tiles.contains(&Tile::sou_tile(6)), true);

}
/*
Hand:[M:6],[M:6],[P:6],[P:9],[P:9],[S:6],[S:6],
Called Sets:[M:7],[M:8],[M:9],-[P:1],[P:2],[P:3],-
Waiting on Tiles:
Callable Tiles:[S:6]:{Calls { chii: false, pon: true, open_kan: false, added_kan: false, closed_kan: false, ron: false, ron_set: Set { set_type: Kan, tiles: [Tile { suit: Man, value: East, red: true }, Tile { suit: Man, value: East, red: true }, Tile { suit: Man, value: East, red: true }, Tile { suit: Man, value: East, red: true }] } }}

[M:6]:{Calls { chii: false, pon: true, open_kan: false, added_kan: false, closed_kan: false, ron: false, ron_set: Set { set_type: Kan, tiles: [Tile { suit: Man, value: East, red: true }, Tile { suit: Man, value: East, red: true }, Tile { suit: Man, value: East, red: true }, Tile { suit: Man, value: East, red: true }] } }}

[P:9]:{Calls { chii: false, pon: true, open_kan: false, added_kan: false, closed_kan: false, ron: false, ron_set: Set { set_type: Kan, tiles: [Tile { suit: Man, value: East, red: true }, Tile { suit: Man, value: East, red: true }, Tile { suit: Man, value: East, red: true }, Tile { suit: Man, value: East, red: true }] } }}
*/