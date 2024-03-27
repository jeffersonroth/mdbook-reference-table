use clap::Command;
use lazy_static::lazy_static;
use log::{error, info};
use mdbook::{
    book::{Book, BookItem},
    errors::Error as MdBookError,
    preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext},
};
use regex::Regex;
use std::{
    collections::HashMap,
    fs, io,
    path::{Component, Path, PathBuf},
    process,
};

const NAME: &str = "reference-table-preprocessor";

lazy_static! {
    static ref REFERENCE_RE: Regex = Regex::new(
        r##"\{\{reference:\s*\{id:\s*"(?P<id>[^"]+)"\s*,\s*title:\s*"(?P<title>[^"]+)"\}\}\}"##,
    )
    .unwrap();
}

pub fn make_app() -> Command {
    Command::new(NAME).about("An mdbook preprocessor that resolves internal book link references")
}

fn calculate_relative_path(src_file: &Path, target_file: &Path) -> String {
    info!("Source file: {:?}", src_file);
    info!("Target file: {:?}", target_file);

    // Find the common ancestor between the paths
    let mut common_components = 0;
    for (src_component, target_component) in src_file.components().zip(target_file.components()) {
        info!(
            "Src component: {:?}, Target component: {:?}",
            src_component, target_component
        );
        if src_component != target_component {
            break;
        }
        common_components += 1;
    }

    info!("Common components: {:?}", common_components);

    // Construct the relative path
    let mut relative_path = String::new();
    for _ in src_file.components().skip(common_components) {
        relative_path.push_str("../");
    }
    for component in target_file.components().skip(common_components) {
        if let Component::Normal(name) = component {
            relative_path.push_str(&name.to_string_lossy());
            relative_path.push('/');
        }
    }

    info!("Relative path: {:?}", relative_path);

    // Handle the case where source and target paths are the same
    if relative_path.is_empty() {
        relative_path.push_str("./");
    }

    relative_path.trim_end_matches('/').to_string()
}

fn load_references(reference_table_path: &str, src_base_path: &Path) -> HashMap<String, String> {
    let yaml_path = src_base_path.join(reference_table_path);
    let yaml_content =
        fs::read_to_string(&yaml_path).expect("Failed to read reference table YAML file");

    let yaml_data: serde_yaml::Value =
        serde_yaml::from_str(&yaml_content).expect("Failed to parse reference table YAML content");

    let mut references = HashMap::new();
    if let Some(reference_table) = yaml_data["reference-table"].as_sequence() {
        for entry in reference_table {
            if let (Some(id), Some(path)) = (entry["id"].as_str(), entry["path"].as_str()) {
                // Construct the target path without canonicalizing
                let target_path = src_base_path.join(path);

                let relative_path = calculate_relative_path(src_base_path, &target_path);

                references.insert(id.to_string(), relative_path);
            }
        }
    }
    references
}

struct ReferenceTable {
    references: HashMap<String, String>, // Maps IDs to paths
}

impl ReferenceTable {
    fn new(ctx: &PreprocessorContext) -> Self {
        let reference_table_path = ctx
            .config
            .get("preprocessor.reference-table.reference-table")
            .and_then(|c| c.as_str())
            .unwrap_or_default();

        let references = load_references(reference_table_path, &ctx.root);
        Self { references }
    }

    pub fn lookup_path(&self, id: &str) -> Option<&String> {
        self.references.get(id)
    }
}

impl Preprocessor for ReferenceTable {
    fn name(&self) -> &str {
        NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, MdBookError> {
        let mut errors: Vec<String> = Vec::new();

        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(chap) = item {
                match chap.path.as_deref() {
                    Some(path) => {
                        let src_file_path = PathBuf::from(ctx.root.join(path).as_path());
                        chap.content = REFERENCE_RE
                            .replace_all(&chap.content, |caps: &regex::Captures| {
                                let id = caps.name("id").unwrap().as_str();
                                let title = caps.name("title").unwrap().as_str();

                                if let Some(target_path) = self.lookup_path(id) {
                                    let relative_path = calculate_relative_path(
                                        &src_file_path,
                                        PathBuf::from(target_path).as_path(),
                                    );
                                    format!("[{}]({})", title, relative_path)
                                } else {
                                    error!("Reference {} not found", id);
                                    format!("{} (reference not found)", title)
                                }
                            })
                            .to_string();
                    }
                    None => {
                        let error_msg =
                            format!("Chapter {:?} has no path, which is unexpected.", chap.name);
                        errors.push(error_msg.clone());
                        error!("{}", error_msg);
                    }
                }
            }
        });

        if errors.is_empty() {
            Ok(book)
        } else {
            Err(MdBookError::from(anyhow::Error::msg(errors.join("\n"))))
        }
    }
}

fn main() {
    env_logger::init();

    if std::env::args().nth(1).as_deref() == Some("supports") {
        process::exit(0);
    }

    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin()).expect("Failed to parse input");

    let preprocessor = ReferenceTable::new(&ctx);

    let processed_book = preprocessor
        .run(&ctx, book)
        .expect("Failed to process book");

    serde_json::to_writer(io::stdout(), &processed_book).expect("Failed to emit processed book");
}
