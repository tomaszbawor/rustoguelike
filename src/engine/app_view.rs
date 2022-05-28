use chargrid::app::{ColModify, Frame, ViewContext};
use chargrid::render::View;
use rgb24::Rgb24;
use crate::engine::AppData;

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