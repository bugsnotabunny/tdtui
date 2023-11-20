use std::time::Duration;

use crate::ui::point_drawable::PointDrawInfo;

use super::{
    core::{EnemyShared, GameModel, UpdatableObject},
    damage::Damage,
    point::{Point, Positioned},
    trajectory::Trajectory,
};

use rand::seq::IteratorRandom;

#[derive(Default, Debug, Clone)]
struct Aim {
    aim: Option<EnemyShared>,
}

impl Aim {
    pub fn new(aim: Option<EnemyShared>) -> Self {
        Self { aim: aim }
    }

    pub fn aim(&self) -> &Option<EnemyShared> {
        &self.aim
    }

    pub fn is_in_shoot_range(&self, tower: &Tower, trajectory: &dyn Trajectory) -> bool {
        match self.aim.as_ref() {
            Some(aimcell) => {
                let aim = aimcell;
                let enemypos = trajectory.point_from_t(aim.borrow().t_position());
                enemypos.distance(tower.position()) < tower.range()
            }
            None => false,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.aim.is_some() && !self.aim.as_ref().unwrap().borrow().is_dead()
    }

    pub fn try_damage(&mut self, damage: Damage, on_death: impl FnOnce(u64)) {
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

#[derive(Debug, Clone)]
pub struct TowerInfo {
    pub cooldown: Duration,
    pub cost: u64,
    pub range: f32,
    pub name: &'static str,
    pub description: &'static str,
    pub projectile_info: ProjectileInfo,
    pub draw_info: PointDrawInfo,
}

#[derive(Debug, Clone)]
pub struct Tower {
    aim: Aim,
    position: Point,
    cooldown_elapsed: Duration,
    type_info: &'static TowerInfo,
}

impl Tower {
    pub fn new(position: Point, type_info: &'static TowerInfo) -> Self {
        Self {
            aim: Aim::new(None),
            position: position,
            cooldown_elapsed: Duration::from_millis(0),
            type_info: type_info,
        }
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

impl Positioned for Tower {
    fn position(&self) -> Point {
        self.position
    }
}

impl UpdatableObject for Tower {
    fn on_update(&mut self, game_model: &mut impl GameModel, delta_time: Duration) {
        self.update_aim(game_model);
        self.cooldown_elapsed += delta_time;
        if self.cooldown_elapsed >= self.type_info.cooldown {
            self.maybe_shoot(game_model);
            self.cooldown_elapsed = Duration::from_millis(0);
        }
    }
}

impl Tower {
    fn maybe_shoot(&mut self, game_model: &mut impl GameModel) {
        if !self.aim.is_some() {
            return;
        }

        let projectile = Projectile::new(
            self.position,
            self.aim.aim().as_ref().unwrap().clone(),
            &self.type_info.projectile_info,
        );

        game_model.spawn_projectile(projectile);
    }

    fn update_aim(&mut self, game_model: &impl GameModel) {
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
                    .point_from_t(enemy.borrow().t_position());
                enemypos.distance(self.position()) < self.range()
            })
            .map(|rc| rc.clone())
            .choose(&mut rand::thread_rng());

        self.aim = Aim::new(random_chosen_enemy);
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ProjectileInfo {
    pub speed: f32,
    pub damage: Damage,
}

#[derive(Debug, Clone)]
pub struct Projectile {
    position: Point,
    aim: Aim,
    type_info: &'static ProjectileInfo,
}

impl Projectile {
    fn new(position: Point, aim: EnemyShared, type_info: &'static ProjectileInfo) -> Self {
        Self {
            position: position,
            aim: Aim::new(Some(aim)),
            type_info: type_info,
        }
    }

    pub fn type_info(&self) -> &'static ProjectileInfo {
        self.type_info
    }
}

impl Positioned for Projectile {
    fn position(&self) -> Point {
        self.position
    }
}

impl UpdatableObject for Projectile {
    fn on_update(&mut self, game_model: &mut impl GameModel, delta_time: Duration) {
        self.move_to_aim(game_model, delta_time);
    }
}

impl Projectile {
    pub fn is_active(&self) -> bool {
        self.aim.is_some()
    }

    pub fn move_to_aim(&mut self, game_model: &mut impl GameModel, delta_time: Duration) {
        if !self.is_active() {
            return;
        }
        let move_points = self.type_info.speed * delta_time.as_secs_f32();
        let trajectory = game_model.trajectory();
        let t = self.aim.aim().as_ref().unwrap().borrow().t_position();
        let aim_pos = trajectory.point_from_t(t);
        let direction = (aim_pos - self.position).normalize();
        let distance = self.position.distance(aim_pos);

        if distance < move_points {
            self.on_collision(game_model);
            return;
        }
        self.position = self.position + direction * move_points;
    }

    fn on_collision(&mut self, game_model: &mut impl GameModel) {
        self.aim.try_damage(self.type_info.damage, |reward| {
            game_model.wallet_mut().add_money(reward);
        });
        self.aim = Aim::new(None);
    }
}
