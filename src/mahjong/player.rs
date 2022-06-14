use std::collections::hash_map;
use std::hash::{Hash, Hasher};
use std::{fmt, slice::Windows, usize::MAX, iter::empty, collections::HashMap, };
use int_enum::IntEnum;
use num::pow;

use crate::mahjong::tile::*;
use crate::mahjong::Game;

use crate::mahjong::tui_output;

use crate::mahjong::scoring;

pub const NUM_PLAYERS    : usize = 4;


pub const PLAYER_HAND_SIZE : usize = 14;
const STARTING_POINTS : i32 = 25000;

#[derive(Clone, Eq, PartialEq)]
pub struct Player {
    pub hand : Vec<Tile>, 
    pub revealed_sets : Vec<Set>,

    /// Only the visible discards from this player
    /// Must be used with tiles_others_called in order to calculate furiten
    pub discard_pile : Vec<Tile>,

    pub tiles_others_called : Vec<Tile>,

    pub waiting_on_tiles : Vec<Tile>,
    pub callable_tiles : HashMap<Tile, Calls>,


    pub last_picked_tile : Tile,
    pub seat_wind : SuitVal,
    
    pub points : i32,

    pub tenpai : bool,

    pub is_human : bool,

    pub riichi : bool,
    pub double_riichi : bool,
    pub iipatsu : bool,

    pub winning_wait : Option<WaitType>,
    pub ron_or_tsumo : (WinningMethod, usize), // usize contains index to player that was ron'd
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

            for i in 0..self.player.revealed_sets.len(){
                let set_len = match self.player.revealed_sets[i].set_type{
                    SetType::Pair => 2,
                    SetType::Sequence | SetType::Triplet => 3,
                    SetType::ClosedKan | SetType::OpenKan => 4,
                };

                revealed_pos -= set_len;
                if revealed_pos < 0
                {
                    return Some(self.player.revealed_sets[i].tiles[(revealed_pos + set_len) as usize]);
                }
                else if revealed_pos == 0
                {
                    return Some(self.player.revealed_sets[i].tiles[0]);
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
            hand : vec![INVALID_TILE; PLAYER_HAND_SIZE],
            last_picked_tile : INVALID_TILE,
            revealed_sets : Vec::new(),

            discard_pile : Vec::with_capacity(70),
            tiles_others_called : Vec::with_capacity(20),

            waiting_on_tiles : Vec::new(),
            callable_tiles : HashMap::new(),

            seat_wind : SuitVal::East,
            points : STARTING_POINTS,

            tenpai : false,

            riichi : false,
            double_riichi : false,
            iipatsu : false,

            is_human : false,

            winning_wait : None,
            ron_or_tsumo : (WinningMethod::NotWonYet, 42),
        };
    }
}

