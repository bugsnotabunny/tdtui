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

pub trait Tower: UpdatableObject + Drawable {
    fn position(&self) -> &Point;
    fn range(&self) -> f32;
    fn cost(&self) -> u64;
}

pub struct ArcherTower {
    aim: Aim,
    position: Point,
    cooldown_clock: Clock,
}

impl ArcherTower {
    const COOLDOWN: Duration = Duration::from_millis(1500);
    const COST: u64 = 10;
    const RANGE: f32 = 50.0;
    const DAMAGE: Damage = Damage {
        value: 10,
        kind: DamageType::Kinnetic,
    };

    pub fn new(position: Point) -> Self {
        Self {
            aim: Aim::new(None),
            position: position,
            cooldown_clock: Clock::from_now(),
        }
    }

    fn shoot(&mut self, game_model: &mut dyn GameModel) {
        shoot_impl(&mut self.aim, Self::DAMAGE, game_model);
    }

    fn update_aim(&mut self, game_model: &dyn GameModel) {
        let mut aim = std::mem::take(&mut self.aim);
        update_aim_impl(&mut aim, self, game_model);
        self.aim = aim;
    }
}

impl Tower for ArcherTower {
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

impl UpdatableObject for ArcherTower {
    fn on_update(&mut self, game_model: &mut dyn GameModel, _: Duration) {
        self.update_aim(game_model);
        if self.cooldown_clock.elapsed() > Self::COOLDOWN {
            self.shoot(game_model);
            self.cooldown_clock.tick();
        }
    }
}

pub struct MageTower {
    aim: Aim,
    position: Point,
    cooldown_clock: Clock,
}

impl MageTower {
    const COOLDOWN: Duration = Duration::from_millis(2000);
    const COST: u64 = 20;
    const RANGE: f32 = 100.0;
    const DAMAGE: Damage = Damage {
        value: 5,
        kind: DamageType::Magic,
    };

    pub fn new(position: Point) -> Self {
        Self {
            aim: Aim::new(None),
            position: position,
            cooldown_clock: Clock::from_now(),
        }
    }

    fn shoot(&mut self, game_model: &mut dyn GameModel) {
        shoot_impl(&mut self.aim, Self::DAMAGE, game_model);
    }

    fn update_aim(&mut self, game_model: &dyn GameModel) {
        let mut aim = std::mem::take(&mut self.aim);
        update_aim_impl(&mut aim, self, game_model);
        self.aim = aim;
    }
}

impl Tower for MageTower {
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

impl UpdatableObject for MageTower {
    fn on_update(&mut self, game_model: &mut dyn GameModel, _: Duration) {
        self.update_aim(game_model);
        if self.cooldown_clock.elapsed() > Self::COOLDOWN {
            self.shoot(game_model);
            self.cooldown_clock.tick();
        }
    }
}

fn shoot_impl(aim: &mut Aim, damage: Damage, game_model: &mut dyn GameModel) {
    aim.try_shoot(damage, |reward| {
        game_model.wallet_mut().add_money(reward);
    });
}

fn update_aim_impl(aim: &mut Aim, tower: &impl Tower, game_model: &dyn GameModel) {
    if !aim.is_in_shoot_range(tower, game_model.trajectory()) {
        *aim = Aim::new(None);
    }

    if aim.is_some() {
        return;
    }

    let random_chosen_enemy = game_model
        .enemies()
        .iter()
        .filter(|enemy| {
            let enemypos = game_model.trajectory().get_point(enemy.borrow().position());
            enemypos.distance(&tower.position()) < tower.range()
        })
        .map(|rc| rc.clone())
        .choose(&mut rand::thread_rng());

    *aim = Aim::new(random_chosen_enemy);
}
