mod action;
mod game;
mod player;

use game::Game;
use player::{BotPlayer, CLIPlayer};

fn main() {
    let cli_player = CLIPlayer;
    let bot_player = BotPlayer;
    let mut game = Game::new(cli_player, bot_player);
    game.run_game();
}
