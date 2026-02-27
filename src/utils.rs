use std::{env, path::PathBuf};

use is_executable::IsExecutable;

pub fn find_os_executable(name: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .map(|dir| dir.join(name))
            .find(|path| path.is_executable())
    })
}
