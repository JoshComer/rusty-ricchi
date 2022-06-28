use crate::mahjong::player::*;
use crate::mahjong::*;

    pub fn yaku_chiitoitsu(player : &Player, game : &Game) -> usize
    {
        0
    }

    pub fn yaku_pinfu(player : &Player, game : &Game) -> usize
    {
        0
    }

    // three great dragons
    pub fn yakuman_daisangen(player : &Player, game : &Game) -> usize
    {
        return
        ( player.tiles_num_of(Suit::Honor, SuitVal::White) == 3
        &&  player.tiles_num_of(Suit::Honor, SuitVal::Red) == 3
        &&  player.tiles_num_of(Suit::Honor, SuitVal::Green) == 3 )
        as usize;
    }

    // thirteen orphans
    pub fn yakuman_kokushi_musou(player : &Player, game : &Game) -> usize
    { // TODO: Double yakuman if the wait was on the pair
        if  player.tiles_contain(Suit::Man, SuitVal::One)
        &&  player.tiles_contain(Suit::Man, SuitVal::Nine)
        &&  player.tiles_contain(Suit::Pin, SuitVal::One)
        &&  player.tiles_contain(Suit::Pin, SuitVal::Nine)
        &&  player.tiles_contain(Suit::Sou, SuitVal::One)
        &&  player.tiles_contain(Suit::Sou, SuitVal::Nine)
        &&  player.tiles_contain(Suit::Honor, SuitVal::North)
        &&  player.tiles_contain(Suit::Honor, SuitVal::East)
        &&  player.tiles_contain(Suit::Honor, SuitVal::South)
        &&  player.tiles_contain(Suit::Honor, SuitVal::West)
        &&  player.tiles_contain(Suit::Honor, SuitVal::Red)
        &&  player.tiles_contain(Suit::Honor, SuitVal::White)
        &&  player.tiles_contain(Suit::Honor, SuitVal::Green)
        &&  player.hand_num_pairs() == 1
        {
            // double yakuman if there was a 13 sided wait for the last tile
            if player.tiles_num_of(player.last_picked_tile.suit, player.last_picked_tile.value) == 2
            {   2   }
            else
            {   1   }
        }
        else
        {
            0
        }
    }

    // four concealed triplets and a pair
    pub fn yakuman_suuankou(player : &Player, game : &Game) -> usize
    {
        if player.called_sets.len() <= 1 && player.hand_num_triplets() == 4
        {
            // double yakuman if wait is on the pair
            if player.tiles_num_of(player.last_picked_tile.suit, player.last_picked_tile.value) == 2
            {   return 2;   }
            else
            {   return 1;   }
        }

        return 0;
    }

    // three little winds and four great winds
    pub fn yakuman_suushiihou(player : &Player, game : &Game) -> usize
    {
        let num_winds =
            [   player.tiles_num_of(Suit::Honor, SuitVal::East),
                player.tiles_num_of(Suit::Honor, SuitVal::South),
                player.tiles_num_of(Suit::Honor, SuitVal::West),
                player.tiles_num_of(Suit::Honor, SuitVal::North)  ];

        let num_wind_sets  = num_winds.into_iter().filter(|&t| t >= 3 ).count();
        let num_wind_pairs = num_winds.into_iter().filter(|&t| t == 2 ).count();

        // Four great winds - Double yakuman
        if num_wind_sets == 4
        {
            return 2;
        }
        // three little winds
        else if num_wind_sets == 3 && num_wind_pairs == 1
        {
            return 1;
        }
        else
        {
            return 0;
        }
    }

    // all honor tiles
    pub fn yakuman_tsuuiisou(player : &Player, game : &Game) -> usize
    {
        // since honor tiles are always sorted to the right, we can just check
        // if the leftmost tile is an honor tile to see if they're all honors
        // returns 1 for 1 yakuman if the condition is met, otherwise 0 for no yakuman
        return (player.hand[0].suit == Suit::Honor) as usize;
    }

    // all green tiles
    pub fn yakuman_ryuuiisou(player : &Player, game : &Game) -> usize
    {
        for i in 0..player.hand.len()
        {
            let cur : Tile = player.hand[i];

            // suit check
            if cur.suit != Suit::Sou && cur.suit != Suit::Honor
            {
                return 0;
            }

            // value check
            if cur.value != SuitVal::Two && cur.value != SuitVal::Three
                && cur.value != SuitVal::Four && cur.value != SuitVal::Six
                && cur.value != SuitVal::Eight && cur.value != SuitVal::Green
            {
                return 0;
            }
        }

        return 1;
    }

    // all terminal tiles
    pub fn yakuman_chinroutou(player : &Player, game : &Game) -> usize
    {
        for i in 0..player.hand.len()
        {
            let cur : Tile = player.hand[i];

            if cur.value != SuitVal::One && cur.value != SuitVal::Nine
            {
                return 0;
            }
        }

        return 1;
    }

    // TODO: The opened door? Forget the english translation. Full straight with extra terminals
    pub fn yakuman_chuuren_poutou(player : &Player, game : &Game) -> usize
    { // TODO: Double yakuman if the last tile chosen was the extra tile

        // check for all tiles being the same suit
        for i in 1..player.hand.len()
        {
            if player.hand[i].suit != player.hand[0].suit
            {
                return 0;
            }
        }

        return
            ( player.hand.iter().filter(|&t| t.value == SuitVal::One).count() >= 3
            && player.hand.iter().filter(|&t| t.value == SuitVal::Two).count() >= 1
            && player.hand.iter().filter(|&t| t.value == SuitVal::Three).count() >= 1
            && player.hand.iter().filter(|&t| t.value == SuitVal::Four).count() >= 1
            && player.hand.iter().filter(|&t| t.value == SuitVal::Five).count() >= 1
            && player.hand.iter().filter(|&t| t.value == SuitVal::Six).count() >= 1
            && player.hand.iter().filter(|&t| t.value == SuitVal::Seven).count() >= 1
            && player.hand.iter().filter(|&t| t.value == SuitVal::Eight).count() >= 1
            && player.hand.iter().filter(|&t| t.value == SuitVal::Nine).count() >= 3
            ) as usize;
    }


    pub fn yakuman_suukantsu(player : &Player, game : &Game) -> usize
    {
        // checking for the 4 quads
        if player.called_sets.iter().filter(
            |&set| set.set.set_type == SetType::Kan).count() != 4
        {
            return 0;
        }

        // checking for the pair
        // since the 4 quads have to be revealed, the pair could be the only tiles in the hand, or also revealed
        if player.hand_num_pairs() == 1
        || player.called_sets.iter().filter(
            |&set| set.set.set_type == SetType::Pair).count() == 1
        {
            return 1;
        }

        return 0;
    }

    // the below two yakuman are for the dealer getting a completed hand, and the dealer
    // drawing a complete hand with his first tile
    // Dealer has completed hand on draw
    pub fn yakuman_tenhou(player : &Player, game : &Game) -> usize
    {
        if player.seat_wind == SuitVal::East
        && player.called_sets.len() == 0
        && game.next_tile == 0
        {
            return 1;
        }

        return 0;
    }

    // dealer completes hand with first draw
    pub fn yakuman_chiihou(player : &Player, game : &Game) -> usize
    {
        // TODO: If any tile call made by any player, does it interrupt this? Or only if this player has called? For now, assuming any player
        // also assuming that calling a closed kan eliminates possibility of this hand
        if game.next_tile < NUM_PLAYERS
        && game.next_tile != 0
        && game.num_called_tiles == 0
        && player.seat_wind == SuitVal::East
        && player.called_sets.len() == 0
        {
            return 1;
        }

        return 0;
    }


