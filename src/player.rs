use crate::action::{Action, Resource};
use crate::game::{GameInfo, Play, RoundOutcome};
use rand;
use rand::seq::IteratorRandom;
use std::io;
use std::sync::mpsc;

pub struct GUIPlayer {
    state_sender: mpsc::Sender<GameInfo>,
    action_receiver: mpsc::Receiver<Action>,
}

impl Play for GUIPlayer {
    fn get_action(&self, _state: Resource, _other_state: Resource) -> Action {
        self.action_receiver.recv().unwrap_or(Action::Guahao)
    }

    fn send_state(&self, game_info: GameInfo) {
        self.state_sender
            .send(game_info)
            .unwrap_or_else(|_| eprintln!("玩家离线"));
    }
}

impl GUIPlayer {
    pub fn new() -> (Self, mpsc::Receiver<GameInfo>, mpsc::Sender<Action>) {
        let (state_sender, state_receiver) = mpsc::channel::<GameInfo>();
        let (action_sender, action_receiver) = mpsc::channel::<Action>();
        let gui_player = Self {
            state_sender,
            action_receiver,
        };
        (gui_player, state_receiver, action_sender)
    }
}

pub struct CLIPlayer;

impl Play for CLIPlayer {
    fn get_action(&self, _state: Resource, _other_state: Resource) -> Action {
        println!("请输入动作");
        loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("读取命令行输入失败");
            match guess.trim().parse::<Action>() {
                Ok(action) => {
                    println!("我方出招：【{}】", action);
                    return action;
                }
                Err(_) => println!("请输入合法动作"),
            };
        }
    }

    fn send_state(&self, game_info: GameInfo) {
        println!("对方出招：【{}】", game_info.other_action);
        println!("我方剩余：{}", game_info.state);
        println!("对方剩余：{}", game_info.other_state);
        match game_info.outcome {
            RoundOutcome::Win => println!("您赢了"),
            RoundOutcome::Lose => println!("您输了"),
            RoundOutcome::Continue => {}
        }
    }
}

pub struct BotPlayer;

impl Play for BotPlayer {
    fn get_action(&self, state: Resource, other_state: Resource) -> Action {
        let mut rng = rand::thread_rng();
        let sensible_actions = self.list_sensible_actions(state, other_state);
        sensible_actions.into_iter().choose(&mut rng).unwrap_or(Action::Guahao)
    }

    fn send_state(&self, _game_info: GameInfo) {}
}

impl BotPlayer {
    fn list_sensible_actions(&self, state: Resource, other_state: Resource) -> Vec<Action> {
        let mut sensible_actions = vec![Action::Guahao];
        for a in 1..=state[0] {
            sensible_actions.push(Action::Attack(a as u8));
        }
        for d in 1..=other_state[0] {
            sensible_actions.push(Action::Defend(d as u8));
        }
        if other_state[0] > 1 && state[1] > 0 {
            sensible_actions.push(Action::Quanfang);
        }
        if other_state[0] > 0 && state[2] > 0 {
            sensible_actions.push(Action::Fantan);
        }
        sensible_actions
    }
}
