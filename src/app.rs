use std::{error::Error, io, time::Duration};

use crate::{
    input::core::{poll_event, HandleEvent, InputEvent, ScreenInfo},
    model::{clock::Clock, core::GameModel},
    ui::core::{Camera, Screen},
};

#[derive(Debug, PartialEq, Eq)]
enum AppState {
    NotStarted,
    SettingUp,
    Paused,
    Running,
    Closing,
    Closed,
}

pub struct App<G: GameModel + HandleEvent> {
    game_model: G,
    screen: Screen,
    camera: Camera,
    update_clock: Clock,
    state: AppState,
}

impl<G: GameModel + HandleEvent> App<G> {
    pub fn new(model: G, ui: Screen) -> Self {
        Self {
            game_model: model,
            screen: ui,
            camera: Camera::default(),
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
                let screen_info =
                    ScreenInfo::from_frame_size(self.camera.clone(), self.screen.size()?);
                let event = poll_event(timeout, screen_info)?;
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
        self.screen.draw_frame(&self.camera, &self.game_model)?;
        self.update_clock.tick();
        Ok(())
    }
}

impl<G: GameModel + HandleEvent> HandleEvent for App<G> {
    fn handle(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        if self.state != AppState::Paused {
            self.camera.handle(event.clone())?;
            self.game_model.handle(event.clone())?;
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