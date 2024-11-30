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
    thread::spawn(move || Game::new().run_game(gui_player, BotPlayer));
    GUIApp::run_gui(state_receiver, action_sender);
}
