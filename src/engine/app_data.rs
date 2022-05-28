use std::time::Duration;
use chargrid::app::{ColModify, ControlFlow, Frame, Input, ViewContext};
use chargrid::input::KeyboardInput;
use chargrid::render::View;
use coord_2d::{Coord, Size};
use direction::CardinalDirection;
use rgb24::Rgb24;

pub struct AppData {
    pub screen_size: Size,
    pub player_coord: Coord,
}

impl AppData {
    pub fn new(screen_size: Size) -> Self {
        Self {
            screen_size,
            player_coord: screen_size.to_coord().unwrap() / 2,
        }
    }

    pub fn maybe_move_player(&mut self, direction: CardinalDirection) {
        let new_player_coord = self.player_coord + direction.coord();
        if new_player_coord.is_valid(self.screen_size) {
            self.player_coord = new_player_coord
        }
    }

    pub fn handle_input(&mut self, input: chargrid::input::Input) {
        match input {
            Input::Keyboard(key) => match key {
                KeyboardInput::Left => self.maybe_move_player(CardinalDirection::West),
                KeyboardInput::Right => self.maybe_move_player(CardinalDirection::East),
                KeyboardInput::Up => self.maybe_move_player(CardinalDirection::North),
                KeyboardInput::Down => self.maybe_move_player(CardinalDirection::South),
                _ => {}
            },
            _ => {}
        }
    }
}

