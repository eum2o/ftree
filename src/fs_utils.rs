use crate::tree::{TreeItem, TreeItemRefCell};
use std::fs;
use std::rc::Rc;
use std::path::Path;

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
pub(crate) fn traverse_fs(path: &str, item: &Rc<TreeItemRefCell>, git: bool) {

    let git_ignore_path = Path::new(path).join(".gitignore");
    let ignore_matcher = if git {
        if git_ignore_path.exists() {
            Some(gitignore::File::new(&git_ignore_path).unwrap())
        } else {
            None
        }
    } else {
        None
    };

    match fs::read_dir(path) {
        Ok(dir) => {
            let dir_entries: Vec<_> = dir.collect::<Result<_, _>>().expect("Unable to read files");
            for dir_entry in dir_entries.into_iter() {
                let is_dir = dir_entry.metadata().expect("Unable to read metadata").is_dir();
                let file_name = dir_entry.file_name();
                let file_name_str = file_name.to_str().expect("Unable to read the file name");
                let full_path = Path::new(path).join(&file_name);

                // If git functionality is enabled, skip .git folder and check .gitignore
                if git {
                    // Skip .git folder
                    if file_name_str == ".git" {
                        continue;
                    }

                    // Check if the file is ignored by .gitignore
                    if let Some(ref matcher) = ignore_matcher {
                        if matcher.is_excluded(&full_path).unwrap() {
                            continue;
                        }
                    }
                }

                let child_node = TreeItem::new(item, file_name_str.to_string(), is_dir);
                
                // If it's a directory, recursively traverse it
                if is_dir {
                    let new_path = format!("{}/{}", path, file_name_str);
                    traverse_fs(&new_path, &child_node, git);
                }
            }
        }
        Err(err) => {
            panic!("Error reading files in {}: {}", path, err)
        }
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
        traverse_fs(temp_path.to_str().unwrap(), &root, false);

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

    #[test]
    fn test_traverse_fs_with_git() {
        // Prepare
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        fs::create_dir(temp_path.join("dir1")).unwrap();
        fs::create_dir(temp_path.join("dir2")).unwrap();
        fs::create_dir(temp_path.join(".git")).unwrap();
        fs::create_dir(temp_path.join("ignore_dir")).unwrap();
        File::create(temp_path.join("file1.txt")).unwrap().write_all(b"content").unwrap();
        File::create(temp_path.join(".gitignore")).unwrap().write_all(b"ignore_dir").unwrap();
        File::create(temp_path.join("dir1/file2.txt")).unwrap().write_all(b"content").unwrap();

        // Call
        let root = TreeItem::new_top_level(temp_path.to_str().unwrap().to_string(), true);
        traverse_fs(temp_path.to_str().unwrap(), &root, true);

        // Verify
        let root_ref = root.borrow();
        assert_eq!(root_ref.children.len(), 4);

        // Sort children by name for consistent ordering in tests
        let mut children: Vec<_> = root_ref.children.iter()
            .map(|c| Rc::clone(c))
            .collect();
        children.sort_by(|a, b| a.borrow().text.cmp(&b.borrow().text));

        // Check gitignore
        let gitignore = &children[0].borrow();
        assert_eq!(gitignore.text, ".gitignore");
        assert!(!gitignore.is_dir);
        assert_eq!(gitignore.children.len(), 0);

        // Check dir1
        let dir1 = &children[1].borrow();
        assert_eq!(dir1.text, "dir1");
        assert!(dir1.is_dir);
        assert_eq!(dir1.children.len(), 1);
        assert_eq!(dir1.children[0].borrow().text, "file2.txt");
        assert!(!dir1.children[0].borrow().is_dir);

        // Check dir2
        let dir2 = &children[2].borrow();
        assert_eq!(dir2.text, "dir2");
        assert!(dir2.is_dir);
        assert_eq!(dir2.children.len(), 0);

        // Check file1.txt
        let file1 = &children[3].borrow();
        assert_eq!(file1.text, "file1.txt");
        assert!(!file1.is_dir);
        assert_eq!(file1.children.len(), 0);
    }
}
