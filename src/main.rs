use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::{Deref, Index};
use std::rc::{Rc, Weak};

const EXEC_NAME: &str = "ftree";

const LVL_SUFFIX: &str = "├──";
const LVL_SUFFIX_LAST: &str = "└──";

const PARENT_IS_NOT_LAST: &str = "│  ";
const PARENT_IS_LAST: &str = "   ";

type TreeItemRefCell = RefCell<TreeItem>;

struct TreeItem {
    text: String,
    is_dir: bool,
    is_last: bool,
    children: Vec<Rc<TreeItemRefCell>>,
    parent: Option<Weak<TreeItemRefCell>>,
}

impl TreeItem {
    fn new_top_level(text: String, is_dir: bool) -> Rc<TreeItemRefCell> {
        Rc::new(RefCell::new(Self {
            text,
            is_dir,
            is_last: true,
            children: Vec::new(),
            parent: None,
        }))
    }
    fn new(parent: &Rc<TreeItemRefCell>, text: String, is_dir: bool, is_last: bool) -> Rc<TreeItemRefCell> {
        let inst = Self {
            text,
            is_dir,
            is_last,
            children: Vec::new(),
            parent: Some(Rc::downgrade(parent)),
        };

        let r_inst = Rc::new(RefCell::new(inst));
        parent.borrow_mut().children.push(Rc::clone(&r_inst));

        r_inst
    }


    ///
    /// Builds a string like:
    ///
    /// ```
    /// ./
    /// ├── top level folder/
    /// │   ├── code1.x
    /// │   ├── code2.x
    /// │   ├── nested folder 1/
    /// │   │   └── filewithoutext
    /// │   ├── nested folder empty/
    /// │   └── nested folder 2/
    /// │       ├── file1.txt
    /// │       └── file2.txt
    /// ├── readme.md
    /// └── meta.data
    /// ```
    ///
    fn to_row_str(&self, prefix_self: bool) {
        let mut mut_symbols: Vec<String> = Vec::new();

        let prefix = if prefix_self {
            to_row_str_rec(&mut mut_symbols, self, false);
            mut_symbols.reverse();
            let symbols_str = mut_symbols.join("");
            format!("{} ", symbols_str)
        } else {
            String::new()
        };

        println!("{}{}{}", prefix, self.text, if self.is_dir { "/" } else { "" });

        for child in &self.children {
            child.borrow().to_row_str(true);
        }
    }
}

fn to_row_str_rec(symbols: &mut Vec<String>, curr_item: &TreeItem, sent_from_child: bool) {
    let symbol = if sent_from_child {
        format!(" {}", if curr_item.is_last { PARENT_IS_LAST } else { PARENT_IS_NOT_LAST })
    } else {
        format!(" {}", if curr_item.is_last { LVL_SUFFIX_LAST } else { LVL_SUFFIX })
    };
    symbols.push(symbol.to_string());

    if let Some(parent_weak) = &curr_item.parent {
        if let Some(parent_strong) = parent_weak.upgrade() {
            let parent_ref = parent_strong.borrow();
            let parent = parent_ref.deref();

            if let Some(_) = &parent.parent {
                to_row_str_rec(symbols, parent, true);
            }
        }
    }
}

impl Display for TreeItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let txt = format!("{}{}", self.text, if self.is_dir { "/" } else { "" });
        writeln!(f, "{}", txt)
    }
}


fn main() {
    let path = read_path_from_args();
    let root = TreeItem::new_top_level(path.clone(), true);
    read_dir_rec(&path, &root);
    root.borrow().to_row_str(false);
}

fn read_dir_rec(path: &str, item: &Rc<TreeItemRefCell>) {
    match fs::read_dir(path) {
        Ok(dir) => {
            let dir_entries: Vec<_> = dir.collect::<Result<_, _>>().unwrap();
            let num_entries = dir_entries.len();
            for (i, dir_entry) in dir_entries.into_iter().enumerate() {
                let is_last = i == num_entries - 1;
                let is_dir = dir_entry.metadata().unwrap().is_dir();
                let text: String = dir_entry.file_name().to_str().unwrap().to_string();
                let child_node = TreeItem::new(item, text.clone(), is_dir, is_last);
                if is_dir {
                    let new_path = format!("{}/{}", &path, &text);
                    read_dir_rec(&new_path, &child_node);
                }
            }
        }
        Err(err) => { panic!("Error reading files in {}: {}", &path, err) }
    }
}


fn read_path_from_args() -> String {
    let args: Vec<String> = std::env::args().take(2).collect();
    if args.len() != 2 {
        panic!("Error: Invalid arguments.\nSyntax: {EXEC_NAME} <path>\nExample: {EXEC_NAME} .");
    }
    let path = &args[1];
    path.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        let exp = "./";

        // todo
        // let tree = Tree {
        //
        // let act = to_tree_str(&tree);
        //
        // assert_eq!(&act, &exp);
    }
}