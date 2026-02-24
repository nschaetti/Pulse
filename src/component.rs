use crate::{Command, Frame, Rect};

pub trait Component {
    type Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg>;

    fn view(&self, frame: &mut Frame, area: Rect);
}

pub fn update_child<C, ParentMsg>(
    component: &mut C,
    msg: C::Msg,
    lift: impl FnMut(C::Msg) -> ParentMsg,
) -> Command<ParentMsg>
where
    C: Component,
{
    component.update(msg).map(lift)
}

#[cfg(test)]
mod tests {
    use crate::{Command, Frame, Rect};

    use super::{update_child, Component};

    #[derive(Clone, Copy)]
    enum ChildMsg {
        Ping,
        Stop,
    }

    enum ParentMsg {
        Child(ChildMsg),
    }

    struct Child;

    impl Component for Child {
        type Msg = ChildMsg;

        fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
            match msg {
                ChildMsg::Ping => Command::Emit(ChildMsg::Stop),
                ChildMsg::Stop => Command::Quit,
            }
        }

        fn view(&self, _frame: &mut Frame, _area: Rect) {}
    }

    #[test]
    fn update_child_lifts_emitted_message() {
        let mut child = Child;

        let command = update_child(&mut child, ChildMsg::Ping, ParentMsg::Child);

        assert!(matches!(
            command,
            Command::Emit(ParentMsg::Child(ChildMsg::Stop))
        ));
    }

    #[test]
    fn update_child_keeps_quit_command() {
        let mut child = Child;

        let command = update_child(&mut child, ChildMsg::Stop, ParentMsg::Child);

        assert!(matches!(command, Command::Quit));
    }
}
