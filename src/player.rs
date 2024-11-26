use crate::action::{Action, Resource};
use crate::game::Play;
use std::io;

pub struct CLIPlayer;

impl Play for CLIPlayer {
    fn get_action(&self, _state: &Resource, _other_state: &Resource) -> Action {
        println!("请输入动作");
        loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("读取命令行输入失败");
            match guess.trim().parse::<Action>() {
                Ok(action) => {
                    return action;
                }
                Err(_) => {
                    println!("请输入合法动作");
                    continue;
                }
            };
        }
    }
}

pub struct BotPlayer;

impl Play for BotPlayer {
    fn get_action(&self, state: &Resource, other_state: &Resource) -> Action {
        if state[0] == 0 {
            Action::Guahao
        } else {
            Action::Attack(1)
        }
    }
}
