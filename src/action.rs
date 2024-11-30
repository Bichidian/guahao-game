use std::{fmt, ops, str};

// pub type Resource = [i8; 3];

#[derive(Clone)]
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

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "挂号{}，全防{}，反弹{}", self[0], self[1], self[2])
    }
}

pub const INIT_STATE: Resource = Resource { 0: [0, 1, 1] };

#[derive(Clone)]
pub enum Action {
    Guahao,
    Attack(u8),
    Defend(u8),
    Quanfang,
    Fantan,
}

pub struct ParseActionError;

impl str::FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let act: Action = if let Ok(n) = s.parse::<i16>() {
            if n > 0 {
                Action::Attack(n as u8)
            } else if n < 0 {
                Action::Defend(-n as u8)
            } else {
                return Err(ParseActionError);
            }
        } else {
            match s {
                "g" => Action::Guahao,
                "q" => Action::Quanfang,
                "f" => Action::Fantan,
                _ => return Err(ParseActionError),
            }
        };
        Ok(act)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: &str = match &self {
            Action::Guahao => "挂号",
            Action::Attack(1) => "喂药",
            Action::Attack(2) => "打针",
            Action::Attack(3) => "开刀",
            Action::Attack(n) => &format!("攻击{n}"),
            Action::Defend(1) => "捂嘴",
            Action::Defend(2) => "捂肩",
            Action::Defend(3) => "捂腹",
            Action::Defend(n) => &format!("防御{n}"),
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
            Action::Attack(n) => [*n as i8, 0, 0],
            Action::Defend(_) => [0, 0, 0],
            Action::Quanfang => [0, 1, 0],
            Action::Fantan => [0, 0, 1],
        }
        .into()
    }
}
