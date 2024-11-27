use std::{fmt, str};

pub type Resource = [i8; 3];

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
            Action::Attack(n) => &format!("攻击{n}"),
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
    }
}
