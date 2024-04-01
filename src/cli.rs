use clap::Command;

pub const NAME: &str = "reference-table-preprocessor";

pub fn make_app() -> Command {
    Command::new(NAME).about("An mdbook preprocessor that resolves internal book link references")
}
