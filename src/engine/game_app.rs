use std::time::Duration;
use chargrid::app::{ColModify, ControlFlow, Frame, Input, ViewContext};
use coord_2d::Size;
use crate::engine::{AppData, AppView};

pub struct App {
    data: AppData,
    view: AppView,
}

impl App {
    pub fn new(screen_size: Size) -> Self {
        Self {
            data: AppData::new(screen_size),
            view: AppView::new(),
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

