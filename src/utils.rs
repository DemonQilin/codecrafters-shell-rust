use std::{env, path::PathBuf};

use is_executable::IsExecutable;

pub fn find_os_executable(str: &str) -> Option<PathBuf> {
    env::var_os("PATH")
        .and_then(|paths| env::split_paths(&paths).find(|dir| dir.join(str).is_executable()))
        .map(|dir| dir.join(str))
}
