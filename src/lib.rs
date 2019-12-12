use std::env;
use std::{fs, path};

/// Obtains the path to the crate root, if the crate root cannot be found `None` is returned.
pub fn root() -> Option<path::PathBuf> {
    if let Ok(exec_path) = env::current_exe() {
        walk_up(&exec_path)
    } else {
        None
    }
}

/// Recursive helper function to obtain the path to the crate root.
fn walk_up(path: &path::Path) -> Option<path::PathBuf> {
    if let Ok(dir) = fs::read_dir(&path) {
        let mut found_target = false;
        let mut found_cargo_toml = false;

        for entry_result in dir {
            if let Ok(entry) = &entry_result {
                if !found_target && entry.file_name() == "target" {
                    found_target = true;
                }

                if !found_cargo_toml && entry.file_name() == "Cargo.toml" {
                    found_cargo_toml = true;
                }

                if found_target && found_cargo_toml {
                    return Some(path.to_path_buf());
                }
            }
        }
    }

    if let Some(parent) = path.parent() {
        walk_up(parent)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_crate() {
        assert_eq!(root().unwrap(), env::current_dir().unwrap());
    }
}
