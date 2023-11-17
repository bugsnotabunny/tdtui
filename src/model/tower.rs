use std::time::Duration;

use super::{
    clock::Clock,
    core::{EnemyShared, GameModel, UpdatableObject},
    damage::Damage,
    point::Point,
    trajectory::Trajectory,
};

use rand::seq::IteratorRandom;

#[derive(Default)]
struct Aim {
    aim: Option<EnemyShared>,
}

impl Aim {
    pub fn new(aim: Option<EnemyShared>) -> Self {
        Self { aim: aim.clone() }
    }

    pub fn is_in_shoot_range(&self, tower: &Tower, trajectory: &dyn Trajectory) -> bool {
        match self.aim.as_ref() {
            Some(aimcell) => {
                let aim = aimcell;
                let enemypos = trajectory.get_point(aim.borrow().t_position());
                enemypos.distance(&tower.position()) < tower.range()
            }
            None => false,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.aim.is_some() && !self.aim.as_ref().unwrap().borrow().is_dead()
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

pub struct Tower {
    aim: Aim,
    position: Point,
    cooldown_clock: Clock,
    type_info: &'static TowerInfo,
}

impl Tower {
    pub fn new(position: Point, type_info: &'static TowerInfo) -> Self {
        Self {
            aim: Aim::new(None),
            position: position,
            cooldown_clock: Clock::from_now(),
            type_info: type_info,
        }
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn cost(&self) -> u64 {
        self.type_info.cost
    }

    pub fn range(&self) -> f32 {
        self.type_info.range
    }

    pub fn type_info(&self) -> &'static TowerInfo {
        &self.type_info
    }
}

impl UpdatableObject for Tower {
    fn on_update(&mut self, game_model: &mut dyn GameModel, _: Duration) {
        self.update_aim(game_model);
        if self.cooldown_clock.elapsed() > self.type_info.cooldown {
            self.shoot(game_model);
            self.cooldown_clock.tick();
        }
    }
}

impl Tower {
    fn shoot(&mut self, game_model: &mut dyn GameModel) {
        self.aim.try_shoot(self.type_info.damage.clone(), |reward| {
            game_model.wallet_mut().add_money(reward);
        });
    }

    fn update_aim(&mut self, game_model: &dyn GameModel) {
        if !self.aim.is_in_shoot_range(self, game_model.trajectory()) || !self.aim.is_alive() {
            self.aim = Aim::new(None);
        }

        if self.aim.is_some() {
            return;
        }

        let random_chosen_enemy = game_model
            .enemies()
            .iter()
            .filter(|enemy| {
                let enemypos = game_model
                    .trajectory()
                    .get_point(enemy.borrow().t_position());
                enemypos.distance(self.position()) < self.range()
            })
            .map(|rc| rc.clone())
            .choose(&mut rand::thread_rng());

        self.aim = Aim::new(random_chosen_enemy);
    }
}

pub struct TowerInfo {
    pub cooldown: Duration,
    pub cost: u64,
    pub range: f32,
    pub damage: Damage,
    pub close_up_sprite: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}