impl Player {
    fn revealed_sets_tiles_len(&self) -> usize 
    {
        let mut len = 0;

        for i in 0..self.revealed_sets.len()
        {
            match self.revealed_sets[i].set_type
            {
                SetType::Pair => len += 2,
                SetType::Sequence | SetType::Triplet => len += 3,
                SetType::OpenKan | SetType::ClosedKan => len += 4
            }
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

    pub fn rotate_wind(&mut self)
    {
        self.seat_wind = match self.seat_wind{
            SuitVal::East => SuitVal::South,
            SuitVal::South => SuitVal::West,
            SuitVal::West => SuitVal::North,
            SuitVal::North => SuitVal::East,
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
        for set in &self.revealed_sets {
            let mut added_fu =  match set.set_type {
                                        SetType::ClosedKan => 16,        
                                        SetType::OpenKan => 8,
                                        SetType::Triplet => 2,
                                        _ => 0,
                                    };

            if set.has_honor_or_terminal()
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
    /// ONLY UPDATES WITH PAIR WAITS IF PLAYER TENPAI IS SET TO TRUE
    pub fn update_callable_tiles(&mut self) -> ()
    {   // TODO: ??? Maybe make it so that we don't have to recalculate every single callable tile each time
        self.callable_tiles.clear();

        // update with upgrading open triplets to open kans
        for set in &self.revealed_sets
        {
            match set.set_type {
                SetType::Triplet => self.callable_tiles.entry(set.tiles[0]).or_default().kan = true,
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
                entry.kan = true;
            }
        }

        // the commented code should do the same thing as the lines below. No idea why it doesn't work sometimes though
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


        // if in tenpai, look for possible pairs
        if self.tenpai
        {
            // add lonely tiles
            for tile in &self.hand
            {
                if self.tiles_num_of(tile.suit, tile.value) == 1 && ! self.callable_tiles.contains_key(tile)
                {
                    self.callable_tiles.entry(*tile).or_default().pair = true;
                }
            }
        }
    }


    // a "complete hand" still needs a yakuhai to be considered a winning hand
    // and not all winning hands are "complete" hands.
    pub fn check_complete_hand_and_update_waits(&self) -> bool 
    {
        let (tile_vec, mut hands_vec) = self.get_hands_with_pairs();

        let mut vec_vec_of_sets : Vec<Vec<Set>> = vec![];

        for hand in &mut hands_vec
        {
            for inner_hand in Player::find_triplets_from_pair_hands(hand)
            {
                vec_vec_of_sets.push(inner_hand);
            }
        }

        let mut max : usize = 0;
        for vec_set in &vec_vec_of_sets
        {
            let local_max = vec_set.len();
            if local_max > max
            {
                max = local_max;
            }
        }

        // remove the sets which 
        vec_vec_of_sets.retain(|vec_set| vec_set.len() == max);

        println!("Len of vec_vec_of_sets is {}", vec_vec_of_sets.len());
        for vec in vec_vec_of_sets{
            print!("..next has a triplet num of {}..", vec.len());
        }
        print!("\n");

//        let (pairs, sequences, triplets_and_quads) = self.num_pairs_and_triplets();

        false
    }
}

impl Player
{
    /// Takes the called tile along with the type of call and moves them to the players revealed sets
    pub fn open_tiles_with_call(&mut self, called_tile : Tile, call : CallTypes)
    {
        let mut made_set = Set {
            set_type : SetType::Pair,
            tiles : Vec::with_capacity(4),
            ron : (call == CallTypes::Ron)
        };

        made_set.tiles.push(called_tile);

        let made_set = match call {
            CallTypes::Tsumo | CallTypes::Ron => unimplemented!(),
            CallTypes::Pon => {
                // TODO: Make sure that if player has 3 of same tiles and a red one, but calls pon instead of kan that the red is taken into the call
                for i in 0..2{
                    let removed_tile = self.hand.remove(
                        self.hand.iter().position(
                            |find_tile| *find_tile == called_tile
                        ).unwrap()
                    );

                    made_set.tiles.push(removed_tile);
                }

                made_set
            }
            | CallTypes::Kan => {
                for i in 0..3{
                    let removed_tile = self.hand.remove(
                        self.hand.iter().position(
                            |find_tile| *find_tile == called_tile
                        ).unwrap()
                    );

                    made_set.tiles.push(removed_tile);
                }

                made_set
            },
            CallTypes::Chii(tile_from_hand_1, tile_from_hand_2) => {
                    self.hand.remove(
                        self.hand.iter().position(
                            |find_tile| *find_tile == tile_from_hand_1
                        ).unwrap()
                    );

                    self.hand.remove(
                        self.hand.iter().position(
                            |find_tile| *find_tile == tile_from_hand_2
                        ).unwrap()
                    );

                    made_set.tiles.push(tile_from_hand_1);
                    made_set.tiles.push(tile_from_hand_2);

                    made_set
            }
        };

        self.revealed_sets.push(made_set);
    }

    pub fn choose_whether_to_call(self_index : usize, discarded_tile : Tile, game : &mut Game) -> Option<CallTypes>
    {
        println!("\n\n\n\n\n\n\n\ncalling choice called for {} on {}\n\n\n\n\n\n\n\n", self_index, discarded_tile);

        // TODO: DONT FORGET TO SHUFFLE AND TO UPDATE CALLABLE TILES ON THIS PLAYER IF A CALL IS ACTUALLY MADE
        if ! game.players[self_index].is_human
        {
            None
        }
        else
        {
            let mut all_possible_calls : Vec<Set> = vec![];

            let possible_calls = game.players[self_index].callable_tiles.entry(discarded_tile).or_default();

            if possible_calls.pon
            {
                all_possible_calls.push(
                    Set {
                        set_type : SetType::Triplet,
                        tiles : vec![discarded_tile ; 3],
                        ron : false,
                    }
                );
            }
            if possible_calls.kan
            {
                all_possible_calls.push(
                    Set {
                        set_type : SetType::OpenKan, // closed kan happens at discard
                        tiles : vec![discarded_tile ; 4],
                        ron : false,
                    }
                );
            }
            if possible_calls.chii
            {
                let thing = get_chii_combinations_with_tile(&game.players[self_index].hand, discarded_tile);
                let thing = 0; // TODO: FINISH THIS
            }

            tui_output::output_player_perspective(game, self_index);
            tui_output::get_player_call_choice(game, self_index, discarded_tile, &mut all_possible_calls);
            None
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
        for set in &self.revealed_sets
        {
            match set.set_type {
                SetType::Pair => {
                    pairs.push(set.tiles[0]);
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
                    ron : false
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
                    ron : false
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
        if self.revealed_sets.len() == 0 
        {
            return true;
        }
        else
        {
            for set in &self.revealed_sets
            {
                // hand is still closed if it's only closed kans, or a set was created from ron
                if set.set_type != SetType::ClosedKan && set.ron == false
                {
                    return false;
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
            ron : false
        };

        // look for a pair in revealed sets
        for set in &self.revealed_sets
        {
            if set.set_type == SetType::Pair
            {
                ret_set.tiles = set.tiles.clone();
                ret_set.ron = set.ron;
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





fn get_chii_combinations_with_tile(hand : &Vec<Tile>, tile : Tile)
{

}