mod fs_utils;
mod tree;
use crate::tree::TreeItem;

const EXEC_NAME: &str = "ftree";

fn main() {
    let path = read_path_from_args();
    let root = TreeItem::new_top_level(path.clone(), true);
    fs_utils::traverse_fs(&path, &root);
    println!("{}", root.borrow().to_row_str(false));
}

fn read_path_from_args() -> String {
    let args: Vec<String> = std::env::args().take(2).collect();
    if args.len() != 2 {
        panic!("Error: Invalid arguments.\nSyntax: {EXEC_NAME} <path>\nExample: {EXEC_NAME} .");
    }
    let path = &args[1];
    path.to_string()
}

