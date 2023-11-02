use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::ui::core::Drawable;

use super::{
    clock::Clock,
    core::{GameModel, UpdatableObject},
    damage::{Damage, DamageType},
    enemy::Enemy,
    point::Point,
    trajectory::Trajectory,
};

use rand::seq::IteratorRandom;

#[derive(Default)]
struct Aim {
    aim: Option<Rc<RefCell<dyn Enemy>>>,
}

impl Aim {
    pub fn new(aim: Option<Rc<RefCell<dyn Enemy>>>) -> Self {
        Self { aim: aim.clone() }
    }

    pub fn is_in_shoot_range(&self, tower: &impl Tower, trajectory: &dyn Trajectory) -> bool {
        match self.aim.as_ref() {
            Some(aimcell) => {
                let aim = aimcell.borrow();
                let enemypos = trajectory.get_point(aim.position());
                enemypos.distance(&tower.position()) < tower.range()
            }
            None => false,
        }
    }

    pub fn try_shoot(&mut self, damage: Damage, on_death: impl FnOnce(u64)) {
        if !self.is_some() {
            return;
        }
        let aim = self.aim.as_ref().unwrap();
        aim.borrow_mut().take_damage(damage);
        if aim.borrow().is_dead() {
            on_death(aim.borrow().reward());
            self.aim = None;
        }
    }

    pub fn is_some(&self) -> bool {
        self.aim.is_some()
    }
}

macro_rules! create_tower_type {
    ($a:ident ) => {
        pub struct $a {
            aim: Aim,
            position: Point,
            cooldown_clock: Clock,
        }

        impl $a {
            pub fn new(position: Point) -> Self {
                Self {
                    aim: Aim::new(None),
                    position: position,
                    cooldown_clock: Clock::from_now(),
                }
            }

            fn shoot(&mut self, game_model: &mut dyn GameModel) {
                self.aim.try_shoot(Self::DAMAGE, |reward| {
                    game_model.wallet_mut().add_money(reward);
                });
            }

            fn update_aim(&mut self, game_model: &dyn GameModel) {
                if !self.aim.is_in_shoot_range(self, game_model.trajectory()) {
                    self.aim = Aim::new(None);
                }

                if self.aim.is_some() {
                    return;
                }

                let random_chosen_enemy = game_model
                    .enemies()
                    .iter()
                    .filter(|enemy| {
                        let enemypos = game_model.trajectory().get_point(enemy.borrow().position());
                        enemypos.distance(self.position()) < self.range()
                    })
                    .map(|rc| rc.clone())
                    .choose(&mut rand::thread_rng());

                self.aim = Aim::new(random_chosen_enemy);
            }
        }

        impl Tower for $a {
            fn position(&self) -> &Point {
                &self.position
            }

            fn cost(&self) -> u64 {
                Self::COST
            }

            fn range(&self) -> f32 {
                Self::RANGE
            }
        }

        impl UpdatableObject for $a {
            fn on_update(&mut self, game_model: &mut dyn GameModel, _: Duration) {
                self.update_aim(game_model);

                if self.cooldown_clock.elapsed() > Self::COOLDOWN {
                    self.shoot(game_model);
                    self.cooldown_clock.tick();
                }
            }
        }
    };
}

pub trait Tower: UpdatableObject + Drawable {
    fn position(&self) -> &Point;
    fn range(&self) -> f32;
    fn cost(&self) -> u64;
}

pub trait TowerStats {
    const COOLDOWN: Duration;
    const COST: u64;
    const RANGE: f32;
    const DAMAGE: Damage;
}

create_tower_type!(ArcherTower);
impl TowerStats for ArcherTower {
    const COOLDOWN: Duration = Duration::from_millis(1500);
    const COST: u64 = 10;
    const RANGE: f32 = 50.0;
    const DAMAGE: Damage = Damage {
        value: 10,
        kind: DamageType::Kinnetic,
    };
}

create_tower_type!(MageTower);
impl TowerStats for MageTower {
    const COOLDOWN: Duration = Duration::from_millis(2000);
    const COST: u64 = 20;
    const RANGE: f32 = 100.0;
    const DAMAGE: Damage = Damage {
        value: 5,
        kind: DamageType::Magic,
    };
}
