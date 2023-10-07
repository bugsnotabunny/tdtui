#[derive(Clone)]
pub enum DamageType {
    KINNETIC,
    EXPLOSIVE,
    MAGIC,
}

#[derive(Clone)]
pub struct Damage {
    pub value: u8,
    pub kind: DamageType,
}
