use crate::mahjong::tile::Tile;
use crate::mahjong::tile::Set;

use super::Game;
use super::player::NUM_PLAYERS;


struct ScoreRecord {
    // TODO: Store the yaku/yakuman scored by winning player
    player_scores : [i32; NUM_PLAYERS],
    change_in_score : [i32; NUM_PLAYERS],
}


enum CommandType {
    // Game actions

    /// Stores the game state for the start of a hand
    HandSetup(Game),
    /// Stores the game state at the end of a hand
    FinalState(Game),
    /// Stores scoring for players
    Score(ScoreRecord),

    WinningPlayer,

    Discard(Tile),
    
    // call actions
    Chii(Set),
    Pon(Set),
    OpenKan(Set),
    ClosedKan(Set),
    Ron(Set),
    Tsumo(Set),


}

enum GameOrPlayer {
    Game,
    Player(usize)
}

struct Command {
    action : CommandType,
    game_or_player : GameOrPlayer,

}






