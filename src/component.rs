use crate::{Command, Frame, Rect};

pub trait Component {
    type Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg>;

    fn view(&self, frame: &mut Frame, area: Rect);
}
