pub trait YmspTask {
    fn run(&self) -> color_eyre::Result<()>;
}
