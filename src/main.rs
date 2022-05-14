use std::error::Error;

use egui_tetra::StateWrapper;
use tetra::ContextBuilder;

use game_state::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub use crate::game_state::GameState;

mod algo;
mod game_state;
mod graph;
mod input;
mod camera_event;

fn main() -> Result<(), Box<dyn Error>> {
    ContextBuilder::new("Graph vis", SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .show_mouse(true)
        .quit_on_escape(true)
        .build()?
        .run(|ctx| Ok(StateWrapper::new(GameState::new(ctx)?)))
}
