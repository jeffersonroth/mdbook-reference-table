mod cli;
mod reference_table;

use env_logger;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use reference_table::ReferenceTable;
use std::{io, process};

fn main() {
    env_logger::init();

    if std::env::args().nth(1).as_deref() == Some("supports") {
        process::exit(0);
    }

    let _app = cli::make_app();

    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin()).expect("Failed to parse input");

    let preprocessor = ReferenceTable::new(&ctx);

    let processed_book = preprocessor
        .run(&ctx, book)
        .expect("Failed to process book");

    serde_json::to_writer(io::stdout(), &processed_book).expect("Failed to emit processed book");
}
