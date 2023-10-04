use std::env::{split_paths, var_os};
use std::path::{Path, PathBuf};

pub fn which<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    var_os("PATH").and_then(|paths| {
        split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}
