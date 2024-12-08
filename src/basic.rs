use std::{fmt, ops, str};

#[derive(Clone, Copy)]
pub struct Resource([i8; 3]);

impl ops::Deref for Resource {
    type Target = [i8; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Resource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[i8; 3]> for Resource {
    fn from(value: [i8; 3]) -> Self {
        Self { 0: value }
    }
}

pub const INIT_STATE: Resource = Resource { 0: [0, 1, 1] };

#[derive(Clone, Copy)]
pub enum Action {
    Guahao,
    Attack(u8),
    Defend(u8),
    Quanfang,
    Fantan,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: &str = match &self {
            Action::Guahao => "挂号",
            Action::Attack(0) => "喂药",
            Action::Attack(1) => "打针",
            Action::Attack(2) => "开刀",
            Action::Attack(n) => &format!("攻击{}", n + 1),
            Action::Defend(0) => "捂嘴",
            Action::Defend(1) => "捂肩",
            Action::Defend(2) => "捂腹",
            Action::Defend(n) => &format!("防御{}", n + 1),
            Action::Quanfang => "全防",
            Action::Fantan => "反弹",
        };
        write!(f, "{}", &name)
    }
}

impl Action {
    pub fn get_cost(&self) -> Resource {
        match self {
            Action::Guahao => [-1, 0, 0],
            Action::Attack(n) => [(n + 1) as i8, 0, 0],
            Action::Defend(_) => [0, 0, 0],
            Action::Quanfang => [0, 1, 0],
            Action::Fantan => [0, 0, 1],
        }
        .into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RoundOutcome {
    Win,
    Lose,
    Continue,
}
