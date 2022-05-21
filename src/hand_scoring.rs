pub mod hand_scoring {

// three great dragons
    fn yakuman_daisangen(player : &Player) -> u8
    {
        return
          ( player.hand_contains_num_of(Suit::Honor, SuitVal::White) == 3
        &&  player.hand_contains_num_of(Suit::Honor, SuitVal::Red) == 3
        &&  player.hand_contains_num_of(Suit::Honor, SuitVal::Green) == 3 )
        as u8;
    }

    // thirteen orphans
    fn yakuman_kokushi_musou(player : &Player) -> u8
    { // TODO: Double yakuman if the wait was on the pair
        return 
          ( player.hand_contains(Suit::Man, SuitVal::One)
        &&  player.hand_contains(Suit::Man, SuitVal::Nine)
        &&  player.hand_contains(Suit::Pin, SuitVal::One)
        &&  player.hand_contains(Suit::Pin, SuitVal::Nine)
        &&  player.hand_contains(Suit::Sou, SuitVal::One)
        &&  player.hand_contains(Suit::Sou, SuitVal::Nine)
        &&  player.hand_contains(Suit::Honor, SuitVal::North)
        &&  player.hand_contains(Suit::Honor, SuitVal::East)
        &&  player.hand_contains(Suit::Honor, SuitVal::South)
        &&  player.hand_contains(Suit::Honor, SuitVal::West)
        &&  player.hand_contains(Suit::Honor, SuitVal::Red)
        &&  player.hand_contains(Suit::Honor, SuitVal::White)
        &&  player.hand_contains(Suit::Honor, SuitVal::Green)
        &&  player.hand_num_pairs() == 1 )
        as u8;
    }

    // four concealed triplets and a pair
    fn yakuman_suuankou(player : &Player) -> u8
    { // TODO: Double yakuman if tsumo on the pair
        return (player.revealed_sets.is_empty() && player.hand_num_triplets() == 4) as u8;
    }

    // three little winds and four great winds
    fn yakuman_suushiihou(player : &Player) -> u8
    {
        let num_winds =
            [   player.hand_contains_num_of(Suit::Honor, SuitVal::East),
                player.hand_contains_num_of(Suit::Honor, SuitVal::South),
                player.hand_contains_num_of(Suit::Honor, SuitVal::West),
                player.hand_contains_num_of(Suit::Honor, SuitVal::North)  ];

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
    fn yakuman_tsuuiisou(player : &Player) -> u8
    {
        // since honor tiles are always sorted to the right, we can just check
        // if the leftmost tile is an honor tile to see if they're all honors
        // returns 1 for 1 yakuman if the condition is met, otherwise 0 for no yakuman
        return (player.hand[0].suit == Suit::Honor) as u8;
    }

    // all green tiles
    fn yakuman_ryuuiisou(player : &Player) -> u8
    {
        for i in 0..PLAYER_HAND_SIZE
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
    fn yakuman_chinroutou(player : &Player) -> u8
    {
       for i in 0..PLAYER_HAND_SIZE
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
    fn yakuman_chuuren_poutou(player : &Player) -> u8
    { // TODO: Double yakuman if the last tile chosen was the extra tile

        // check for all tiles being the same suit
        for i in 1..PLAYER_HAND_SIZE
        {
            if player.hand[i].suit != player.hand[0].suit
            {
                return 0;
            }
        }

        return 
             ( player.hand.into_iter().filter(|&t| t.value == SuitVal::One).count() >= 3
            && player.hand.into_iter().filter(|&t| t.value == SuitVal::Two).count() >= 1
            && player.hand.into_iter().filter(|&t| t.value == SuitVal::Three).count() >= 1
            && player.hand.into_iter().filter(|&t| t.value == SuitVal::Four).count() >= 1
            && player.hand.into_iter().filter(|&t| t.value == SuitVal::Five).count() >= 1
            && player.hand.into_iter().filter(|&t| t.value == SuitVal::Six).count() >= 1
            && player.hand.into_iter().filter(|&t| t.value == SuitVal::Seven).count() >= 1
            && player.hand.into_iter().filter(|&t| t.value == SuitVal::Eight).count() >= 1
            && player.hand.into_iter().filter(|&t| t.value == SuitVal::Nine).count() >= 3
            ) as u8;
    }


    fn yakuman_suukantsu(player : &Player) -> u8
    {
        // checking for the 4 quads
        if player.revealed_sets.iter().filter(
            |&set| set.set_type == SetType::Quad).count() != 4
        {
            return 0;
        }

        // checking for the pair

        unimplemented!();

        return 1;
    }

    // the below two yakuman are for the dealer getting a completed hand, and the dealer
    // drawing a complete hand with his first tile
    fn yakuman_tenhou(player : &Player) -> u8
    {
        unimplemented!()
    }

    fn yakuman_chiihou(player : &Player) -> u8
    {
        unimplemented!()
    }


const YAKUMAN_FUNCS : [ &dyn Fn(&Player) -> u8 ; 8] = [
    &yakuman_daisangen,    
    &yakuman_kokushi_musou,    
    &yakuman_suuankou,
    &yakuman_suushiihou,
    &yakuman_tsuuiisou,    
    &yakuman_ryuuiisou,    
    &yakuman_chinroutou,
    &yakuman_chuuren_poutou,
 ];

}