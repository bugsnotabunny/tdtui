use std::{cell::RefCell, error::Error, fmt::Display, rc::Rc, time::Duration};

use super::{
    enemy::Enemy,
    point::Positioned,
    spawner::Spawner,
    tower::{Projectile, Tower},
    trajectory::Trajectory,
    wallet::Wallet,
};

pub type EnemyShared = Rc<RefCell<Enemy>>;
pub type EnemyUnique = Box<Enemy>;

pub trait GameModel {
    fn update(&mut self, delta_time: Duration);

    fn is_over(&self) -> bool;
    fn trajectory(&self) -> &dyn Trajectory;

    fn enemies(&self) -> &Vec<EnemyShared>;
    fn towers(&self) -> &Vec<Tower>;
    fn projectiles(&self) -> &Vec<Projectile>;

    fn spawn_projectile(&mut self, projectile: Projectile);
    fn spawn_tower(&mut self, tower: Tower) -> Result<(), Box<dyn Error>>;
    fn spawn_enemy(&mut self, enemy: Enemy);

    fn wallet(&self) -> &Wallet;
    fn wallet_mut(&mut self) -> &mut Wallet;
}

pub trait UpdatableObject {
    fn on_update(&mut self, game_model: &mut impl GameModel, delta_time: Duration);
}

pub struct ConcreteGameModel<S: Spawner, T: Trajectory> {
    min_tower_gap: f32,
    trajectory: T,
    spawner: S,
    towers: Vec<Tower>,
    enemies: Vec<EnemyShared>,
    projectiles: Vec<Projectile>,
    player_wallet: Wallet,
}

impl<S: Spawner, T: Trajectory> ConcreteGameModel<S, T> {
    const ROAD_LEN: f32 = 100.0;

    pub fn new(spawner: S, trajectory: T, initial_balance: u64, min_tower_gap: f32) -> Self {
        let mut wallet = Wallet::default();
        wallet.add_money(initial_balance);

        Self {
            min_tower_gap,
            towers: Vec::new(),
            enemies: Vec::new(),
            projectiles: Vec::new(),
            player_wallet: wallet,
            spawner: spawner,
            trajectory: trajectory,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TooTightTowerPlacementErr {}

impl Display for TooTightTowerPlacementErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tried to build tower too close to other tower")
    }
}

impl Error for TooTightTowerPlacementErr {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl<S: Spawner, T: Trajectory> GameModel for ConcreteGameModel<S, T> {
    fn update(&mut self, delta_time: Duration) {
        let enemies = std::mem::take(&mut self.enemies);
        for enemy in enemies.iter() {
            enemy.borrow_mut().on_update(self, delta_time);
        }
        self.enemies = enemies;

        let mut towers = std::mem::take(&mut self.towers);
        for tower in towers.iter_mut() {
            tower.on_update(self, delta_time);
        }
        self.towers = towers;

        let mut projectiles = std::mem::take(&mut self.projectiles);
        for projectile in projectiles.iter_mut() {
            projectile.on_update(self, delta_time);
        }
        self.projectiles = projectiles;

        self.projectiles.retain(|projectile| projectile.is_active());
        self.enemies.retain(|enemy| !enemy.borrow().is_dead());

        let mut spawner = std::mem::take(&mut self.spawner);
        spawner.on_update(self, delta_time);
        self.spawner = spawner;
    }

    fn wallet(&self) -> &Wallet {
        &self.player_wallet
    }

    fn is_over(&self) -> bool {
        self.is_overrun()
    }

    fn towers(&self) -> &Vec<Tower> {
        &self.towers
    }

    fn trajectory(&self) -> &dyn Trajectory {
        &self.trajectory
    }

    fn enemies(&self) -> &Vec<EnemyShared> {
        &self.enemies
    }

    fn projectiles(&self) -> &Vec<Projectile> {
        &self.projectiles
    }

    fn wallet_mut(&mut self) -> &mut Wallet {
        &mut self.player_wallet
    }

    fn spawn_projectile(&mut self, projectile: Projectile) {
        self.projectiles.push(projectile)
    }

    fn spawn_tower(&mut self, new_tower: Tower) -> Result<(), Box<dyn Error>> {
        if self
            .towers
            .iter()
            .any(|tower| tower.position().distance(new_tower.position()) < self.min_tower_gap)
        {
            return Err(Box::new(TooTightTowerPlacementErr {}));
        }

        self.player_wallet
            .pay_to_do(new_tower.cost(), || self.towers.push(new_tower))?;
        Ok(())
    }

    fn spawn_enemy(&mut self, enemy: Enemy) {
        self.enemies.push(Rc::new(RefCell::new(enemy)))
    }
}

impl<S: Spawner, T: Trajectory> ConcreteGameModel<S, T> {
    fn is_overrun(&self) -> bool {
        self.enemies
            .iter()
            .any(|enemy| enemy.borrow().t_position() > Self::ROAD_LEN)
    }
}
