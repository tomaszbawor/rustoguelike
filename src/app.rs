use std::time::Duration;
use chargrid::app::{ColModify, ControlFlow, Frame, Input, ViewContext};


pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
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
       None
    }
}
