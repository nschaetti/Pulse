use crate::{Command, Frame};

pub trait App {
    type Msg;

    fn init(&mut self) {}

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg>;

    fn view(&self, frame: &mut Frame);
}
