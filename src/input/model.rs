use crate::model::{
    core::ConcreteGameModel,
    damage::{Damage, DamageType},
    road::Road,
    tower::BasicTower,
};

use super::core::{HandleEvent, InputEvent};

impl<R: Road> HandleEvent for ConcreteGameModel<R> {
    fn handle(&mut self, event: InputEvent) {
        match event {
            InputEvent::MousePressedL(input) => {
                self.build(Box::new(BasicTower::new(
                    Damage {
                        value: 1,
                        kind: DamageType::KINNETIC,
                    },
                    100.0,
                    input.to_world_point(),
                )));
            }
            _ => {}
        }
    }
}
