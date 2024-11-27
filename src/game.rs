use crate::action::{Action, Resource};

pub trait Play {
    fn get_action(&self, state: &Resource, other_state: &Resource) -> Action;
}

pub struct Game<T: Play, U: Play> {
    state1: Resource,
    state2: Resource,
    player1: T,
    player2: U,
}

enum RoundOutcome {
    Player1Win,
    Player2Win,
    Continue,
}

impl<T, U> Game<T, U>
where
    T: Play,
    U: Play,
{
    pub fn new(player1: T, player2: U) -> Self {
        Self {
            state1: [0, 1, 1],
            state2: [0, 1, 1],
            player1: player1,
            player2: player2,
        }
    }

    pub fn run_game(&mut self) {
        loop {
            let action1 = self.player1.get_action(&self.state1, &self.state2);
            let action2 = self.player2.get_action(&self.state2, &self.state1);
            let outcome = self.update_state(&action1, &action2);
            self.broadcast(&action1, &action2);
            match outcome {
                RoundOutcome::Player1Win => println!("玩家1赢了！"),
                RoundOutcome::Player2Win => println!("玩家2赢了！"),
                RoundOutcome::Continue => continue,
            }
            break;
        }
    }

    fn update_state(&mut self, action1: &Action, action2: &Action) -> RoundOutcome {
        let cost1 = action1.get_cost();
        for (s, c) in self.state1.iter_mut().zip(cost1.iter()) {
            *s -= *c;
            if *s < 0 {
                return RoundOutcome::Player2Win;
            }
        }

        let cost2 = action2.get_cost();
        for (s, c) in self.state2.iter_mut().zip(cost2.iter()) {
            *s -= *c;
            if *s < 0 {
                return RoundOutcome::Player1Win;
            }
        }

        if let Action::Attack(a1) = action1 {
            match action2 {
                Action::Attack(a2) if a2 < a1 => RoundOutcome::Player1Win,
                Action::Attack(a2) if a2 == a1 => RoundOutcome::Continue,
                Action::Attack(_) /* a2 > a1 */ => RoundOutcome::Player2Win,
                Action::Defend(d2) if d2 == a1 => RoundOutcome::Continue,
                Action::Defend(_) => RoundOutcome::Player1Win,
                Action::Guahao => RoundOutcome::Player1Win,
                Action::Quanfang => RoundOutcome::Continue,
                Action::Fantan => RoundOutcome::Player2Win,
            }
        } else if let Action::Attack(a2) = action2 {
            match action1 {
                Action::Attack(_) => panic!(),
                Action::Defend(d1) if d1 == a2 => RoundOutcome::Continue,
                Action::Defend(_) => RoundOutcome::Player2Win,
                Action::Guahao => RoundOutcome::Player2Win,
                Action::Quanfang => RoundOutcome::Continue,
                Action::Fantan => RoundOutcome::Player1Win,
            }
        } else {
            RoundOutcome::Continue
        }
    }

    fn broadcast(&self, action1: &Action, action2: &Action) {
        println!("玩家1出招：【{}】", action1);
        println!("玩家2出招：【{}】", action2);
        println!(
            "玩家1剩余：挂号{}，全防{}，反弹{}",
            self.state1[0], self.state1[1], self.state1[2]
        );
        println!(
            "玩家2剩余：挂号{}，全防{}，反弹{}",
            self.state2[0], self.state2[1], self.state2[2]
        );
    }
}