pub const YAKUMAN_FUNCS : [ &dyn Fn(&Player, &Game) -> usize ; 11] = [
    &yakuman_daisangen,
    &yakuman_kokushi_musou,
    &yakuman_suuankou,
    &yakuman_suushiihou,
    &yakuman_tsuuiisou,
    &yakuman_ryuuiisou,
    &yakuman_chinroutou,
    &yakuman_chuuren_poutou,
    &yakuman_suukantsu,
    &yakuman_tenhou,
    &yakuman_chiihou,
];

enum YakuType {
// closed only and 1 han
    Riichi,
    Ippatsu,
    MenzenchinTsumohou,
    Pinfu,
    Iipeikou,
// 1 han
    HaiteiRaoyue,
    HouteiRaoyui,
    RinshanKaihou,
    Chankan,
    Tanyao,
    Yakuhai,
// 2 han
    DoubleRiichi,
    Chantaiyao,
    SanshokuDoujun,
    Ikkitsuukan,
    Toitoi,
    Sanankou,
    SanshokuDoukou,
    Sankantsu,
    Chiitoitsu,
    Honroutou,
    Shousangen,
// 3 han
    Honitsu,
    Junchantaiyao,
    Ryanpeikou,
// 6 han
    Chinitsu,

// YAKUMAN
    Kazoe,
    KokushiMusou,
    Daisangen,
    Suuankou,
    Shousuushi,
    Daisuushi,
    Tsuuiisou,
    Ryuuiisou,
    Chinroutou,
    ChuurenPoutou,
    Suukantsu,
    Tenhou,
    Chiihou,
    NagashiMangan,
    Renhou,
    Daisharin,
}

