use crate::basic::{Action, Resource};
use rand;
use rand::seq::IteratorRandom;

pub struct BotPlayer;

impl BotPlayer {
    pub fn get_action(state: Resource, other_state: Resource) -> Action {
        let mut rng = rand::thread_rng();
        let sensible_actions = Self::list_sensible_actions(state, other_state);
        sensible_actions.into_iter().choose(&mut rng).unwrap_or(Action::Guahao)
    }

    fn list_sensible_actions(state: Resource, other_state: Resource) -> Vec<Action> {
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
