pub enum Command<Msg> {
    None,
    Quit,
    Emit(Msg),
    Batch(Vec<Command<Msg>>),
}

impl<Msg> Command<Msg> {
    pub fn none() -> Self {
        Command::None
    }

    pub fn quit() -> Self {
        Command::Quit
    }

    pub fn emit(msg: Msg) -> Self {
        Command::Emit(msg)
    }

    pub fn batch(commands: impl IntoIterator<Item = Command<Msg>>) -> Self {
        Command::Batch(commands.into_iter().collect())
    }

    pub fn map<NextMsg>(self, mut f: impl FnMut(Msg) -> NextMsg) -> Command<NextMsg> {
        self.map_with(&mut f)
    }

    fn map_with<NextMsg, F>(self, f: &mut F) -> Command<NextMsg>
    where
        F: FnMut(Msg) -> NextMsg,
    {
        match self {
            Command::None => Command::None,
            Command::Quit => Command::Quit,
            Command::Emit(msg) => Command::Emit(f(msg)),
            Command::Batch(commands) => Command::Batch(
                commands
                    .into_iter()
                    .map(|command| command.map_with(f))
                    .collect(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn map_preserves_none() {
        let mapped: Command<u8> = Command::<u8>::none().map(|v| v + 1);
        assert!(matches!(mapped, Command::None));
    }

    #[test]
    fn map_preserves_quit() {
        let mapped: Command<u8> = Command::<u8>::quit().map(|v| v + 1);
        assert!(matches!(mapped, Command::Quit));
    }

    #[test]
    fn map_transforms_emit_payload() {
        let mapped = Command::emit(3_u8).map(|v| v + 1);
        assert!(matches!(mapped, Command::Emit(4)));
    }

    #[test]
    fn map_transforms_batch_payloads_in_order() {
        let mapped = Command::batch([
            Command::emit(1_u8),
            Command::none(),
            Command::batch([Command::emit(2_u8), Command::quit()]),
        ])
        .map(|v| v + 10);

        match mapped {
            Command::Batch(commands) => {
                assert!(matches!(commands[0], Command::Emit(11)));
                assert!(matches!(commands[1], Command::None));
                match &commands[2] {
                    Command::Batch(nested) => {
                        assert!(matches!(nested[0], Command::Emit(12)));
                        assert!(matches!(nested[1], Command::Quit));
                    }
                    _ => panic!("expected nested batch"),
                }
            }
            _ => panic!("expected batch"),
        }
    }

    #[test]
    fn helpers_construct_expected_variants() {
        assert!(matches!(Command::<u8>::none(), Command::None));
        assert!(matches!(Command::<u8>::quit(), Command::Quit));
        assert!(matches!(Command::emit(7_u8), Command::Emit(7)));
        assert!(matches!(
            Command::batch([Command::emit(1_u8), Command::emit(2_u8)]),
            Command::Batch(_)
        ));
    }
}
