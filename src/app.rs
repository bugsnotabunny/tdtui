use std::{error::Error, io, time::Duration};

use crate::{
    input::{poll_event, HandleEvent, InputContext, InputEvent, ScreenInfo},
    model::{Clock, GameModel},
    ui::{Camera, Screen},
};

#[derive(Debug, PartialEq, Eq)]
enum AppState {
    NotStarted,
    Paused,
    Running,
    Closing,
    Closed,
}

pub struct App<G: GameModel + HandleEvent> {
    game_model: G,
    input_context: InputContext,
    screen: Screen,
    camera: Camera,
    update_clock: Clock,
    state: AppState,
}

impl<G: GameModel + HandleEvent> App<G> {
    pub fn new(model: G, ui: Screen, camera: Camera) -> Self {
        Self {
            input_context: InputContext::new(model.min_tower_gap()),
            game_model: model,
            screen: ui,
            camera: camera,
            update_clock: Clock::from_now(),
            state: AppState::NotStarted,
        }
    }

    pub fn run(&mut self, tick_duration: Duration) -> io::Result<()> {
        self.state = AppState::Running;
        self.screen.init()?;
        let run_res = self.run_impl(tick_duration);
        self.screen.kill()?;
        run_res?;
        self.state = AppState::Closed;
        Ok(())
    }

    fn run_impl(&mut self, tick_duration: Duration) -> io::Result<()> {
        while self.state != AppState::Closing {
            while self.update_clock.elapsed() < tick_duration {
                let timeout = tick_duration.saturating_sub(self.update_clock.elapsed());

                let screen_info = ScreenInfo::from_frame_size(self.camera, self.screen.size()?);
                self.input_context.set_screen_info(screen_info);

                let event = poll_event(timeout)?;
                let _ = self.handle(event);
            }
            self.update()?;
            if self.game_model.is_over() {
                self.state = AppState::Closing
            }
        }
        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {
        if self.state != AppState::Paused {
            let delta_time = self.update_clock.elapsed();
            self.game_model.update(delta_time);
        }
        self.screen
            .draw_frame(&self.camera, &self.game_model, &self.input_context)?;
        self.update_clock.tick();
        Ok(())
    }

    fn handle(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        self.camera.handle(event, &self.input_context)?;
        self.input_context.handle(event)?;

        if self.state != AppState::Paused {
            self.game_model.handle(event, &self.input_context)?;
        }
        match event {
            InputEvent::GameQuit => self.state = AppState::Closing,
            InputEvent::GamePauseSwitch => {
                self.state = if self.state == AppState::Paused {
                    self.update_clock.tick();
                    AppState::Running
                } else {
                    AppState::Paused
                }
            }
            _ => {}
        }
        Ok(())
    }
}
