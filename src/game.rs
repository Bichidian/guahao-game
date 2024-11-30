use crate::action::{Action, Resource, INIT_STATE};

pub trait Play {
    fn get_action(&self, state: Resource, other_state: Resource) -> Action;
    fn send_state(&self, game_info: GameInfo);
}

pub struct Game<T: Play, U: Play> {
    state1: Resource,
    state2: Resource,
    player1: T,
    player2: U,
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

pub struct GameInfo {
    pub state: Resource,
    pub other_state: Resource,
    pub other_action: Action,
    pub outcome: RoundOutcome,
}

impl<T, U> Game<T, U>
where
    T: Play,
    U: Play,
{
    pub fn new(player1: T, player2: U) -> Self {
        Self {
            state1: INIT_STATE,
            state2: INIT_STATE,
            player1: player1,
            player2: player2,
        }
    }

    pub fn run_game(&mut self) {
        loop {
            let action1 = self.player1.get_action(self.state1, self.state2);
            let action2 = self.player2.get_action(self.state2, self.state1);
            let outcome = self.update_state(action1, action2); // From player1's perspective

            let game_info1 = GameInfo {
                state: self.state1,
                other_state: self.state2,
                other_action: action2,
                outcome: outcome,
            };
            let game_info2 = GameInfo {
                state: self.state2,
                other_state: self.state1,
                other_action: action1,
                outcome: -outcome,
            };

            self.player1.send_state(game_info1);
            self.player2.send_state(game_info2);
            if matches!(outcome, RoundOutcome::Win | RoundOutcome::Lose) {
                break;
            }
        }
    }

    fn update_state(&mut self, action1: Action, action2: Action) -> RoundOutcome {
        let cost1 = action1.get_cost();
        for (s, c) in self.state1.iter_mut().zip(cost1.into_iter()) {
            *s -= c;
            if *s < 0 {
                return RoundOutcome::Lose;
            }
        }

        let cost2 = action2.get_cost();
        for (s, c) in self.state2.iter_mut().zip(cost2.into_iter()) {
            *s -= c;
            if *s < 0 {
                return RoundOutcome::Win;
            }
        }

        if let Action::Attack(a1) = action1 {
            match action2 {
                Action::Attack(a2) if a2 < a1 => RoundOutcome::Win,
                Action::Attack(a2) if a2 == a1 => RoundOutcome::Continue,
                Action::Attack(_) /* a2 > a1 */ => RoundOutcome::Lose,
                Action::Defend(d2) if d2 == a1 => RoundOutcome::Continue,
                Action::Defend(_) => RoundOutcome::Win,
                Action::Guahao => RoundOutcome::Win,
                Action::Quanfang => RoundOutcome::Continue,
                Action::Fantan => RoundOutcome::Lose,
            }
        } else if let Action::Attack(a2) = action2 {
            match action1 {
                Action::Attack(_) => unreachable!(),
                Action::Defend(d1) if d1 == a2 => RoundOutcome::Continue,
                Action::Defend(_) => RoundOutcome::Lose,
                Action::Guahao => RoundOutcome::Lose,
                Action::Quanfang => RoundOutcome::Continue,
                Action::Fantan => RoundOutcome::Win,
            }
        } else {
            RoundOutcome::Continue
        }
    }
}
