pub enum Command<Msg> {
    None,
    Quit,
    Emit(Msg),
}
