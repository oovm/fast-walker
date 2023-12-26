use std::{
    fmt::{Formatter, Write},
    fs::DirEntry,
    path::{Path, PathBuf},
};

/// Convert a path to unix style path.
///
/// # Examples
///
/// ```
/// # use fast_walker::utils::to_unix_path;
/// let path = std::path::Path::new("C:\\P4Root\\test.txt");
/// ```
pub fn to_unix_path(path: &Path) -> String {
    #[cfg(target_os = "windows")]
    {
        path.to_string_lossy().trim_start_matches("\\\\?\\").replace("\\", "/")
    }
    #[cfg(not(target_os = "windows"))]
    {
        path.to_string_lossy().to_string()
    }
}

/// Get path and file name from a DirEntry.
pub fn path_info(entry: std::io::Result<DirEntry>) -> std::io::Result<(PathBuf, String)> {
    let entry = entry?;
    let name = match entry.file_name().to_str() {
        Some(s) => s.to_string(),
        None => Err(std::io::Error::new(std::io::ErrorKind::Other, "file name is not utf-8"))?,
    };
    Ok((entry.path().canonicalize()?, name))
}

#[allow(dead_code)]
pub(crate) fn write_unix_path(f: &mut Formatter<'_>, path: &Path) -> std::fmt::Result {
    for c in path.to_string_lossy().trim_start_matches("\\\\?\\").chars() {
        if c == '\\' { f.write_char('/')? } else { f.write_char(c)? }
    }
    Ok(())
}
