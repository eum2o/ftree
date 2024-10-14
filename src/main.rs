mod fs_utils;
mod tree;
use std::path::PathBuf;
use crate::tree::TreeItem;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Enable git functionality
    #[arg(long)]
    git: bool,

    /// The directory path to operate on
    #[arg(value_name = "DIRECTORY")]
    directory: PathBuf,
}

fn main() {
    let args = Args::parse();

    let path = args.directory;
    let root = TreeItem::new_top_level(path.to_str().unwrap().to_string(), true);

    // If --git is passed, use gitignore
    fs_utils::traverse_fs(path.to_str().unwrap(), &root, args.git);

    println!("{}", root.borrow().to_row_str(false));

}
