use crate::tree::{TreeItem, TreeItemRefCell};
use std::fs;
use std::rc::Rc;

/// Recursively reads a directory and builds a tree structure.
///
/// This function traverses the directory specified by `path`, creating `TreeItem`
/// nodes for each file and subdirectory encountered. It populates the tree
/// structure starting from the given `item` node.
///
/// # Arguments
///
/// * `path` - The path to the directory to be read.
/// * `item` - The tree node to read the children for.
///
/// # Examples
///
/// ```
/// let root = TreeItem::new_top_level("/home/user", true);
/// read_dir_rec("/home/user", &root);
/// ```
pub(crate) fn traverse_fs(path: &str, item: &Rc<TreeItemRefCell>) {
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
                    traverse_fs(&new_path, &child_node);
                }
            }
        }
        Err(err) => { panic!("Error reading files in {}: {}", &path, err) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_traverse_fs() {
        // Prepare
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        fs::create_dir(temp_path.join("dir1")).unwrap();
        fs::create_dir(temp_path.join("dir2")).unwrap();
        File::create(temp_path.join("file1.txt")).unwrap().write_all(b"content").unwrap();
        File::create(temp_path.join("dir1/file2.txt")).unwrap().write_all(b"content").unwrap();

        // Call
        let root = TreeItem::new_top_level(temp_path.to_str().unwrap().to_string(), true);
        traverse_fs(temp_path.to_str().unwrap(), &root);

        // Verify
        let root_ref = root.borrow();
        assert_eq!(root_ref.children.len(), 3);

        // Sort children by name for consistent ordering in tests
        let mut children: Vec<_> = root_ref.children.iter()
            .map(|c| Rc::clone(c))
            .collect();
        children.sort_by(|a, b| a.borrow().text.cmp(&b.borrow().text));

        // Check dir1
        let dir1 = &children[0].borrow();
        assert_eq!(dir1.text, "dir1");
        assert!(dir1.is_dir);
        assert_eq!(dir1.children.len(), 1);
        assert_eq!(dir1.children[0].borrow().text, "file2.txt");
        assert!(!dir1.children[0].borrow().is_dir);

        // Check dir2
        let dir2 = &children[1].borrow();
        assert_eq!(dir2.text, "dir2");
        assert!(dir2.is_dir);
        assert_eq!(dir2.children.len(), 0);

        // Check file1.txt
        let file1 = &children[2].borrow();
        assert_eq!(file1.text, "file1.txt");
        assert!(!file1.is_dir);
        assert_eq!(file1.children.len(), 0);
    }
}