use std::time::Duration;

use super::{
    road::Road,
    tower::Tower,
    wallet::{NotEnoughMoneyErr, Wallet},
};

pub trait GameModel {
    fn update(&mut self, delta_time: Duration);
    fn is_over(&self) -> bool;
    fn road(&self) -> &dyn Road;
    fn towers(&self) -> &Vec<Box<dyn Tower>>;
    fn wallet(&self) -> &Wallet;
}

pub struct ConcreteGameModel<R: Road> {
    road: R,
    towers: Vec<Box<dyn Tower>>,
    player_wallet: Wallet,
}

impl<R: Road> ConcreteGameModel<R> {
    pub fn new(road: R) -> Self {
        Self {
            road: road,
            towers: Vec::new(),
            player_wallet: Wallet::default(),
        }
    }

    pub fn maybe_build(&mut self, tower: Box<dyn Tower>) -> Result<(), NotEnoughMoneyErr> {
        let cost = tower.cost();
        self.player_wallet.pay_to_do(cost, || {
            self.towers.push(tower);
        })
    }
}

impl<R: Road> GameModel for ConcreteGameModel<R> {
    fn update(&mut self, delta_time: Duration) {
        self.road.on_update(delta_time);
        for tower in self.towers.iter_mut() {
            tower.on_update(&self.road);
        }
    }

    fn wallet(&self) -> &Wallet {
        &self.player_wallet
    }

    fn is_over(&self) -> bool {
        self.road.is_overrun()
    }

    fn road(&self) -> &dyn Road {
        &self.road
    }

    fn towers(&self) -> &Vec<Box<dyn Tower>> {
        &self.towers
    }
}