struct HandScore {
    basic_points : usize,
    fu : usize,

    calculated_point_dif : i32,

    hand_yaku : Vec<YakuType>,
}


pub fn score_points(game : &mut Game, winning_player_idx : Option<usize>) -> ()
    {
        if let Some(winning_player_idx) = winning_player_idx
        {
            if game.players[winning_player_idx].is_human
            {
                tui_output::output_player_win_or_lose(&game.players[winning_player_idx], game.human_is_playing);
            }
            else {
                tui_output::output_player_win_or_lose(&game.players[winning_player_idx], game.human_is_playing);
            }
        }

        const EXHAUSTIVE_DRAW_POINTS : i32 = 3000;
        match winning_player_idx
        {
            None => {
                let num_tenpai_players = game.players.iter().filter(|player| player.tenpai).count();

                // no one or everyone is in tenpai
                if num_tenpai_players == 0 || num_tenpai_players == 4
                {  return;  }

                for player in &mut game.players {
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
                if ! game.players.iter().find(|player| player.seat_wind == SuitVal::East).unwrap().tenpai
                {
                    for player in &mut game.players { player.rotate_wind(); }
                }
            }
            Some(winning_player_idx) => {

                let basic_points = game.players[winning_player_idx].score_hand_basic_points(game);

                let winners_seat_wind : SuitVal = game.players[winning_player_idx].seat_wind;

                let winning_player = &mut game.players[winning_player_idx];
                let other_players = game.players.iter().filter(|player| player.seat_wind != game.players[winning_player_idx].seat_wind);

                match game.players[winning_player_idx].ron_or_tsumo {
                    WinningMethod::Ron(victim_index) =>  // "victim" is the player who got ron called on them
                        if game.players[winning_player_idx].seat_wind == SuitVal::East
                        {
                            game.players[victim_index].points -= round_up_to_100((basic_points * 6) as i32);
                            game.players[winning_player_idx].points += round_up_to_100((basic_points * 6) as i32);
                        }
                        else
                        {
                            game.players[victim_index].points -= round_up_to_100((basic_points * 4) as i32);
                            game.players[winning_player_idx].points += round_up_to_100((basic_points * 4) as i32);
                        },
                    WinningMethod::Tsumo =>
                        if game.players[winning_player_idx].seat_wind == SuitVal::East
                        {
                            for player in &mut game.players{
                                if player.seat_wind != winners_seat_wind
                                {
                                    player.points -= round_up_to_100((basic_points * 2) as i32);
                                }
                            }

                            game.players[winning_player_idx].points += round_up_to_100((basic_points * 2) as i32) * (NUM_PLAYERS -1) as i32;
                        }
                        else
                        {
                            for player in &mut game.players {
                                if player.seat_wind == SuitVal::East
                                {
                                    player.points -= round_up_to_100((basic_points * 2) as i32);
                                }
                                else if player.seat_wind != winners_seat_wind
                                {
                                    player.points  -= round_up_to_100(basic_points as i32);

                                }

                            }

                            game.players[winning_player_idx].points += round_up_to_100((basic_points * 2) as i32);
                            game.players[winning_player_idx].points += round_up_to_100((basic_points * 2) as i32) * 2;

                        },
                    _ => panic!("Player won, but did not have ron or tsumo set"),
                }
            }
        }
    }