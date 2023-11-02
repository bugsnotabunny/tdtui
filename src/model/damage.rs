#[derive(Clone)]
pub enum DamageType {
    Kinnetic,
    Magic,
}

#[derive(Clone)]
pub struct Damage {
    pub value: u8,
    pub kind: DamageType,
}
