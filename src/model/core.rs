use std::{cell::RefCell, rc::Rc, time::Duration};

use super::{
    enemy::Enemy,
    point::Point,
    spawner::Spawner,
    tower::Tower,
    tower_selector::TowerSelector,
    trajectory::Trajectory,
    wallet::{NotEnoughMoneyErr, Wallet},
};

pub type EnemyShared = Rc<RefCell<Enemy>>;
pub type EnemyUnique = Box<Enemy>;

pub trait GameModel {
    fn update(&mut self, delta_time: Duration);

    fn is_over(&self) -> bool;
    fn towers(&self) -> &Vec<Tower>;
    fn wallet(&self) -> &Wallet;
    fn selector(&self) -> &TowerSelector;
    fn trajectory(&self) -> &dyn Trajectory;
    fn enemies(&self) -> &Vec<EnemyShared>;

    fn selector_mut(&mut self) -> &mut TowerSelector;
    fn wallet_mut(&mut self) -> &mut Wallet;
}

pub trait UpdatableObject {
    fn on_update(&mut self, game_model: &mut dyn GameModel, delta_time: Duration);
}

pub struct ConcreteGameModel<S: Spawner, T: Trajectory> {
    trajectory: T,
    spawner: S,
    tower_selector: TowerSelector,
    towers: Vec<Tower>,
    enemies: Vec<EnemyShared>,
    player_wallet: Wallet,
}

impl<S: Spawner, T: Trajectory> ConcreteGameModel<S, T> {
    const ROAD_LEN: f32 = 100.0;

    pub fn new(spawner: S, trajectory: T, initial_balance: u64) -> Self {
        let mut wallet = Wallet::default();
        wallet.add_money(initial_balance);

        Self {
            towers: Vec::new(),
            enemies: Vec::new(),
            tower_selector: TowerSelector::default(),
            player_wallet: wallet,
            spawner: spawner,
            trajectory: trajectory,
        }
    }

    pub fn maybe_build_from_selector(&mut self, position: Point) -> Result<(), NotEnoughMoneyErr> {
        let tower = self.selector().produce_current(position);
        let cost = tower.cost();
        self.player_wallet.pay_to_do(cost, || {
            self.towers.push(tower);
        })
    }

    pub fn switch_selector(&mut self) {
        self.tower_selector.to_next();
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

        self.enemies.retain(|enemy| !enemy.borrow().is_dead());
        self.maybe_spawn_new_enemy();
    }

    fn selector(&self) -> &TowerSelector {
        &self.tower_selector
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

    fn wallet_mut(&mut self) -> &mut Wallet {
        &mut self.player_wallet
    }

    fn selector_mut(&mut self) -> &mut TowerSelector {
        &mut self.tower_selector
    }
}

impl<S: Spawner, T: Trajectory> ConcreteGameModel<S, T> {
    fn is_overrun(&self) -> bool {
        self.enemies
            .iter()
            .any(|enemy| enemy.borrow().t_position() > Self::ROAD_LEN)
    }

    fn maybe_spawn_new_enemy(&mut self) -> bool {
        match self.spawner.maybe_spawn() {
            Some(enemy) => {
                self.enemies.push(Rc::new(RefCell::new(*enemy)));
                true
            }
            None => false,
        }
    }
}
