use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};


const LVL_SUFFIX: &str = "├──";
const LVL_SUFFIX_LAST: &str = "└──";
const PARENT_IS_NOT_LAST: &str = "│  ";
const PARENT_IS_LAST: &str = "   ";

pub(crate) type TreeItemRefCell = RefCell<TreeItem>;

pub(crate) struct TreeItem {
    pub(crate) text: String,
    pub(crate) is_dir: bool,
    pub(crate) is_last: bool,
    pub(crate) children: Vec<Rc<TreeItemRefCell>>,
    pub(crate) parent: Option<Weak<TreeItemRefCell>>,
}

impl TreeItem {
    pub(crate) fn new_top_level(text: String, is_dir: bool) -> Rc<TreeItemRefCell> {
        Rc::new(RefCell::new(Self {
            text,
            is_dir,
            is_last: true,
            children: Vec::new(),
            parent: None,
        }))
    }
    pub(crate) fn new(parent: &Rc<TreeItemRefCell>, text: String, is_dir: bool, is_last: bool) -> Rc<TreeItemRefCell> {
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
    pub(crate) fn to_row_str(&self, prefix_self: bool) -> String {
        let mut mut_symbols: Vec<String> = Vec::new();

        let prefix = if prefix_self {
            fill_symbols(&mut mut_symbols, self, false);
            mut_symbols.reverse();
            let symbols_str = mut_symbols.join("");
            format!("{} ", symbols_str)
        } else {
            String::new()
        };

        let mut rows: Vec<String> = Vec::new();
        rows.push(format!("{}{}", prefix, &self));

        for child in &self.children {
            rows.push(child.borrow().to_row_str(true));
        }
        rows.join("\n")
    }
}

fn fill_symbols(symbols: &mut Vec<String>, curr_item: &TreeItem, sent_from_child: bool) {
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

            if parent.parent.is_some() {
                fill_symbols(symbols, parent, true);
            }
        }
    }
}

impl Display for TreeItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = self.text.replace("\\", "/");
        let trail = if self.is_dir && !name.ends_with("/") { "/" } else { "" };
        let to_display = format!("{}{}", name, trail);
        write!(f, "{}", to_display)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_top_level() {
        let root = TreeItem::new_top_level("root".to_string(), true);
        let root_ref = root.borrow();
        assert_eq!(root_ref.text, "root");
        assert!(root_ref.is_dir);
        assert!(root_ref.is_last);
        assert!(root_ref.children.is_empty());
        assert!(root_ref.parent.is_none());
    }

    #[test]
    fn new_child() {
        let root = TreeItem::new_top_level("root".to_string(), true);
        let child = TreeItem::new(&root, "child".to_string(), false, true);

        let root_ref = root.borrow();
        assert_eq!(root_ref.children.len(), 1);

        let child_ref = child.borrow();
        assert_eq!(child_ref.text, "child");
        assert!(!child_ref.is_dir);
        assert!(child_ref.is_last);
        assert!(child_ref.children.is_empty());
        assert!(child_ref.parent.is_some());
    }

    #[test]
    fn to_row_str_single_item() {
        let root = TreeItem::new_top_level("root".to_string(), true);
        let result = root.borrow().to_row_str(false);
        assert_eq!(result, "root/");
    }

    #[test]
    fn to_row_str_with_children() {
        let root = TreeItem::new_top_level("root".to_string(), true);
        TreeItem::new(&root, "file1.txt".to_string(), false, false);
        TreeItem::new(&root, "file2.txt".to_string(), false, true);

        let result = root.borrow().to_row_str(false);
        let expected = "root/\n ├── file1.txt\n └── file2.txt";
        assert_eq!(result, expected);
    }

    #[test]
    fn to_row_str_nested_structure() {
        let root = TreeItem::new_top_level("root".to_string(), true);
        let folder = TreeItem::new(&root, "folder".to_string(), true, false);
        TreeItem::new(&folder, "file_in_folder.txt".to_string(), false, true);
        TreeItem::new(&root, "file_in_root.txt".to_string(), false, true);

        let result = root.borrow().to_row_str(false);
        let expected = "root/\n ├── folder/\n │   └── file_in_folder.txt\n └── file_in_root.txt";
        assert_eq!(result, expected);
    }

    #[test]
    fn display() {
        let item = TreeItem {
            text: "test".to_string(),
            is_dir: true,
            is_last: false,
            children: Vec::new(),
            parent: None,
        };
        assert_eq!(format!("{}", item), "test/");

        let file_item = TreeItem {
            text: "file.txt".to_string(),
            is_dir: false,
            is_last: true,
            children: Vec::new(),
            parent: None,
        };
        assert_eq!(format!("{}", file_item), "file.txt");
    }
}