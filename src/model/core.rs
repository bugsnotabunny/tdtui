use std::{cell::RefCell, rc::Rc, time::Duration};

use super::{
    enemy::Enemy,
    spawner::Spawner,
    tower::Tower,
    trajectory::Trajectory,
    wallet::{NotEnoughMoneyErr, Wallet},
};

pub trait GameModel {
    fn update(&mut self, delta_time: Duration);

    fn is_over(&self) -> bool;
    fn towers(&self) -> &Vec<Box<RefCell<dyn Tower>>>;
    fn wallet(&self) -> &Wallet;
    fn trajectory(&self) -> &dyn Trajectory;
    fn enemies(&self) -> &Vec<Rc<RefCell<Enemy>>>;
}

pub trait UpdatableObject {
    fn on_update(&mut self, game_model: &dyn GameModel, delta_time: Duration);
}

pub struct ConcreteGameModel<S: Spawner, T: Trajectory> {
    trajectory: T,
    spawner: S,
    towers: Vec<Box<RefCell<dyn Tower>>>,
    enemies: Vec<Rc<RefCell<Enemy>>>,
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
            player_wallet: wallet,
            spawner: spawner,
            trajectory: trajectory,
        }
    }

    pub fn maybe_build(&mut self, tower: Box<RefCell<dyn Tower>>) -> Result<(), NotEnoughMoneyErr> {
        let cost = tower.borrow().cost();
        self.player_wallet.pay_to_do(cost, || {
            self.towers.push(tower);
        })
    }
}

impl<S: Spawner, T: Trajectory> GameModel for ConcreteGameModel<S, T> {
    fn update(&mut self, delta_time: Duration) {
        for tower in self.towers.iter() {
            tower.borrow_mut().on_update(self, delta_time);
        }

        self.enemies.retain(|enemy| !enemy.borrow().is_dead());
        for enemy in self.enemies.iter() {
            enemy.borrow_mut().on_update(self, delta_time);
        }

        self.maybe_spawn_new_enemy();
    }

    fn wallet(&self) -> &Wallet {
        &self.player_wallet
    }

    fn is_over(&self) -> bool {
        self.is_overrun()
    }

    fn towers(&self) -> &Vec<Box<RefCell<dyn Tower>>> {
        &self.towers
    }

    fn trajectory(&self) -> &dyn Trajectory {
        &self.trajectory
    }

    fn enemies(&self) -> &Vec<Rc<RefCell<Enemy>>> {
        &self.enemies
    }
}

impl<S: Spawner, T: Trajectory> ConcreteGameModel<S, T> {
    fn is_overrun(&self) -> bool {
        self.enemies
            .iter()
            .any(|rc| rc.borrow().position() > Self::ROAD_LEN)
    }

    fn maybe_spawn_new_enemy(&mut self) -> bool {
        match self.spawner.maybe_spawn() {
            Some(enemy) => {
                let enemy = Rc::new(RefCell::new(enemy));
                self.enemies.push(enemy);
                true
            }
            None => false,
        }
    }
}
