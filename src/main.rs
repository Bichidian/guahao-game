mod action;
mod game;
mod gui;
mod player;

use game::Game;
use gui::GUIApp;
use player::{BotPlayer, GUIPlayer};
use std::thread;

fn main() {
    let (gui_player, state_receiver, action_sender) = GUIPlayer::new();
    let mut game = Game::new(gui_player, BotPlayer);
    thread::spawn(move || game.run_game());
    GUIApp::run_gui(state_receiver, action_sender);
}
