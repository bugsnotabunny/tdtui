use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum DamageType {
    Kinnetic,
    Magic,
}

#[derive(Clone)]
pub struct Damage {
    pub value: f32,
    pub kind: DamageType,
}

impl Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = format!("{:?}", self);
        write!(f, "{}", text.chars().next().unwrap())
    }
}

impl Display for Damage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.kind)
    }
}
