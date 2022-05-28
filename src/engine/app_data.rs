use std::time::Duration;
use chargrid::app::{ColModify, ControlFlow, Frame, Input, ViewContext};
use chargrid::render::View;
use coord_2d::{Coord, Size};
use rgb24::Rgb24;

pub struct AppData {
    pub player_coord: Coord
}

impl AppData {
    pub fn new(screen_size: Size) -> Self {
        Self {
            player_coord: screen_size.to_coord().unwrap() / 2
        }
    }
}

