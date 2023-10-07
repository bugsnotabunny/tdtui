use std::{cell::RefCell, rc::Rc};

use crate::{
    damage::{Damage, DamageType},
    enemy::Enemy,
};

pub struct Tower {
    damage: Damage,
    radius: u8,
    aim: Rc<RefCell<(u16, Enemy)>>,
}

thread_local! {
    static BILLY: RefCell<Rc<RefCell<(u16, Enemy)>>> = RefCell::new(Rc::new(RefCell::new((u16::MAX, Enemy::new(u8::MIN, u8::MIN)))));
}

impl Tower {
    pub fn new() -> Self {
        Self {
            damage: Damage {
                value: 10,
                kind: DamageType::KINNETIC,
            },
            radius: 8,
            aim: BILLY.with(|cell| cell.borrow_mut().clone()),
        }
    }

    pub fn aim(&mut self) -> Rc<RefCell<(u16, Enemy)>> {
        self.aim.clone()
    }

    pub fn set_aim(&mut self, enemy: Rc<RefCell<(u16, Enemy)>>) {
        self.aim = enemy
    }

    pub fn shoot(&mut self) {
        self.aim().borrow_mut().1.take_damage(self.damage.clone())
    }

    pub fn radius(&self) -> u8 {
        self.radius
    }
}
