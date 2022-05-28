use std::time::Duration;
use chargrid::app::{ColModify, ControlFlow, Frame, Input, ViewContext};
use chargrid::render::View;
use coord_2d::{Coord, Size};
use rgb24::Rgb24;


pub struct App {
    data: AppData,
    view: AppView
}

impl App {
    pub fn new(screen_size: Size) -> Self {
        Self {
            data: AppData::new(screen_size),
            view: AppView::new()
        }
    }
}

impl chargrid::app::App for App {
    fn on_input(&mut self, input: Input) -> Option<ControlFlow> {
        use chargrid::input::{keys, Input};
        match input {
            Input::Keyboard(keys::ETX) | Input::Keyboard(keys::ESCAPE) => {
                Some(chargrid::app::ControlFlow::Exit)
            }
            _ => None,
        }
    }

    fn on_frame<F, C>(&mut self, since_last_frame: Duration, view_context: ViewContext<C>, frame: &mut F) -> Option<ControlFlow> where F: Frame, C: ColModify {
        use chargrid::render::View;
        self.view.view(&self.data, view_context, frame);
        None
    }
}

pub struct AppData {
    player_coord: Coord
}
impl AppData {
    pub fn new(screen_size: Size) -> Self {
        Self {
            player_coord: screen_size.to_coord().unwrap() / 2
        }
    }
}

pub struct AppView {}
impl AppView {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> View<&'a AppData> for AppView {
    fn view<F: Frame, C: ColModify>(&mut self, data: &'a AppData, context: ViewContext<C>, frame: &mut F) {
        let view_cell = chargrid::render::ViewCell::new()
            .with_character('@')
            .with_foreground(Rgb24::new_grey(255));
        frame.set_cell_relative(data.player_coord, 0, view_cell, context);
    }

    // fn size<C: ColModify>(&mut self, data: &'a AppData, context: ViewContext<C>) -> Size {
    //     todo!()
    // }
    //
    // fn view_size<F: Frame, C: ColModify>(&mut self, data: &'a AppData, context: ViewContext<C>, frame: &mut F) -> Size {
    //     todo!()
    // }
}