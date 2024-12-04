use crate::action::{Action, Resource};

pub trait Play {
    fn get_action(&self, state: Resource, other_state: Resource) -> Action;
}

#[derive(Clone, Copy)]
pub enum RoundOutcome {
    Win,
    Lose,
    Continue,
}

impl std::ops::Neg for RoundOutcome {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            RoundOutcome::Continue => RoundOutcome::Continue,
            RoundOutcome::Win => RoundOutcome::Lose,
            RoundOutcome::Lose => RoundOutcome::Win,
        }
    }
}
